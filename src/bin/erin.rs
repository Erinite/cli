
use colored::*;
use rust_embed::RustEmbed;
use clap::{App, load_yaml, crate_authors, crate_version};
use std::fs::{File, DirBuilder};
use std::io::prelude::*;
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
    let erinite = Erinite { project: Project { name: project_name.to_string(), features: Vec::new()}};
    let toml = toml::to_string(&erinite).unwrap();
    println!("{}", toml);
    println!("{}", template("test.clj").render(&globals).unwrap());
    DirBuilder::new()
        .recursive(true)
        .create(project_name).unwrap();
    let mut file = File::create(format!("{}/erinite.toml", project_name)).unwrap();
    file.write_all(toml.as_bytes()).unwrap();
    output("Project created", format!("{}/", project_name).as_str());
}

fn read_project_file () -> Erinite {
    let mut file = File::open("erinite.toml").expect("Could not open erinite.toml");
    let mut erinite = String::new();
    file.read_to_string(&mut erinite).expect("Could not read erinite.toml");
    toml::from_str(erinite.as_str()).expect("Could not read erinite.toml")
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
