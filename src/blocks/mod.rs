pub mod battery;
pub mod cpu;
pub mod date;
pub mod keymap;
pub mod mem;
pub mod mpris;

pub trait BlockComponent {
    fn call(&mut self) -> String;
}

pub struct Block {
    obj: Box<dyn BlockComponent>,
}

impl Block {
    pub fn new(object: impl BlockComponent + 'static) -> Self {
        Self {
            obj: Box::new(object),
        }
    }

    pub fn show(&mut self) -> String {
        self.obj.call()
    }
}
