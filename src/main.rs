use csv::Writer;
use open;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

fn process_files(path: &Path, file_extension: &str) -> Result<String, Box<dyn Error>> {
    let mut filenames = HashSet::new();
    let csv_file = format!("{}.csv", file_extension);
    let mut wtr = Writer::from_path(&csv_file)?;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }

        if p.extension().and_then(|s| s.to_str()).unwrap_or("") == file_extension {
            let filename = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let mut duplicated = "";
            if !filenames.insert(filename.to_string()) {
                duplicated = "duplicated";
            }
            wtr.write_record(&[p.as_os_str().as_encoded_bytes(), duplicated.as_bytes()])?;
        }
    }

    wtr.flush()?;
    Ok(csv_file)
}

fn prompt_open_file(csv_file: &str) -> Result<(), Box<dyn Error>> {
    println!("View and edit file '{}' [Y/n]? ", csv_file);

    let mut yes_no = String::new();
    io::stdin().read_line(&mut yes_no).expect("Failed to read line");

    match yes_no.trim() {
        "Y" | "y" | "" => {
            match open::that(csv_file) {
                Ok(()) => println!("Opened '{}' successfully.", csv_file),
                Err(err) => {
                    eprintln!("An error occurred when opening '{}': {}", csv_file, err);
                    eprintln!("Launcher failed. Do you want to open the application selector? [Y/n]");

                    let mut selector_yes_no = String::new();
                    io::stdin().read_line(&mut selector_yes_no).expect("Failed to read line");

                    match selector_yes_no.trim() {
                        "Y" | "y" | "" => {
                            match open::with(csv_file, "start") {
                                Ok(()) => println!("Opened '{}' with application selector successfully.", csv_file),
                                Err(err) => eprintln!("An error occurred when opening '{}' with application selector: {}", csv_file, err),
                            }
                        }
                        _ => println!("Application selector not opened."),
                    }
                }
            };
        }
        "N" | "n" => println!("File '{}' not opened.", csv_file),
        _ => println!("Invalid input. File {} not opened.", csv_file),
    }

    Ok(())
}

fn prompt_continue() -> bool {
    println!("Do you want to continue? [Y/n]");
    let mut continue_yes_no = String::new();
    io::stdin().read_line(&mut continue_yes_no).expect("Failed to read line");

    match continue_yes_no.trim() {
        "Y" | "y" | "" => true,
        _ => false,
    }
}

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

    let csv_file = process_files(path, file_extension)?;
    prompt_open_file(&csv_file)?;

    if prompt_continue() {
        println!("Continuing...");
    } else {
        println!("Exiting...");
    }

    Ok(())
}
