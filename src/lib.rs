use std::collections::HashMap;

use crate::plugins::{Context, PluginDefinition, PluginRole};
use serde::Deserialize;
use std::cmp::Ordering;
use toml::Value;
use walkdir::{DirEntry, WalkDir};

pub mod plugins;
pub mod utils;

#[derive(Debug, Deserialize, Clone)]
pub struct InputFile {
    pub filename: String,
    pub path: String,
    pub is_directory: bool,
    pub extension: String,
}

impl InputFile {
    pub fn create_directory(&self, output_filename: String) -> String {
        utils::create_directory(
            output_filename
                .as_str()
                .replace(&self.filename.to_string(), ""),
        );
        output_filename
    }

    pub fn replace_extensions(&self, new_value: &str) -> Self {
        let new_filepath = self.replace_path_extension(new_value);

        InputFile {
            filename: self.replace_filename_extension(new_value),
            path: new_filepath.to_string(),
            is_directory: self.is_directory,
            extension: utils::file_extension(new_filepath.as_str()),
        }
    }

    fn from(dir_entry: DirEntry) -> Self {
        let filename = dir_entry.file_name().to_str().unwrap().to_string();
        InputFile {
            filename: filename.to_string(),
            extension: utils::file_extension(filename.as_str()),
            path: dir_entry.path().to_str().unwrap().to_string(),
            is_directory: dir_entry.file_type().is_dir(),
        }
    }

    pub fn replace_filename_extension(&self, new_value: &str) -> String {
        self.filename.replace(self.extension.as_str(), new_value)
    }

    pub fn replace_path_extension(&self, new_value: &str) -> String {
        self.path.replace(self.extension.as_str(), new_value)
    }

    pub fn read_contents(&self) -> String {
        utils::read_file(self.path.to_string())
    }

    pub fn is_in_directory(&self, directory: String, root: String) -> bool {
        self.path.replace(&(root + "/"), "").starts_with(&directory)
    }

    pub fn is_extension(&self, extension: &str) -> bool {
        self.extension.eq(extension)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: Option<String>,
    pub description: Option<String>,
    pub assets_directory_name: Option<String>,
    pub data_directory_name: Option<String>,
    pub output_directory_name: Option<String>,
    pub root_directory_path: Option<String>,
    pub config_filename: Option<String>,
    pub files: Option<Vec<InputFile>>,
    pub plugin_options: Option<HashMap<String, Value>>,
}

impl Config {
    pub fn sorted_plugins(&self) {}

    pub fn from_file(file_path: &str) -> Self {
        let contents = utils::read_file(file_path.to_string());
        let mut config: Config = toml::from_str(&contents).unwrap();
        config.config_filename = Some("inapinch.toml".to_string()); // TODO: pull from `file_path`
        config.root_directory_path =
            Some(file_path.replace(&("/".to_owned() + config.config_filename().as_str()), ""));
        config.files = Some(config.find_files());
        config
    }

    fn find_files(&self) -> Vec<InputFile> {
        let mut files = Vec::new();
        for entry in WalkDir::new(self.root_directory_path()) {
            let input_file = InputFile::from(entry.unwrap());
            if !input_file.is_directory
                && input_file.filename.ne(&self.config_filename())
                && !input_file
                    .is_in_directory(self.output_directory_name(), self.root_directory_path())
            {
                files.push(input_file);
            }
        }
        files
    }

    fn root_directory_path(&self) -> String {
        self.root_directory_path.as_ref().unwrap().to_string()
    }

    fn config_filename(&self) -> String {
        utils::to_string(self.config_filename.as_ref(), "inapinch.toml")
    }

    fn output_filename(&self, input_file: &InputFile) -> String {
        let file_root = self.root_directory_path();
        input_file.path.as_str().replace(
            file_root.to_string().as_str(),
            &*(file_root + "/" + self.output_directory_name().as_str()),
        )
    }

    fn assets_directory_name(&self) -> String {
        utils::to_string(self.assets_directory_name.as_ref(), "assets")
    }

    fn data_directory_name(&self) -> String {
        utils::to_string(self.data_directory_name.as_ref(), "data")
    }

    fn output_directory_name(&self) -> String {
        utils::to_string(self.output_directory_name.as_ref(), "dist")
    }
}

pub struct Pinch {
    pub config: Config,
    pub plugins: HashMap<String, PluginDefinition>,
    pub context: Context,
}

impl Pinch {
    pub fn from_config(config: Config) -> Self {
        Pinch {
            config,
            plugins: HashMap::new(),
            context: HashMap::new(),
        }
    }

    pub fn from_file(file_path: &str) -> Self {
        Pinch::from_config(Config::from_file(file_path))
    }

    pub fn register_file(&mut self, input_file: InputFile) {
        let mut files = self.config.files.clone().unwrap();
        files.push(input_file);
        self.config.files = Some(files);
    }

    pub fn register_plugin(&mut self, plugin: PluginDefinition) {
        self.plugins.insert(plugin.name.to_string(), plugin);
    }

    pub fn remove_plugin(&mut self, name: String) -> Option<PluginDefinition> {
        self.plugins.remove(name.as_str())
    }

    pub fn build_with_defaults(&mut self) {
        if self.plugins.is_empty() {
            self.plugins = HashMap::new();
            self.register_plugin(plugins::assets::plugin());
            self.register_plugin(plugins::data::plugin());
            self.register_plugin(plugins::handlebars::plugin());
            self.register_plugin(plugins::markdown::plugin());
        }
        self.build()
    }

    pub fn build(&mut self) {
        self.pre_process();
        self.process_files();
        self.post_process();
    }

    fn pre_process(&mut self) {
        if self.config.files.is_none() {
            panic!("No files configured")
        }

        utils::create_directory(
            self.config.root_directory_path() + "/" + self.config.output_directory_name().as_str(),
        );

        let mut plugins: Vec<&PluginDefinition> = self.plugins.values().collect();
        plugins.sort_by(|a, b| match a.role {
            PluginRole::LoadContext => match b.role {
                PluginRole::LoadContext => Ordering::Equal,
                PluginRole::Prep => Ordering::Greater,
                _ => Ordering::Less,
            },
            PluginRole::Prep => match b.role {
                PluginRole::LoadContext => Ordering::Less,
                PluginRole::Prep => Ordering::Equal,
                _ => Ordering::Less,
            },
            PluginRole::Transform => match b.role {
                PluginRole::LoadContext => Ordering::Greater,
                PluginRole::Prep => Ordering::Greater,
                PluginRole::Transform => Ordering::Equal,
                PluginRole::Custom => Ordering::Less,
            },
            PluginRole::Custom => match b.role {
                PluginRole::Custom => Ordering::Equal,
                _ => Ordering::Greater,
            },
        });
        let mut all_new_files: Vec<InputFile> = vec![];
        for plugin in plugins {
            if plugin.pre_process.is_some() {
                let (context, new_files) =
                    plugin.pre_process.unwrap()(&self.config, self.context.to_owned());

                self.context = context;
                if new_files.is_some() {
                    for new_file in new_files.unwrap() {
                        all_new_files.push(new_file);
                    }
                }
            }
        }

        for new_file in all_new_files {
            self.register_file(new_file);
        }
    }

    fn process_files(&self) {
        for file in self.config.files.as_ref().unwrap() {
            for (_name, plugin) in self.plugins.iter() {
                if plugin.applies.is_some() && plugin.applies.unwrap()(file) {
                    let apply_plugin = plugin.process.expect("`process` is required");
                    let output_contents =
                        apply_plugin(file.read_contents(), self.context.to_owned(), &self.config);
                    let output_filename = self.config.output_filename(file);
                    utils::create_directory(
                        output_filename
                            .as_str()
                            .replace(&file.filename.to_string(), ""),
                    );

                    let create_output_filename = plugin.output_filename.expect("`output_filename` isn't set. This field is required for `PluginLifecycle::Process` plugins.");
                    utils::create_file(
                        create_output_filename(file, output_filename),
                        output_contents,
                    );
                }
            }
        }
    }

    fn post_process(&self) {
        for (_name, plugin) in self.plugins.iter() {
            if plugin.post_process.is_some() {
                plugin.post_process.unwrap()(&self.config, self.context.clone());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_options_from_file() {
        let config = Config::from_file("example_apps/basic/inapinch.toml");
        assert_eq!(config.name, Some("basic".to_string()));
        assert_eq!(config.description, None);
        assert_eq!(config.files.unwrap().len(), 1); // index.md, exclude inapinch.toml
    }

    #[test]
    fn build_basic_site() {
        let mut pinch = Pinch::from_file("example_apps/basic/inapinch.toml");
        pinch.build_with_defaults();
        assert_eq!(pinch.plugins.len(), 4); // markdown, data, handlebars, assets

        let index_contents =
            utils::read_file("example_apps/basic/dist/pages/index.html".to_string());
        assert_eq!(index_contents.trim(), "<h1>pinch</h1>");
        // utils::remove_directories("example_apps/basic/dist".to_string());
    }

    #[test]
    fn build_complex_site() {
        Pinch::from_file("example_apps/complex/inapinch.toml").build_with_defaults();

        let index_contents =
            utils::read_file("example_apps/complex/dist/pages/index.html".to_string());
        assert_eq!(
            index_contents.trim(),
            "<h1>Hello Jeff</h1>\n<p>Would you like to subscribe to cat facts?</p>"
        );
        // utils::remove_directories("example_apps/complex/dist".to_string());
    }
}
