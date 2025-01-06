use csv::Writer;
use open;
use std::env;
use std::error::Error;
use std::io;
use std::path::Path;
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

    let csv_file = format!("{}.csv", file_extension);

    let mut wtr = Writer::from_path(&csv_file.clone())?;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if (!p.is_file()) {
            continue;
        }

        if p.extension().and_then(|s| s.to_str()).unwrap_or("") == file_extension {
            wtr.write_record(&[p.as_os_str().as_encoded_bytes()])?;
        }
    }

    wtr.flush()?;

    println!("View and edit file '{}' [Y/n]? ", csv_file);

    let mut yes_no = String::new();

    io::stdin()
        .read_line(&mut yes_no)
        .expect("Failed to read line");

    match yes_no.trim() {
        "Y" | "y" | "" => match open::that(csv_file.clone()) {
            Ok(()) => println!("Opened '{}' successfully.", csv_file),
            Err(err) => {
                eprintln!("An error occurred when opening '{}': {}", csv_file, err);
                if let Some(1) = err.raw_os_error() {
                    eprintln!("Launcher failed with ExitStatus(1). Trying to open application selector...");
                    match open::that_with("open", &csv_file) {
                        Ok(()) => println!("Opened '{}' with application selector successfully.", csv_file),
                        Err(err) => eprintln!("An error occurred when opening '{}' with application selector: {}", csv_file, err),
                    }
                }
            },
        },
        "N" | "n" => {
            println!("File not opened.");
        }
        _ => {
            println!("Invalid input. File not opened.");
        }
    }

    Ok(())
}
