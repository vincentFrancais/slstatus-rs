use super::{Block, Block2, BlockComponent};

pub fn datetime_block() -> Block {
    Block {
        func: Box::new(|| chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
    }
}

struct DateTimeComponent {}

impl BlockComponent for DateTimeComponent {
    fn call(&self) -> String {
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

fn datetime_block2() -> Block2 {
    Block2::new(DateTimeComponent {})
}
