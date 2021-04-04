#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;
extern crate sp_io;

use crate::data::ExecutionContext;
use crate::types::{Gas, ModuleTx, ScriptTx, VmResult};

pub mod access_path;
pub mod data;
pub mod gas_schedule;
pub mod mvm;
pub mod types;
pub mod vm_config;

pub trait Vm {
    /// Publishes module to the chain.
    fn publish_module(&self, gas: Gas, module: ModuleTx) -> VmResult;
    /// Execute script.
    fn execute_script(&self, gas: Gas, context: ExecutionContext, tx: ScriptTx) -> VmResult;
    /// Clear vm cache.
    fn clear(&self);
}
