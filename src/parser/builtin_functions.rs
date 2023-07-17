use ::anyhow::Result;
use anyhow::{anyhow, Ok};
use std::collections::HashMap;

use crate::object::object::Object;

type BuiltinFn = fn(Option<Vec<Object>>) -> Result<Object>;
#[derive(PartialEq, Clone, Debug)]
pub struct BuiltinFunctions {
    fns: HashMap<String, BuiltinFn>,
}

impl BuiltinFunctions {
    pub fn setup() -> Self {
        let map: HashMap<String, BuiltinFn> = HashMap::new();
        let mut bf = BuiltinFunctions { fns: map };
        bf.set_fn("len".to_string(), len);
        bf
    }
    pub fn get_fn(&self, name: String) -> Option<BuiltinFn> {
        self.fns.get(&name).copied()
    }

    fn set_fn(&mut self, name: String, func: BuiltinFn) -> () {
        self.fns.insert(name, func);
    }
}

fn len(arguments: Option<Vec<Object>>) -> Result<Object> {
    if let Some(args) = arguments {
        match &args[0] {
            Object::String(s) => Ok(Object::Integer(s.len() as isize)),
            _ => Err(anyhow!("Wrong argument type for len function.")),
        }
    } else {
        Err(anyhow!("Argument must be suplied"))
    }
}
