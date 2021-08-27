use crate::plugins::PluginDefinition;
use crate::plugins::PluginRole::Custom;
use crate::utils;

pub fn plugin() -> PluginDefinition {
    PluginDefinition {
        name: "assets".to_string(),
        role: Custom,
        output_filename: None,
        applies: None,
        pre_process: None,
        process: None,
        post_process: Some(|config, _context| {
            let mut css_contents: Vec<String> = vec![];
            let mut js_contents: Vec<String> = vec![];

            for input_file in config.files.as_ref().unwrap() {
                if input_file
                    .is_in_directory(config.assets_directory_name(), config.root_directory_path())
                {
                    if input_file.is_extension(".css") {
                        css_contents.push(input_file.read_contents());
                    } else if input_file.is_extension(".js") {
                        js_contents.push(input_file.read_contents());
                    }

                    let output_filename = config.output_filename(input_file);
                    input_file.create_directory(output_filename.to_string());

                    utils::copy_file(input_file.path.to_string(), output_filename.to_string());
                }
            }

            if !css_contents.is_empty() || !js_contents.is_empty() {
                let output_path = config.root_directory_path()
                    + "/"
                    + config.output_directory_name().as_str()
                    + "/"
                    + config.assets_directory_name().as_str();

                utils::create_directory(output_path.to_string());

                if !css_contents.is_empty() {
                    utils::create_file(
                        output_path.as_str().to_owned() + "/app.css",
                        css_contents.join("\n"),
                    );
                }

                if !js_contents.is_empty() {
                    utils::create_file(
                        output_path.as_str().to_owned() + "/app.js",
                        js_contents.join("\n"),
                    );
                }
            }
        }),
    }
}
