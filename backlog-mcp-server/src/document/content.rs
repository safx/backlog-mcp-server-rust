//! Pure text helpers for paging and editing a document's Markdown body.

use std::borrow::Cow;

use regex::Regex;

/// Upper bound (bytes) of the text block returned by one page, including line numbers.
pub(crate) const MAX_PAGE_BYTES: usize = 50_000;
pub(crate) const DEFAULT_LIMIT: u32 = 200;
pub(crate) const MAX_LIMIT: u32 = 2000;

/// `str::lines()` drops `\r`, but replacements run on the raw body; normalize once so
/// anchors copied from a page always match.
pub(crate) fn normalize_newlines(text: &str) -> Cow<'_, str> {
    if text.contains("\r\n") {
        Cow::Owned(text.replace("\r\n", "\n"))
    } else {
        Cow::Borrowed(text)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Page {
    /// Lines formatted as `N<TAB>text\n`, never longer than `MAX_PAGE_BYTES`.
    pub text: String,
    pub returned_lines: u32,
    /// Set only when the scan stopped before the end of the document.
    pub next_line: Option<u32>,
    /// Line number of a single line that exceeded `MAX_PAGE_BYTES` and was cut.
    pub truncated_line: Option<u32>,
}

/// Returns up to `limit` lines starting at 1-based `start_line`; with `search`, only lines
/// matching the regex. `start_line` and `limit` are clamped to valid ranges.
pub(crate) fn page(plain: &str, start_line: u32, limit: u32, search: Option<&Regex>) -> Page {
    let start_line = start_line.max(1) as usize;
    let limit = limit.clamp(1, MAX_LIMIT);
    let total = plain.lines().count();
    let mut page = Page::default();

    for (index, line) in plain.lines().enumerate().skip(start_line - 1) {
        let number = index + 1;
        if search.is_some_and(|re| !re.is_match(line)) {
            continue;
        }
        let prefix = format!("{number}\t");
        if page.text.len() + prefix.len() + line.len() + 1 > MAX_PAGE_BYTES {
            if page.returned_lines == 0 {
                let budget = MAX_PAGE_BYTES - page.text.len() - prefix.len() - 1;
                let cut = line.floor_char_boundary(budget.min(line.len()));
                page.text.push_str(&prefix);
                page.text.push_str(&line[..cut]);
                page.text.push('\n');
                page.returned_lines = 1;
                page.truncated_line = Some(number as u32);
                page.next_line = (number < total).then_some(number as u32 + 1);
            } else {
                page.next_line = Some(number as u32);
            }
            return page;
        }
        page.text.push_str(&prefix);
        page.text.push_str(line);
        page.text.push('\n');
        page.returned_lines += 1;
        if page.returned_lines == limit {
            page.next_line = (number < total).then_some(number as u32 + 1);
            return page;
        }
    }
    page
}

/// Replaces `old` with `new` in `plain`. Exactly one occurrence is required unless
/// `replace_all`. Returns the new body and the number of replacements.
pub(crate) fn apply_edit(
    plain: &str,
    old: &str,
    new: &str,
    replace_all: bool,
) -> std::result::Result<(String, usize), String> {
    if old.is_empty() {
        return Err("old_string must not be empty.".to_string());
    }
    if old == new {
        return Err("old_string and new_string are identical; nothing to change.".to_string());
    }
    match plain.matches(old).count() {
        0 => Err(
            "old_string was not found in the document body. Copy it verbatim from document_content_get output (without the line-number prefix)."
                .to_string(),
        ),
        1 => Ok((plain.replacen(old, new, 1), 1)),
        n if replace_all => Ok((plain.replace(old, new), n)),
        n => Err(format!(
            "old_string matches {n} times. Add surrounding context to make it unique, or set replace_all."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = "# Title\nline two\n## Section\nline four\n";

    fn re(pattern: &str) -> Regex {
        Regex::new(pattern).unwrap()
    }

    #[test]
    fn normalizes_crlf_only_when_present() {
        assert!(matches!(normalize_newlines("a\nb"), Cow::Borrowed(_)));
        assert_eq!(normalize_newlines("a\r\nb\r\n"), "a\nb\n");
    }

    #[test]
    fn pages_by_line_range() {
        let p = page(DOC, 2, 2, None);
        assert_eq!(p.text, "2\tline two\n3\t## Section\n");
        assert_eq!(
            (p.returned_lines, p.next_line, p.truncated_line),
            (2, Some(4), None)
        );

        // Limit reached exactly at the last line: nothing remains.
        assert_eq!(page(DOC, 3, 2, None).next_line, None);
        // Limit larger than the rest.
        assert_eq!(page(DOC, 3, 50, None).returned_lines, 2);
        // Start beyond the end.
        assert_eq!(page(DOC, 10, 5, None), Page::default());
        assert_eq!(page("", 1, 5, None), Page::default());
    }

    #[test]
    fn clamps_start_line_and_limit() {
        let p = page(DOC, 0, 0, None);
        assert_eq!(p.text, "1\t# Title\n");
        assert_eq!(p.next_line, Some(2));
        assert_eq!(page(DOC, 1, 5000, None).returned_lines, 4);
    }

    #[test]
    fn search_returns_matching_lines_only() {
        let p = page(DOC, 1, 200, Some(&re("^#{1,6} ")));
        assert_eq!(p.text, "1\t# Title\n3\t## Section\n");
        assert_eq!(p.next_line, None);

        // Limit reached in search mode: next_line points past the last match without look-ahead.
        let p = page(DOC, 1, 1, Some(&re("line")));
        assert_eq!(p.text, "2\tline two\n");
        assert_eq!(p.next_line, Some(3));

        let p = page(DOC, 2, 200, Some(&re("(?i)TITLE")));
        assert_eq!(p, Page::default());
    }

    #[test]
    fn oversized_first_line_is_cut_and_cursor_advances() {
        let doc = format!("{}\nshort\n", "x".repeat(MAX_PAGE_BYTES + 10_000));
        let p = page(&doc, 1, 200, None);
        assert!(p.text.len() <= MAX_PAGE_BYTES);
        assert!(p.text.starts_with("1\txxx"));
        assert_eq!(
            (p.returned_lines, p.next_line, p.truncated_line),
            (1, Some(2), Some(1))
        );
        assert_eq!(page(&doc, 2, 200, None).text, "2\tshort\n");
    }

    #[test]
    fn oversized_later_line_ends_the_page_before_it() {
        let doc = format!("short\n{}\ntail\n", "y".repeat(MAX_PAGE_BYTES));
        let p = page(&doc, 1, 200, None);
        assert_eq!(p.text, "1\tshort\n");
        assert_eq!((p.next_line, p.truncated_line), (Some(2), None));
        let p = page(&doc, 2, 200, None);
        assert!(p.text.len() <= MAX_PAGE_BYTES);
        assert_eq!((p.next_line, p.truncated_line), (Some(3), Some(2)));
        assert_eq!(page(&doc, 3, 200, None).text, "3\ttail\n");
    }

    #[test]
    fn oversized_search_match_is_cut() {
        let doc = format!("a\nneedle {}\nneedle b\n", "z".repeat(MAX_PAGE_BYTES));
        let p = page(&doc, 1, 200, Some(&re("needle")));
        assert!(p.text.len() <= MAX_PAGE_BYTES);
        assert!(p.text.starts_with("2\tneedle zzz"));
        assert_eq!((p.next_line, p.truncated_line), (Some(3), Some(2)));
    }

    #[test]
    fn oversized_multibyte_line_is_cut_on_char_boundary() {
        let doc = "あ".repeat(MAX_PAGE_BYTES / 3 + 1000);
        let p = page(&doc, 1, 200, None);
        assert!(p.text.len() <= MAX_PAGE_BYTES);
        assert!(p.text.ends_with("あ\n"));
        assert_eq!((p.next_line, p.truncated_line), (None, Some(1)));
    }

    #[test]
    fn applies_unique_replacement() {
        assert_eq!(
            apply_edit("a\nb\nc", "b", "B", false),
            Ok(("a\nB\nc".into(), 1))
        );
        assert_eq!(
            apply_edit("a\nb\nb", "b", "B", true),
            Ok(("a\nB\nB".into(), 2))
        );
    }

    #[test]
    fn rejects_invalid_edits() {
        assert!(apply_edit("abc", "", "x", false).is_err());
        assert!(apply_edit("abc", "b", "b", false).is_err());
        assert!(
            apply_edit("abc", "z", "x", false)
                .unwrap_err()
                .contains("not found")
        );
        assert!(
            apply_edit("aba", "a", "x", false)
                .unwrap_err()
                .contains("2 times")
        );
    }
}
