#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Custom(usize),
    Inherited,
}
