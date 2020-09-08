use glob::glob;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

struct Delta {
	old: PathBuf,
	new: PathBuf,
}

//From https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file.
const MS_RESERVED_STRINGS: [(&str, &str); 9] = [
	("<", "﹤"),
	(">", "﹥"),
	(":", "ː"),
	("\"", "“"),
	("/", "⁄"),
	("\\", "∖"),
	("|", "⼁"),
	("?", "﹖"),
	("*", "﹡"),
];

//As above, but the minimum I needed for my files/filesystem/driver. Most
//notably, "<" and ">" seem to work fine, so there's no sense mangling them.
const FUSEBLK_APPARENT_RESERVED_STRINGS: [(&str, &str); 4] =
	[(":", "ː"), ("?", "﹖"), ("|", "⼁"), ("/", "⁄")];

//Twitter and co. have a bad habit of extending filenames a bit. This breaks
//opening the files.
const BAD_EXTENSIONS: [(&str, &str); 8] = [
	(".jpg:large", ".jpg"),
	(".jpg_large", ".jpg"),
	(".jpg:small", ".jpg"),
	(".jpg_small", ".jpg"),
	(".jpeg.jpg", ".jpg"),
	(".jpg.jpg", ".jpg"),
	(".jpeg.jpeg", ".jpeg"),
	(".jpg.jpeg", ".jpeg"),
];

const HELPTEXT: &str = "\
Usage: fuseblk-filename-fixer [-f or -m] [-e] [-u] [path]

Defaults: --minimal --extensions ~/Pictures/collections

Options:
\t--full | -f: Replace all Microsoft reserved strings:
\t\t<>:\"/\\|?*
\t--minimal | -m: Replace a minimal set to make ext4 fuseblk work:
\t\t:?|/
\t--extensions | -e: Fix Twitter's mangled file extensions.
\t--use-replacement-char | -r: Use � instead of a Unicode homoglyph.
";

fn main() {
	let mut input_buf = [0];
	let mut stdin = io::stdin();

	let mut args: env::Args = env::args();
	args.next().unwrap(); //Ignore filepath.

	//Options
	let mut source_dir = "".to_string();
	let mut use_ms_reserved = false;
	let mut use_fuseblk_apparent_reserved = false;
	let mut use_bad_extensions = false;
	let mut use_replacement_char = false;

	while let Some(arg) = args.next() {
		match &*arg {
			"help" | "--help" | "-h" => {
				print!("{}", HELPTEXT);
				process::exit(0);
			}
			"--full" | "-f" => use_ms_reserved = true,
			"--minimal" | "-m" => use_fuseblk_apparent_reserved = true,
			"--extensions" | "-e" => use_bad_extensions = true,
			"--use-replacement-char" | "-r" => use_replacement_char = true,
			_ if source_dir == "" => source_dir = arg,
			_ => {
				println!("Unknown arg {:?} - try --help.", arg);
				process::exit(1);
			}
		}
	}

	//Default to fixing extensions and being as non-destructive as possible.
	if !(use_ms_reserved
		|| use_fuseblk_apparent_reserved
		|| use_bad_extensions
		|| use_replacement_char)
	{
		use_fuseblk_apparent_reserved = true;
		use_bad_extensions = true;
	}

	//If --minimal has been specified, --full will make it not work. Complain.
	if use_ms_reserved && use_fuseblk_apparent_reserved {
		println!("-f or --full precludes the use of the -m or --minimal options for safety.");
		process::exit(1);
	}

	//Default to the path I usually call this on.
	if source_dir == "" {
		source_dir = env::var("HOME")
			.expect("$HOME not set. Please specify a directory.")
			.to_owned() + "/Pictures/collections";
	}

	eprintln!("source_dir: {:?}", source_dir);
	eprintln!("use_ms_reserved: {:?}", use_ms_reserved);
	eprintln!(
		"use_fuseblk_apparent_reserved: {:?}",
		use_fuseblk_apparent_reserved
	);
	eprintln!("use_bad_extensions: {:?}", use_bad_extensions);
	eprintln!("use_replacement_char): {:?}", use_replacement_char);

	println!("Scanning for files to fix…");

	let mut pattern = PathBuf::from(&source_dir);
	pattern.push("**/*");
	let pattern = pattern.to_str().expect("Invalid utf8.");
	let mut rename_queue = Vec::new();
	for entry in glob(pattern).expect("Failed to read glob pattern") {
		match entry {
			Err(e) => println!("i/o error: {:?}", e),
			Ok(old_path) => {
				let mut new_path = old_path.clone();
				let mut new_file_name = String::from(
					//COW strings don't work with set_file_name.
					old_path
						.file_name()
						.unwrap() //This will always work because glob will not hand us an invalid path.
						.to_string_lossy(),
				);

				if use_bad_extensions {
					for (pat, sub) in &BAD_EXTENSIONS {
						//.iter() also works here.
						new_file_name = new_file_name.replace(pat, sub);
					}
				}

				if use_ms_reserved {
					for (pat, sub) in &MS_RESERVED_STRINGS {
						new_file_name = new_file_name
							.replace(pat, if !use_replacement_char { sub } else { "�" });
					}
				} else if use_fuseblk_apparent_reserved {
					for (pat, sub) in &FUSEBLK_APPARENT_RESERVED_STRINGS {
						new_file_name = new_file_name
							.replace(pat, if !use_replacement_char { sub } else { "�" });
					}
				} else {
					unreachable!("These are mutually exclusive for safety. This is checked earlier, near where args are read.");
				}

				new_path.set_file_name(new_file_name);

				if new_path != old_path {
					println!("{:?} →\n{:?}\n", old_path, new_path);
					rename_queue.push(Delta {
						old: old_path,
						new: new_path,
					});
				}
			}
		}
	}

	let num_files: usize = rename_queue.len();
	if num_files == 0 {
		println!("Nothing to do.");
		process::exit(0);
	}

	println!("\nClean {:?} files? (Y/n/Yes/no)", num_files);
	stdin.read(&mut input_buf).expect("i/o error");
	let input = (input_buf[0] as char).to_lowercase().next().unwrap();
	if input == 'n' {
		process::exit(1);
	}
	/* Not needed because no further prompts now. How do we just read char-by-char?
	//Drain buffer to end of line.
	while input != '\n' {
		stdin.read(&mut input_buf).expect("i/o error");
		input = (input_buf[0] as char).to_lowercase().next().unwrap();
	}
	*/

	let mut num_renamed: usize = 0;
	for delta in rename_queue.drain(..) {
		match fs::rename(delta.old, delta.new) {
			Ok(()) => num_renamed += 1,
			Err(e) => println!("Rename failed: {:?}", e),
		}
		print!("\rCleaning {:?}/{:?} ", num_renamed, num_files);
	}

	println!("\nDone.");
}
