//! Plugin architecture for extensible middleware.

pub mod plugin;
pub mod registry;
pub mod examples;

pub use plugin::{Plugin, PluginContext, PluginPhase, PluginResult, BoxedPlugin};
pub use registry::PluginRegistry;
