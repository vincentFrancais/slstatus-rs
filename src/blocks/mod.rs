use std::fmt::Display;

pub mod battery;
pub mod cpu;
pub mod date;
pub mod keymap;
pub mod mem;
pub mod mpris;

// TODO: use Block2 struct to have statefull components

trait BlockComponent {
    fn call(&self) -> String;
}

struct Block2 {
    obj: Box<dyn BlockComponent>,
}

impl Block2 {
    fn new(object: impl BlockComponent + 'static) -> Self {
        Self {
            obj: Box::new(object),
        }
    }
}

pub struct Block {
    pub func: Box<dyn Fn() -> String>,
}

impl Block {
    pub fn show(&self) -> String {
        (self.func)()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show())
    }
}
