use ::anyhow::Result;
use std::collections::HashMap;

use crate::object::object::Object;

type builtinfn = fn(Option<Vec<Object>>) -> Result<Object>;
pub struct BuiltinFunctions {
    fns: HashMap<String, builtinfn>,
}

impl BuiltinFunctions {
    pub fn get_fn(&self, name: String) -> Option<builtinfn> {
        self.fns.get(&name)
    }
}
