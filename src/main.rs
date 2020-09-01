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

fn main() {
    let mut input_buf = [0];
    let mut stdin = io::stdin();

    let mut args: env::Args = env::args();
    args.next().unwrap(); //Ignore filepath.
    let source_dir = args
        .next()
        .unwrap_or(env::var("HOME").unwrap().to_owned() + "/Pictures/collections");
    let source_dir = PathBuf::from(&source_dir);
    
    println!("Scanning for files to fix…");
    
    let mut pattern = source_dir.clone();
    pattern.push("**/*");
    let pattern = pattern.to_str().expect("Invalid utf8.");
    let mut rename_queue = Vec::new();
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Err(e) => println!("i/o error: {:?}", e),
            Ok(old_path) => {
                let mut new_path = old_path.clone();
                new_path.set_file_name(
                    old_path
                        .file_name()
                        .expect("path cannot be .. or /")
                        .to_str()
                        .expect("Invalid utf8.")
                        .replace(".jpg:large", ".jpg") //Fix extention problems from Twitter.
                        .replace(".jpg_large", ".jpg")
                        .replace(".jpg:small", ".jpg")
                        .replace(".jpg_small", ".jpg")
                        .replace(".jpeg.jpg", ".jpg")
                        .replace(".jpg.jpg", ".jpg")
                        .replace(".jpeg.jpeg", ".jpeg")
                        .replace(".jpg.jpeg", ".jpeg")
                        .replace(":", "ː") //Substitue out special characters after fixing jpegs.
                        .replace("?", "﹖")
                        .replace("|", "⼁")
                        .replace("/", "⁄")
                );
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
