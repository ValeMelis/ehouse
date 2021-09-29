use chrono::prelude::*;
use std::env;
use easy_reader::EasyReader;
use std::io::Write;
use std::io::Read;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let command: String = args[1].parse().unwrap();

        if args.len() >=3 {
            let current_file: String = args[2].parse().unwrap();

            let mut directory: String = args[2].parse().unwrap();
            directory.replace("_"," ").split_whitespace().nth(0);

            let filename: String = String::from(format!("/.ehouse/{}/{}.txt",directory,current_file));
            
            if args.len() == 4 {
                let movements: isize = args[3].parse().unwrap();
                let date = Local::today();

                let b = std::path::Path::new(&filename).exists();
                let mut file;

                if b!=true {
                    file =  std::fs::File::create(filename.as_str()).expect("create failed");
                }

                let mut last_line = EasyReader::new(file).expect("read failed");
                last_line.build_index();
                last_line.eof();
                let last_line = last_line.prev_line().expect("read failed").unwrap().chars().nth(0);
                let n: isize = last_line.unwrap().to_string().parse().unwrap();

                file.write_all(format!("{}) {} {}", n, movements, date).as_bytes()).expect("write failed");

            }
        }
    }
}
