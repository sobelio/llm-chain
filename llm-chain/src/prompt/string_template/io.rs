use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::StringTemplate;

/// Reads a prompt template from a file.
// XXX: Don't leak
pub fn read_prompt_template_file<P: AsRef<Path>>(
    path: P,
) -> Result<StringTemplate, std::io::Error> {
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(StringTemplate::tera(contents))
}
