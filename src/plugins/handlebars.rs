use crate::plugins::PluginDefinition;
use crate::plugins::PluginRole::Transform;
use crate::{utils, InputFile};
use handlebars::Handlebars;
use serde_json::json;

pub fn plugin() -> PluginDefinition {
    PluginDefinition {
        name: "handlebars".to_string(),
        role: Transform,
        output_filename: None,
        process: None,
        pre_process: Some(|config, context| {
            let mut new_files: Vec<InputFile> = vec![];
            for file in config.files.as_ref().unwrap() {
                if file.is_extension(".mustache") {
                    let file_contents = file.read_contents();

                    let template_registry = Handlebars::new();
                    let template = template_registry
                        .render_template(file_contents.as_str(), &json!(&context))
                        .unwrap();

                    let new_file = file.replace_extensions("");
                    utils::create_file(new_file.path.to_string(), template);
                    new_files.push(new_file);
                }
            }
            (context, Some(new_files))
        }),
        applies: None,
        post_process: Some(|config, _context| {
            for file in config.files.as_ref().unwrap() {
                if file.is_extension(".mustache") {
                    utils::remove_file(file.replace_path_extension(""))
                }
            }
        }),
    }
}
