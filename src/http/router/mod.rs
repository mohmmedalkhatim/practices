use crate::http::requst::Requst;
use crate::http::respone::Respone;
use std::collections::HashMap;

pub struct Router {
    pub routes: Option<HashMap<String, Box<fn(Requst) -> Respone>>>,
}
impl Router {
    pub fn gethandler(self, s: String) -> Result<Box<fn(Requst) -> Respone>, String> {
        match self.routes {
            Some(state) => {
                let fun = state.get(&s);
                match fun {
                    Some(s) => Ok(s.to_owned()),
                    None => Err("couldn't find the handler".to_string()),
                }
            }
            None => {
                panic!("the router is missing")
            }
        }
    }
}
