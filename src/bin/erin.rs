
use colored::*;
use rust_embed::RustEmbed;
use clap::{App, load_yaml, crate_authors, crate_version};
use std::fs::{self, File, DirBuilder};
use std::io::prelude::*;
use std::process;
use std::error::Error;
use serde::{Serialize, Deserialize};


#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

#[derive(Serialize)]
#[derive(Deserialize)]
struct Erinite {
    project: Project,
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct Project {
    name: String,
    features: Vec<String>,
}

fn terminate_error (message: &str, error: &dyn Error) -> ! {
    eprintln!("{}: {:?}", message.red().bold(), error.to_string());
    process::exit(1);
}
fn terminate (message: &str) -> ! {
    eprintln!("{}.", message.red().bold());
    process::exit(1);
}

fn template (filename: &str) -> liquid::Template {
    let liquid_filename = format!("{}.liquid", filename);
    let file = Templates::get(&liquid_filename).unwrap();
    let content = std::str::from_utf8(file.as_ref());
    let template = liquid::ParserBuilder::with_stdlib()
        .build().unwrap()
        .parse(content.unwrap()).unwrap();
    template
}

fn output (label: &str, message: &str) {
    println!("{}", format!("{}: {}", label.green(), message.blue()));
}

fn create_project (project_name: &str) {
    output("Creating new project", project_name);
            
    let globals = liquid::object!({
        "file": {
            "namespace": "foo.bar",
            "name": project_name,
        }
    });
    println!("{}", template("test.clj").render(&globals).unwrap());
    let path = "";
    DirBuilder::new()
        .recursive(true)
        .create(path).unwrap();
    output("Project created", project_name);
}

fn read_project_file () -> Erinite {
    let mut file = File::open("erinite.toml").unwrap_or_else(|error|{terminate_error("Could not open erinite.toml", &error)});
    let mut erinite = String::new();
    file.read_to_string(&mut erinite).unwrap_or_else(|error|{terminate_error("Could not read erinite.toml", &error)});
    toml::from_str(erinite.as_str()).unwrap_or_else(|error|{terminate_error("Could not read erinite.toml", &error)})
}

fn add_feature (feature_name: &str) {
    let mut erinite = read_project_file();
    erinite.project.features.push(feature_name.to_string());
    for feature in erinite.project.features {
        println!("{}", feature);
    }

    output("Adding new feature", feature_name);
    output("Feature added", feature_name);
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml)
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .get_matches();
    match matches.subcommand() {
        ("new", Some(project)) => create_project(project.value_of("PROJECT").unwrap()),
        ("add", Some(add_command)) => {
            match add_command.subcommand() {
                ("feature", Some(feature)) => add_feature(feature.value_of("FEATURE").unwrap()),
                ("", None) => unreachable!(),
                _ => unreachable!(),
            }
        }
        ("", None) => unreachable!(),
        _ => unreachable!(),
    }
}
