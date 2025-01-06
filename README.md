# Files Organizer

This project is a simple file organizer written in Rust.
It scans a specified directory for files with a given extension and writes the paths of these files to a CSV file.

## Usage

```sh
files-organizer [FILE EXTENSION] [PATH]
```

- `FILE EXTENSION`: The file extension to search for (e.g., `mp3`).
- `PATH`: The directory path to search in.

## Example

```sh
files-organizer mp3 /
```

This command will search for all `.mp3` files in the `/` directory and write their paths to `mp3.csv`.

## Dependencies

- [walkdir](https://crates.io/crates/walkdir)
- [csv](https://crates.io/crates/csv)

## License

This project is licensed under the MIT License.
