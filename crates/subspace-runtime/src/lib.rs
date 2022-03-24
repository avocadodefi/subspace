//! Subspace runtime

#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub use parachain_template_runtime as execution;
pub use subspace_consensus_runtime as consensus;
