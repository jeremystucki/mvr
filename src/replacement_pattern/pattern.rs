#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Element {
    Text(String),
    CaptureGroup(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Pattern {
    pub(crate) elements: Vec<Element>,
}
