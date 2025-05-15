use std::{collections::HashMap, sync::Arc};

use futures_util::future::BoxFuture;
use serde_json::Value;

use crate::{
    Frame, FuncRegistryBuilder,
    func_registry::{CallFunctionError, dyn_wrapper::DynFunctionType},
    query::QueryCallback,
};

/// A registry for functions that can be called from JavaScript.
#[derive(Default, Clone)]
pub struct FuncRegistry {
    pub(crate) functions: Arc<HashMap<String, Box<dyn DynFunctionType>>>,
    pub(crate) spawner: Option<Arc<dyn Fn(BoxFuture<'static, ()>) + Send + Sync>>,
}

impl FuncRegistry {
    /// Creates a new `FuncRegistryBuilder`.
    #[inline]
    pub fn build() -> FuncRegistryBuilder {
        FuncRegistryBuilder::default()
    }

    pub(crate) fn call(&self, frame: Frame, name: &str, args: Vec<Value>, callback: QueryCallback) {
        let Some(func) = self.functions.get(name) else {
            callback.result(Err(CallFunctionError::NotFound(name.to_string())));
            return;
        };
        func.call(self.spawner.as_deref(), frame, args, callback)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&str, usize)> {
        self.functions
            .iter()
            .map(|(name, func)| (name.as_str(), func.num_arguments()))
    }
}
