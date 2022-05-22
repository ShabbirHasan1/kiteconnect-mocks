use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
