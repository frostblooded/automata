use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::ops::Deref;
use std::clone::Clone;
use std::convert::From;

// A wrapper struct for the HashSet struct that allows us
// to, in a way, implement the Hash trait for HashSet.
// This isn't otherwise possible as both HashSet and
// Hash are things that are not ours.
pub struct HashableSet<T> {
    set: HashSet<T>
}

impl<T: Hash + Eq> Hash for HashableSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in &self.set {
            item.hash(state);
        }
    }
}

impl<T> Deref for HashableSet<T> {
    type Target = HashSet<T>;

    fn deref(&self) -> &HashSet<T> {
        &self.set
    }
}

impl<T> From<HashSet<T>> for HashableSet<T> {
    fn from(hash_set: HashSet<T>) -> Self {
        HashableSet::<T> {
            set: hash_set
        }
    }
}

impl<T: Clone> Clone for HashableSet<T> {
    fn clone(&self) -> Self {
        HashableSet::<T> {
            set: self.set.clone()
        }
    }
}

impl<T: Eq + Hash> PartialEq for HashableSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set
    }
}

impl<T: Eq + Hash> Eq for HashableSet<T> {}
