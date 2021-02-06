use eyre::Result;
use eyre::WrapErr;
use std::collections::HashMap;
use std::{path::PathBuf, process::Command};
use structopt::StructOpt;

use serde::Deserialize;
use serde_yaml::from_reader;

#[derive(Debug, StructOpt)]
struct Args {
    /// Path to the configuration file
    pub config: Option<PathBuf>,
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    /// List known hosts
    List,
    /// Connect to a known host
    Connect {
        /// Host to connect to; Must be defined in the configuration file
        host: String,
    },
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    pub hosts: HashMap<String, ConfigHost>,
}

const fn default_port() -> u16 {
    22
}

#[derive(Debug, Deserialize)]
struct ConfigHost {
    pub user_name: String,
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl ConfigHost {
    pub fn as_ssh_args(&self) -> Vec<String> {
        let mut args = vec![];

        args.push("-p".into());
        args.push(format!("{}", self.port));
        args.push(format!("{}@{}", self.user_name, self.host));

        args
    }
}

const DEFAULT_LOCATION: &'static str = ".ssh_known_hosts.yml";

fn get_default_location() -> Result<PathBuf> {
    let mut path = PathBuf::from(
        std::env::var("HOME").wrap_err("Could not get value of home directory for user")?,
    );
    path.push(DEFAULT_LOCATION);
    Ok(path)
}

fn print_hosts(hosts: &HashMap<String, ConfigHost>) {
    let min_width = hosts
        .iter()
        .fold(6, |i, (k, _)| if k.len() > i { k.len() } else { i });

    let total_width = hosts.iter().fold(
        9,
        |c, (_, v)| if v.host.len() > c { v.host.len() } else { c },
    ) + min_width
        + 3;

    eprintln!("{1:<0$}   Real Host", min_width, "Local");
    eprintln!("{1:->0$}", total_width, "");

    hosts.iter().for_each(|(k, v)| {
        eprintln!("{1:<0$}   {2}", min_width, k, v.host);
    });
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::from_args();

    log::info!("Got arguments:\n{:#?}", &args);

    let config = {
        let config_location = match args.config {
            Some(config) => config,
            None => get_default_location().wrap_err("Could not get default location")?,
        };

        let file = std::fs::File::open(&config_location).wrap_err_with(|| {
            format!(
                "Not able to read configuration file at {}",
                &config_location.display()
            )
        })?;

        log::info!("Got config file from {}", &config_location.display());

        let reader = std::io::BufReader::new(file);

        let config =
            from_reader::<_, ConfigFile>(reader).wrap_err("Could not parse configuration file")?;

        log::info!("Got configuration values from file:\n{:#?}", &config);

        config
    };

    match args.subcommand {
        SubCommand::Connect { host } => {
            log::info!(
                "Attempting to get host information from configuration for {}",
                &host
            );

            if let Some(host) = config.hosts.get(&host) {
                use std::os::unix::process::CommandExt;

                log::info!(
                    "Found host information. Attempting to connecto to {}",
                    &host.host
                );

                // Usually ends execution of self, so if we get past exec its always an error
                Err(Command::new("ssh").args(&host.as_ssh_args()).exec())
                    .wrap_err("Could not execute process")
            } else {
                log::warn!("No host found, warning user");

                eprintln!("No host found with name '{}'\n", &host);

                print_hosts(&config.hosts);

                log::trace!("Finished printing hosts, Exiting");
                Ok(())
            }
        }
        SubCommand::List => {
            log::trace!("Printing hosts");
            print_hosts(&config.hosts);
            log::trace!("Finished printing hosts");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() -> Result<()> {
        let config_str = include_str!("../.ssh_known_hosts.example.yml");

        let config = serde_yaml::from_str::<ConfigFile>(config_str)
            .wrap_err("Could not deserialize example known hosts")?;

        Ok(())
    }
}
