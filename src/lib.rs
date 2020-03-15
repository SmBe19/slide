use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use clap::{App, Arg, ArgMatches, crate_authors, crate_description, crate_version, SubCommand};

mod util;
mod generator;
mod ty;
mod codegen;
mod plugin;
mod tester;
mod errors;

pub fn run(args: ArgMatches) -> Result<(), Box<dyn Error>> {
    let input_file = match args.value_of("INPUT") {
        Some(path) => path,
        None => panic!("No path given"),
    };
    let input_file = shellexpand::tilde(input_file);
    let input_file = Path::new(input_file.as_ref());

    let template_path = env::var("SLIDE_TEMPLATE_PATH").unwrap_or(String::from("~/.local/share/slide/template/"));
    let template_path = shellexpand::tilde(&template_path);
    let template_path = Path::new(template_path.as_ref());

    match args.subcommand() {
        ("init", Some(sub_args)) => cmd_init(input_file, template_path, sub_args),
        ("gen", Some(_)) => cmd_gen(input_file, template_path),
        ("compile", Some(_)) => cmd_compile(input_file),
        ("check", Some(sub_args)) => cmd_check(input_file, sub_args),
        ("", None) => cmd_full_auto(input_file, template_path),
        _ => Ok(())
    }
}

pub fn parse_arguments() -> ArgMatches<'static> {
    let input_arg = Arg::with_name("INPUT")
        .help("File to operate on")
        .required(true)
        .index(1);
    let template_arg = Arg::with_name("template")
        .help("Template to use")
        .short("t")
        .long("template")
        .takes_value(true);
    let print_failures = Arg::with_name("fail")
        .help("Activate printing failed output")
        .long("fail");
    let check_pattern = Arg::with_name("pattern")
        .help("Only check tests with the name starting with the pattern")
        .long("only")
        .short("o")
        .takes_value(true);
    let subcommand_init = SubCommand::with_name("init")
        .about("Initialize a new template")
        .arg(template_arg);
    let subcommand_gen = SubCommand::with_name("gen")
        .about("Generate code based on the config in the file");
    let subcommand_compile = SubCommand::with_name("compile")
        .about("Compile the code");
    let subcommand_check = SubCommand::with_name("check")
        .about("Run the code on the provided samples")
        .arg(print_failures)
        .arg(check_pattern);
    let matches = App::new("slide")
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(input_arg)
        .subcommand(subcommand_init)
        .subcommand(subcommand_gen)
        .subcommand(subcommand_compile)
        .subcommand(subcommand_check)
        .get_matches();
    matches
}

pub fn cmd_full_auto(path: &Path, template_path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        let template_file = template_path.join("template.cpp");
        fs::copy(template_file, path)?;
    }
    generator::generate(path, template_path)?;
    tester::run_tester(path, None,false)?;
    Ok(())
}

pub fn cmd_init(path: &Path, template_path: &Path, sub_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "File already exists"
        ).into());
    }
    let template_file = template_path.join(
        sub_args.value_of("template")
            .map(|template| format!("template-{}.cpp", template))
            .unwrap_or(String::from("template.cpp"))
    );
    if !template_file.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Template {} does not exist", template_file.to_str().unwrap_or("???"))
        ).into());
    }
    fs::copy(template_file, path)?;
    generator::generate(path, template_path)
}

pub fn cmd_gen(path: &Path, template_path: &Path) -> Result<(), Box<dyn Error>> {
    generator::generate(path, template_path)
}

pub fn cmd_compile(path: &Path) -> Result<(), Box<dyn Error>> {
    tester::compile(path)?;
    Ok(())
}

pub fn cmd_check(path: &Path, sub_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    tester::run_tester(path, sub_args.value_of("pattern"), sub_args.is_present("fail"))
}
