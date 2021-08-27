use crate::{Config, InputFile};
use std::collections::HashMap;

pub mod assets;
pub mod data;
pub mod handlebars;
pub mod markdown;

pub type Context = HashMap<String, String>;

type PreprocessingOutput = (Context, Option<Vec<InputFile>>);

pub enum PluginRole {
    LoadContext,
    Prep,
    Transform,
    Custom,
}

pub struct PluginDefinition {
    pub name: String,
    pub role: PluginRole,
    pub output_filename: Option<fn(file: &InputFile, output: String) -> String>,
    pub applies: Option<fn(file: &InputFile) -> bool>,
    pub pre_process: Option<fn(config: &Config, context: Context) -> PreprocessingOutput>,
    pub process: Option<fn(file_contents: String, context: Context) -> String>,
    pub post_process: Option<fn(config: &Config, context: Context)>,
}
