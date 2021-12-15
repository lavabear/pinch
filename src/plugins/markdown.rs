use crate::plugins::PluginDefinition;
use crate::plugins::PluginRole::Transform;
use crate::InputFile;
use pulldown_cmark::{html, Options, Parser};

fn default_markdown_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options
}

pub fn plugin() -> PluginDefinition {
    PluginDefinition {
        name: "markdown".to_string(),
        role: Transform,
        output_filename: Some(|_file: &InputFile, output: String| -> String {
            output.replace(".md", ".html")
        }),
        applies: Some(|file: &InputFile| -> bool { file.is_extension(".md") }),
        process: Some(|file_contents, _context, _config| -> String {
            let markdown_options = default_markdown_options(); // TODO: Pull from config (toml file) with `plugin_options` (probably add config as 3rd param)
            let parser = Parser::new_ext(file_contents.as_str(), markdown_options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            html_output
        }),
        pre_process: None,
        post_process: None,
    }
}
