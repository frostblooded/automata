use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Transition<T> {
    pub from: u32,
    pub label: T,
    pub to: u32
}

impl<T> Transition<T> {
    pub fn new(new_from: u32, new_label: T, new_to: u32) -> Self {
        Transition {
            from: new_from,
            label: new_label,
            to: new_to
        }
    }
}

impl<T: PartialEq + Eq + Ord> Ord for Transition<T> {
    fn cmp(&self, other: &Transition<T>) -> Ordering {
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

impl<T: PartialEq + Eq + Ord> PartialOrd for Transition<T> {
    fn partial_cmp(&self, other: &Transition<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
