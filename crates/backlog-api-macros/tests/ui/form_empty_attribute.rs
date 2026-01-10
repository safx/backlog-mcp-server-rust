use backlog_api_macros::ToFormParams;

#[derive(ToFormParams)]
struct EmptyFormAttribute {
    content: String,
    #[form]
    field_with_empty_form: String,
}

fn main() {}
