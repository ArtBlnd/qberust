#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueRef {
    pub(crate) id_table: usize,
    pub(crate) id: usize,
}
