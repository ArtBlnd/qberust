use crate::symbol::{Symbol, Symbolic};
use crate::values::instruction::Phi;
use crate::values::{Value, ValueRef, ValueTable};

use std::mem;
use std::ops::Deref;
use std::sync::Arc;

use parking_lot::Mutex;

pub struct BlockData {
    pub(crate) phi: Option<Phi>,
    pub(crate) jmp: Jumps,
    pub(crate) values: ValueTable,
}

#[derive(Clone)]
pub struct Block {
    symbol: Symbol,
    data: Arc<Mutex<BlockData>>,
}

impl Block {
    /// Set phi instruction on block
    /// only one phi instruction exists per one block unlike other instruction.
    pub fn set_phi(&self, phi: Phi) -> Option<Phi> {
        let mut data = self.data.try_lock().unwrap();
        mem::replace(&mut data.phi, Some(phi))
    }

    /// Set destination block of current block
    pub fn set_jmp(&self, jmp: Jumps) -> Jumps {
        let mut data = self.data.try_lock().unwrap();
        mem::replace(&mut data.jmp, jmp)
    }

    /// Inserts value into value table and get reference
    pub fn insert_value(&self, value: Value) -> ValueRef {
        let mut data = self.data.try_lock().unwrap();
        data.values.insert(value)
    }

    /// removes value from value table
    /// related value referenes will be invalidated.
    pub fn remove_value(&self, vref: ValueRef) -> Option<Value> {
        let mut data = self.data.try_lock().unwrap();
        data.values.remove(vref)
    }

    /// Visit inner block data.
    /// you only have access internal block data by using this function because of locking.
    pub fn visit_data<F>(&self, f: F)
    where
        F: Fn(&BlockData),
    {
        let data = self.data.try_lock().unwrap();
        (f)(data.deref())
    }
}

impl Symbolic for Block {
    fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}

pub enum Jumps {
    None,
    Unconditional {
        block: Block,
    },
    IfNotZF {
        block: Block,
        lhs: ValueRef,
        rhs: ValueRef,
    },
}
