use qberust_ir::symbol::SymbolTable;

pub struct Module {
    global_symbol: SymbolTable,
    global_alignment: usize,
}