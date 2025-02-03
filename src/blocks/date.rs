use super::{Block, BlockComponent};

struct DateTimeComponent {}

impl BlockComponent for DateTimeComponent {
    fn call(&mut self) -> String {
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

pub fn datetime_block() -> Block {
    Block::new(DateTimeComponent {})
}
