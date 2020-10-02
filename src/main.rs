use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let index = if cfg!(debug_assertions) { 1 } else { 1 };
    let args: Vec<String> = env::args().collect();
    let id = &args[index].to_uppercase();

    let license_name = license::from_id(id).unwrap().name();

    if std::path::Path::new("LICENSE").exists() {
        println!("LICENSE file already exists. Please remove it to continue.");
        return Ok(());
    } else {
        println!(
            "Creating LICENSE file with the following license: {}",
            license_name
        );
        create_file(create_contents(id))?;
        return Ok(());
    }
}

fn create_file(contents: String) -> std::io::Result<()> {
    let mut file = File::create("LICENSE")?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

fn create_contents(id: &str) -> String {
    let license = license::from_id(id).unwrap();

    return String::from(license.text());
}
