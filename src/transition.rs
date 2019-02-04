#[derive(Hash, PartialEq, Eq)]
pub struct State {
    id: u32
}

#[derive(Hash, PartialEq, Eq)]
pub struct Transition<S> {
    from: State,
    to: State,
    label: Option<S>
}
