use backlog_api_macros::ToFormParams;

#[derive(ToFormParams)]
struct TupleStruct(String, u32);

fn main() {}
