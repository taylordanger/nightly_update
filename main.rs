use std::fs::{File,OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;

fn main() {
        let mut fname = String::new();
        println!("Enter Filename: ");
        std::io::stdin().read_line(&mut fname).expect("Failed to read input MEOW!!");
        let fname = fname.trim();

        let input_path = Path::new(fname);
        let temp_path = Path::new("temp.txt:");

        if !input_path.exists() {
            println!("NOPE no file found stop playing around");
            return;
        }

        {
            let input_file = File::open(&input_path).expect("Failed uhgggg suckfest..cant open that file");
            let mut temp_file = File::create(&temp_path).expect("Failed...nah no temporary file created");

            let reader = BufReader::new(input_file);
            let mut writer = BufWriter::new(&mut temp_file);

            for byte_result in reader.bytes(){
                let byte = byte_result.expect("Failed to read byte");
                writer.write_all(&[byte + 100]).expect("dude no dice encryption failed")
            }

        }

        {
            let mut input_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&input_path)
                .expect("BROOOOOOO no go dude-ette failed to open file for writing");

            let temp_file = File::open(&temp_path).expect("Failed to open temporary file cool cat");
            let reader = BufReader::new(temp_file);

            for byte_result in reader.bytes() {
                    let byte = byte_result.expect("NOOOO YOU FAIL to read encrypted byte");
                    input_file.write_all(&[byte]).expect("Fraileeeedddddd you are so frail ")
            }
        }

        std::fs::remove_file(temp_path).expect("Failed to delete temporary file jeez");
        println!("\nFile {} Encrypted Seuccessfully!", fname);
}