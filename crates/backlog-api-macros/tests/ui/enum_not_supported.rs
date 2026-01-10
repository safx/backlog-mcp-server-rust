use backlog_api_macros::ToFormParams;

#[derive(ToFormParams)]
enum NotSupported {
    A,
    B,
}

fn main() {}
