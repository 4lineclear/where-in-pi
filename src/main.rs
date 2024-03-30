use std::sync::Arc;

fn main() {
    let context = Arc::new(where_in_pi::Context::new());
    let output = where_in_pi::split_context(1, 10, context.clone());
    println!("{output:#?}\n\n{context:#?}");
}
