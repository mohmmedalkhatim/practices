pub struct Hero {
    name: String,
    hb: i32,
    mp: i32,
    postion: (i32, i32),
}
impl Hero {
    pub fn new(N: String) -> Hero {
        Hero {
            name: N,
            hb: 100,
            mp: 100,
            postion: (0, 0),
        }
    }
}
