use std::time::Duration;

use anyhow::bail;
use clap::ValueEnum;
use is_terminal::IsTerminal;

use crate::consts::PLUGINS;
use crate::consts::TICK_STRING;
use crate::util::prompt::plugin_enum_to_string;
use crate::util::prompt::prompt_multi_options;

use super::*;

/// Add a new plugin to your project
#[derive(Parser)]
pub struct Args {
    /// The name of the plugin to add
    #[arg(short, long, value_enum)]
    plugin: Vec<ClapPluginEnum>,
}

pub async fn command(args: Args, _json: bool) -> Result<()> {
    let configs = Configs::new()?;

    let client = GQLClient::new_authorized(&configs)?;
    let linked_project = configs.get_linked_project().await?;

    let vars = queries::project_plugins::Variables {
        id: linked_project.project.clone(),
    };

    let res =
        post_graphql::<queries::ProjectPlugins, _>(&client, configs.get_backboard(), vars).await?;

    let body = res.data.context("Failed to retrieve response body")?;

    let project_plugins: Vec<_> = body
        .project
        .plugins
        .edges
        .iter()
        .map(|p| plugin_enum_to_string(&p.node.name))
        .collect();

    let filtered_plugins: Vec<_> = PLUGINS
        .iter()
        .map(|p| p.to_string())
        .filter(|plugin| !project_plugins.contains(&plugin.to_string()))
        .collect();

    let selected = if !std::io::stdout().is_terminal() || !args.plugin.is_empty() {
        if args.plugin.is_empty() {
            bail!("No plugins specified");
        }
        let filtered: Vec<_> = args
            .plugin
            .iter()
            .map(clap_plugin_enum_to_plugin_enum)
            .map(|p| plugin_enum_to_string(&p))
            .filter(|plugin| !project_plugins.contains(&plugin.to_string()))
            .collect();

        if filtered.is_empty() {
            bail!("Plugins already exist");
        }

        filtered
    } else {
        prompt_multi_options("Select plugins to add", filtered_plugins)?
    };

    if selected.is_empty() {
        bail!("No plugins selected");
    }

    for plugin in selected {
        let vars = mutations::plugin_create::Variables {
            project_id: linked_project.project.clone(),
            name: plugin.to_lowercase(),
        };
        if std::io::stdout().is_terminal() {
            let spinner = indicatif::ProgressBar::new_spinner()
                .with_style(
                    indicatif::ProgressStyle::default_spinner()
                        .tick_chars(TICK_STRING)
                        .template("{spinner:.green} {msg}")?,
                )
                .with_message(format!("Creating {plugin}..."));
            spinner.enable_steady_tick(Duration::from_millis(100));
            post_graphql::<mutations::PluginCreate, _>(&client, configs.get_backboard(), vars)
                .await?;
            spinner.finish_with_message(format!("Created {plugin}"));
        } else {
            println!("Creating {}...", plugin);
            post_graphql::<mutations::PluginCreate, _>(&client, configs.get_backboard(), vars)
                .await?;
        }
    }

    Ok(())
}

#[derive(ValueEnum, Clone, Debug)]
enum ClapPluginEnum {
    Postgresql,
    Mysql,
    Redis,
    Mongodb,
}

fn clap_plugin_enum_to_plugin_enum(
    clap_plugin_enum: &ClapPluginEnum,
) -> queries::project_plugins::PluginType {
    match clap_plugin_enum {
        ClapPluginEnum::Postgresql => queries::project_plugins::PluginType::postgresql,
        ClapPluginEnum::Mysql => queries::project_plugins::PluginType::mysql,
        ClapPluginEnum::Redis => queries::project_plugins::PluginType::redis,
        ClapPluginEnum::Mongodb => queries::project_plugins::PluginType::mongodb,
    }
}
