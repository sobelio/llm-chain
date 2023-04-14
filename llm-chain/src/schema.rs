use std::collections::HashMap;

pub struct Document {
    pub page_content: String,
    pub metadata: HashMap<String, String>,
}
