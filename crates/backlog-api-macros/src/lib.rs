use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod to_form_params;

/// Derives the `From<&Self> for Vec<(String, String)>` implementation for API parameter structs.
///
/// This macro automatically generates form parameter serialization code for Backlog API parameters.
/// It supports various field types and attributes for customization.
///
/// # Attributes
///
/// - `#[form(skip)]` - Skip this field during serialization
/// - `#[form(name = "customName")]` - Use custom field name in API
/// - `#[form(array)]` - Treat as array parameter (adds `[]` suffix)
/// - `#[form(date_format = "%Y-%m-%d")]` - Format DateTime fields with the specified format
///
/// # Examples
///
/// ```rust
/// use backlog_api_macros::ToFormParams;
///
/// #[derive(ToFormParams)]
/// struct AddCommentParams {
///     content: String,
///     #[form(array, name = "notifiedUserId")]
///     notified_user_ids: Option<Vec<u32>>,
///     #[form(skip)]
///     internal_field: String,
/// }
/// ```
#[proc_macro_derive(ToFormParams, attributes(form))]
pub fn derive_to_form_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    to_form_params::expand_derive_to_form_params(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
