use chrono::prelude::*;
use std::env;
use easy_reader::EasyReader;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let command: String = args[1].parse().unwrap();

        if args.len() >=3 {
            let current_file: String = args[2].parse().unwrap();

            let directory: String = args[2].parse().unwrap();
            directory.replace("_"," ").split_whitespace().nth(0);

            let filename: String = String::from(format!("{}.txt",current_file));
            
            if args.len() == 4 {
                if command == "add" {
                    let movements: isize = args[3].parse().unwrap();
                    let date = Utc::today();

                    let b = std::path::Path::new(&filename).exists();

                    if b!=true {
                      let mut file1 = std::fs::File::create(&filename.as_str()).expect("create failed");

                      file1.write_all(format!("1) {} {}\n",movements, date).as_bytes()).expect("write failed");

                    }

                    let mut file = OpenOptions::new()
                        .append(true)
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&filename)
                        .expect("cannot open file");

                    let mut last_line = EasyReader::new(&file).expect("read failed");
                    //last_line.build_index();
                    last_line.eof();
                    let last_line = last_line.prev_line().expect("read failed").unwrap().chars().nth(0);
                    let mut n: isize = last_line.unwrap().to_string().parse().unwrap();
                    n = n+1;

                    file.write_all(format!("{}) {} {}\n", n, movements, date).as_bytes()).expect("write failed");


                }
                
            }
        }
    }
}
