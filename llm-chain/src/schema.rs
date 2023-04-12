use std::collections::HashMap;

#[derive(Clone)]
pub struct Document<T: ToString> {
    page_content: T,
    metadata: HashMap<String, String>,
}
