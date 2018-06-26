extern crate toml;

extern crate dirs;

extern crate strfmt;
use strfmt::strfmt;

extern crate duct_sh;

extern crate colored;
use colored::*;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::{collections::HashMap, env, fs, path::Path};

const DEFAULT_CONFIG: &'static str = "
[sources.git]
package_directory = \"{key}\"

[sources.git.commands]
download = \"git clone {location} {package_directory}\"
update = \"git pull\"

################################################################################

[packages]
";

#[derive(Debug, Deserialize)]
struct Source {
    package_directory: String,
    commands: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Package {
    source: String,
    location: String,
    commands: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    sources: HashMap<String, Source>,
    packages: HashMap<String, Package>,
}

fn main() {
    let config_dir = dirs::config_dir().unwrap().join("µmanager");
    let config_path = config_dir.join("config.toml");

    if !config_dir.exists() {
        println!(
            "{} at {:?}, creating it ...",
            "Couldn't find config".cyan(),
            config_path
        );
        fs::create_dir(&config_dir).expect("Could not create config dir.");
        fs::write(&config_path, DEFAULT_CONFIG).expect("Could not write default config.");
    }

    let config: Config = toml::from_str(
        &fs::read_to_string(config_path).expect("Could not read config."),
    ).expect("Could not parse the config.");

    let download_dir = dirs::data_dir().unwrap().join("µmanager");
    if !download_dir.exists() {
        println!(
            "{} at {:?}, creating it ...",
            "Couldn't find download dir".cyan(),
            download_dir
        );
        fs::create_dir(&download_dir).expect("Could not create download dir.");
    }

    env::set_current_dir(download_dir).unwrap();

    config.packages.iter().for_each(|(name, package)| {
        let source = config
            .sources
            .get(&package.source)
            .expect(&format!("Unknown source {:?}", &package.source));

        let mut template_vars = HashMap::new();
        template_vars.insert("key".to_owned(), name.to_owned());
        template_vars.insert("location".to_owned(), package.location.to_owned());

        let package_directory =
            strfmt(&source.package_directory, &template_vars).expect("Invalid 'package_directory'");

        template_vars.insert("package_directory".to_owned(), package_directory.clone());

        let run_commands = || {
            if let Some(commands) = &package.commands {
                if let Some(build_cmd) = commands.get("build") {
                    println!("{} {:?} ...", "Building".green(), name);
                    duct_sh::sh_dangerous(build_cmd).run().unwrap();
                }

                if let Some(install_cmd) = commands.get("install") {
                    println!("{} {:?} ...", "Installing".green(), name);
                    duct_sh::sh_dangerous(install_cmd).run().unwrap();
                }
            }
        };

        if Path::new(&package_directory).exists() {
            assert!(
                fs::metadata(&package_directory)
                    .map(|md| md.is_dir())
                    .unwrap_or(false)
            );

            let update_cmd =
                strfmt(&source.commands["update"], &template_vars).expect("Invalid 'update'");

            let old_wd = env::current_dir().unwrap();
            env::set_current_dir(package_directory).unwrap();

            println!("{} {:?} ...", "Updating".green(), name);
            duct_sh::sh_dangerous(update_cmd).run().unwrap();

            run_commands();

            env::set_current_dir(old_wd).unwrap();
        } else {
            let download_cmd =
                strfmt(&source.commands["download"], &template_vars).expect("Invalid 'download'");

            println!("{} {:?} ...", "Downloading".green(), name);
            duct_sh::sh_dangerous(download_cmd).run().unwrap();

            let old_wd = env::current_dir().unwrap();
            env::set_current_dir(package_directory).unwrap();

            run_commands();

            env::set_current_dir(old_wd).unwrap();
        }

        println!();
    });
}
