use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use anyhow::Result;
use clap::builder::Str;
use clap::Parser;
use convert_case::{Case, Casing};
use generators::common::{make_cargo_toml, make_lib_rs};
use serde::Serialize;
use types::IDL;
use zip::write::FileOptions;
use zip::ZipWriter;
use crate::generators::readme::generate_readme;

mod types;
mod generators;

#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Package {
    /// A single file
    File,
    /// A production-ready crate
    #[default]
    Crate,

    Zip,
}

impl ToString for Package {
    fn to_string(&self) -> String {
        match self {
            Package::File => "file".to_string(),
            Package::Crate => "crate".to_string(),
            Package::Zip => "zip".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename of IDL file
    #[arg(short, long)]
    filename: String,
    /// Type of Package to generate (Crate or File)
    #[clap(short, long, default_value_t, value_enum)]
    package: Package,

    #[arg(short, long)]
    out: String,
}

fn make(idl: &IDL, package: &Package, out_dir: &String) -> Result<()> {
    match package {
        Package::File => {
            let mut file = File::create(format!("{}.rs", idl.get_name().to_case(Case::Snake)))?;
            file.write_all(make_lib_rs(idl).as_bytes())?;
        },
        Package::Crate => {
            create_dir_all(format!("{}/src", out_dir))?;
            let mut toml_file: File = File::create(format!("{}/Cargo.toml", out_dir))?;
            toml_file.write_all(make_cargo_toml(idl).as_bytes())?;

            let mut readme_file: File = File::create(format!("{}/README.md", out_dir))?;
            readme_file.write_all(generate_readme(idl).as_bytes())?;

            let mut lib_file: File = File::create(format!("{}/src/lib.rs", out_dir))?;
            lib_file.write_all(make_lib_rs(idl).as_bytes())?;
        },
        Package::Zip => {
            let file = File::create(format!("{}.zip", idl.get_name().to_case(Case::Snake)))?;

            let mut zip = ZipWriter::new(file);

            zip.start_file("Cargo.toml", FileOptions::default())?;
            zip.write_all(make_cargo_toml(idl).as_bytes())?;
            zip.add_directory("src", FileOptions::default())?;
            zip.start_file("src/lib.rs", FileOptions::default())?;
            zip.write_all(make_lib_rs(idl).as_bytes())?;
            zip.finish()?;
        },
    }
    println!("✅ Successfully generated {} for {} v{}", package.to_string(), idl.get_name(), idl.get_version());
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(&args.filename)?;
    let reader = BufReader::new(file);

    let idl: IDL = serde_json::from_reader(reader)?;

    make(&idl, &args.package, &args.out)?;
    Ok(())
}
