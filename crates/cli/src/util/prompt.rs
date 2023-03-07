use std::fmt::Display;

use anyhow::{Context, Result};

use railwayapp_graphql::queries::project::ProjectProjectServicesEdgesNode;
use railwayapp_graphql::queries::project_plugins::PluginType;

use crate::commands::Configs;

pub fn prompt_options<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
    let select = inquire::Select::new(message, options);
    select
        .with_render_config(Configs::get_render_config())
        .prompt()
        .context("Failed to prompt for options")
}

pub fn prompt_confirm(message: &str) -> Result<bool> {
    let confirm = inquire::Confirm::new(message);
    confirm
        .with_render_config(Configs::get_render_config())
        .prompt()
        .context("Failed to prompt for confirm")
}

pub fn prompt_confirm_with_default(message: &str, default: bool) -> Result<bool> {
    let confirm = inquire::Confirm::new(message);
    confirm
        .with_default(default)
        .with_render_config(Configs::get_render_config())
        .prompt()
        .context("Failed to prompt for confirm")
}

pub fn prompt_multi_options<T: Display>(message: &str, options: Vec<T>) -> Result<Vec<T>> {
    let multi_select = inquire::MultiSelect::new(message, options);
    multi_select
        .with_render_config(Configs::get_render_config())
        .prompt()
        .context("Failed to prompt for multi options")
}

pub fn prompt_text(message: &str) -> Result<String> {
    let text = inquire::Text::new(message);
    text.with_render_config(Configs::get_render_config())
        .prompt()
        .context("Failed to prompt for text")
}

pub fn prompt_select<T: Display>(message: &str, options: Vec<T>) -> Result<T> {
    inquire::Select::new(message, options)
        .with_render_config(Configs::get_render_config())
        .prompt()
        .context("Failed to prompt for select")
}

#[derive(Debug, Clone)]
pub struct PromptService<'a>(pub &'a ProjectProjectServicesEdgesNode);

impl<'a> Display for PromptService<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

pub fn plugin_enum_to_string(plugin: &PluginType) -> String {
    match plugin {
        PluginType::postgresql => "PostgreSQL".to_owned(),
        PluginType::mysql => "MySQL".to_owned(),
        PluginType::redis => "Redis".to_owned(),
        PluginType::mongodb => "MongoDB".to_owned(),
        PluginType::Other(other) => other.to_owned(),
    }
}
