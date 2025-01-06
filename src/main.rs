use std::env;
use std::error::Error;
use std::path::Path;
use csv::Writer;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let prog_bin = Path::new(&args[0])
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    if args.len() < 3 {
        println!("Usage: {} [FILE EXTENSION] [PATH]", prog_bin);
        return Ok(());
    }

    let file_extension = &args[1];
    let path = Path::new(&args[2]);

    if !path.exists() {
        eprintln!("Error: The path '{}' does not exist.", path.display());
        return Ok(());
    }

    let mut wtr = Writer::from_path("mp3.csv")?;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }

        if p.extension().and_then(|s| s.to_str()).unwrap_or("") == file_extension {
            wtr.write_record(&[p.as_os_str().as_encoded_bytes()])?;
        }
    }

    wtr.flush()?;

    Ok(())
}
