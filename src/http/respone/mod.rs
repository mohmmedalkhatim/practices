use std::collections::HashMap;

pub struct Respone {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}
