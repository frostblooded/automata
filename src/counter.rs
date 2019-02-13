#[derive(Debug)]
pub struct Counter {
    pub value: u32
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: 0
        }
    }

    pub fn tick(&mut self) -> u32 {
        let return_value = self.value;
        self.value += 1;
        return_value
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}