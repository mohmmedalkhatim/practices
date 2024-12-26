use std::{cmp::{self, min}, i32::MIN};
#[derive(Debug)]
struct Element {
    val: i32,
    min: usize,
}
impl Element {
    pub new(ssval:usize)->Self{
        Element {
            val,
            min: min(val, self.min),
        }
    }
}
#[derive(Debug)]
pub struct MinStack {
    stack: Vec<Element>,
    min: i32,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: i32::MAX,
        }
    }
    pub fn pop(&mut self) {
        self.min;
        self.stack.pop();
    }
    pub fn push(&mut self, val: i32) {
        self.min = min(val, self.min);
        self.stack.push(Element::new(val))

    }
    pub fn top(&mut self) -> i32 {
        self.stack.last().expect("this valuelable most exists").val
    }
    pub fn get_min(&self)->i32 {
        self.min
    }
}