use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use async_trait::async_trait;

use crate::error::BindgenResult;
use crate::DojoData;

pub mod bevy;
pub mod typescript;
pub mod typescript_v2;
pub mod unity;

pub mod godot;

#[derive(Debug)]
pub enum BuiltinPlugins {
    Typescript,
    Unity,
    TypeScriptV2,
    Bevy,
    Godot
}

impl fmt::Display for BuiltinPlugins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltinPlugins::Typescript => write!(f, "typescript"),
            BuiltinPlugins::Unity => write!(f, "unity"),
            BuiltinPlugins::TypeScriptV2 => write!(f, "typescript_v2"),
            BuiltinPlugins::Bevy => write!(f, "bevy"),
            BuiltinPlugins::Godot => write!(f, "godot"),
        }
    }
}

#[async_trait]
pub trait BuiltinPlugin {
    /// Generates code by executing the plugin.
    ///
    /// # Arguments
    ///
    /// * `data` - Dojo data gathered from the compiled project.
    async fn generate_code(&self, data: &DojoData) -> BindgenResult<HashMap<PathBuf, Vec<u8>>>;
}
