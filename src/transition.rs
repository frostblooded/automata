use std::cmp::Ordering;

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

impl Ord for Transition {
    fn cmp(&self, other: &Transition) -> Ordering {
        let from_cmp = self.from.cmp(&other.from);
        
        if from_cmp == Ordering::Equal {
            let label_cmp = self.label.cmp(&other.label);

            if label_cmp == Ordering::Equal {
                return self.to.cmp(&other.to);
            }
            else {
                return label_cmp;
            }
        }
        else {
            return from_cmp;
        }
    }
}

impl PartialOrd for Transition {
    fn partial_cmp(&self, other: &Transition) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
