#[derive(Debug)]
pub(crate) struct Counter {
    pub(crate) value: u32
}

impl Counter {
    pub(crate) fn new() -> Self {
        Counter {
            value: 0
        }
    }

    pub(crate) fn tick(&mut self) -> u32 {
        let return_value = self.value;
        self.value += 1;
        return_value
    }

    pub(crate) fn reset(&mut self) {
        self.value = 0;
    }
}