
#[derive(Debug,Clone,Default)]
pub struct Hero {
    pub name: String,
    pub hb: i32,
    pub mp: i32,
    pub postion: (i32, i32),
}
impl Hero {
    pub fn new(N: String) -> Hero {
        Hero {
            name: N,
            hb: 10000,
            mp: 100,
            postion: (0, 0),
        }
    }
}
