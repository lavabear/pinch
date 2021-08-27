use crate::plugins::PluginDefinition;
use crate::plugins::PluginRole::LoadContext;
use std::collections::HashMap;

pub fn plugin() -> PluginDefinition {
    PluginDefinition {
        name: "data".to_string(),
        role: LoadContext,
        output_filename: None,
        applies: None,
        pre_process: Some(|config, mut context| {
            for input_file in config.files.as_ref().unwrap() {
                if input_file
                    .is_in_directory(config.data_directory_name(), config.root_directory_path())
                {
                    let additional_context: HashMap<String, String> =
                        serde_json::from_str(input_file.read_contents().as_str()).unwrap();
                    context = context.into_iter().chain(additional_context).collect();
                }
            }
            (context, None)
        }),
        process: None,
        post_process: None,
    }
}
