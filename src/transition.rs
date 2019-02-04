#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Transition {
    pub from: u32,
    pub label: Option<char>,
    pub to: u32
}

impl Transition {
    pub fn new(new_from: u32, new_label: Option<char>, new_to: u32) -> Self {
        Transition {
            from: new_from,
            label: new_label,
            to: new_to
        }
    }
}
