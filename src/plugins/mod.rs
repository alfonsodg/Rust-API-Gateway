//! Plugin architecture for extensible middleware.

pub mod examples;
pub mod plugin;
pub mod registry;

pub use plugin::{BoxedPlugin, Plugin, PluginContext, PluginPhase, PluginResult};
pub use registry::PluginRegistry;
