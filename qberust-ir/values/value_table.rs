use crate::values::{Constant, Instruction, Value, ValueRef};

use std::iter::Iterator;

pub struct ValueTable {
    id_table: usize,
    table: Vec<Option<Value>>,
}

impl ValueTable {
    pub fn new() -> Self {
        Self {
            id_table: rand::random(),
            table: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: impl Into<Value>) -> ValueRef {
        self.table.push(Some(value.into()));
        ValueRef {
            id_table: self.id_table,
            id: self.table.len() - 1,
        }
    }

    pub fn remove(&mut self, vref: ValueRef) -> Option<Value> {
        if vref.id_table != self.id_table {
            return None;
        }

        if let Some(v) = self.table.get_mut(vref.id) {
            v.take()
        } else {
            None
        }
    }

    pub fn get(&self, vref: impl AsRef<ValueRef>) -> Option<&Value> {
        if let Some(Some(v)) = self.table.get(vref.as_ref().id) {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, vref: impl AsRef<ValueRef>) -> Option<&mut Value> {
        if let Some(Some(v)) = self.table.get_mut(vref.as_ref().id) {
            Some(v)
        } else {
            None
        }
    }

    pub fn iter_instr(&self) -> impl Iterator<Item = &Instruction> {
        InstrIter {
            iter: self.table.iter(),
        }
    }

    pub fn iter_const(&self) -> impl Iterator<Item = &Constant> {
        ConstIter {
            iter: self.table.iter(),
        }
    }
}

struct InstrIter<I> {
    iter: I,
}

impl<'a, I> Iterator for InstrIter<I>
where
    I: Iterator<Item = &'a Option<Value>>,
{
    type Item = &'a Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let v = self.iter.next();
            let value = match v {
                None => return None,
                Some(v) => v,
            };

            if let Some(Value::Instruction(instr)) = value {
                return Some(instr);
            }
        }
    }
}

struct ConstIter<I> {
    iter: I,
}

impl<'a, I> Iterator for ConstIter<I>
where
    I: Iterator<Item = &'a Option<Value>>,
{
    type Item = &'a Constant;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let v = self.iter.next();
            let value = match v {
                None => return None,
                Some(v) => v,
            };

            if let Some(Value::Constant(instr)) = value {
                return Some(instr);
            }
        }
    }
}
