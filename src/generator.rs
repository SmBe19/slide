use std::path::Path;
use std::error::Error;
use std::fs;

pub fn generate(file: &Path, template_path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(())
}
