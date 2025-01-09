use std::collections::HashMap;

pub struct Requst {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}
