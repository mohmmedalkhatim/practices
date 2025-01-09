use crate::http::router::Router;
pub struct Server {
    pub router: Router,
}

impl Server {
    fn bind(port: i32) -> Self {
        if port > 10000 {
            panic!("you have to add a port smailler 10000")
        }
        Server {
            router: Router { routes: None },
        }
    }
    fn router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }
}
