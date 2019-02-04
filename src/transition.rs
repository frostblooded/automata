#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Transition {
    from: u32,
    label: Option<char>,
    to: u32
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
