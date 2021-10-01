//git push -u origin main
//-add 2021_09 +200

//import of the external crates
use chrono::prelude::*;
use std::env;
use easy_reader::EasyReader;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    //Importing arguments from the cli
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        //assigning a variable dor the command
        let command: String = args[1].parse().unwrap();

        if args.len() >=3 {
            //saving the name of the file in input
            let current_file: String = args[2].parse().unwrap();

            //current directory
            let directory: String = args[2].parse().unwrap();
            directory.replace("_"," ").split_whitespace().nth(0);

            //name of the file with the right extension
            let filename: String = String::from(format!("/.ehouse/{}/{}.txt",directory,current_file));
            
            if args.len() == 4 {
                //implementation of the add command
                if command == "add" {
                    //movements of the money
                    let movements: isize = args[3].parse().unwrap();
                    //current date utc
                    let date = Utc::today();

                    //boolean for knowing if the file already exists or not
                    let b = std::path::Path::new(&filename).exists();

                    if b!=true {
                        std::fs::create_dir_all(format!("/.ehouse/{}",directory)).expect("error creating the dir");
                        //creating the file if not already existing and writing the first line

                        let mut file1 = std::fs::File::create(&filename.as_str()).expect("create failed");

                        file1.write_all(format!("1) {} {}\n",movements, date).as_bytes()).expect("write failed");

                        println!("File created");

                    } else {
                        //opening the file
                        let mut file = OpenOptions::new()
                            .append(true)
                            .read(true)
                            .write(true)
                            .create(true)
                            .open(&filename)
                            .expect("cannot open file");

                        //creating the new line
                        let mut last_line = EasyReader::new(&file).expect("read failed");
                        //last_line.build_index();
                        last_line.eof();
                        let last_line = last_line.prev_line().expect("read failed").unwrap().chars().nth(0);
                        let mut n: isize = last_line.unwrap().to_string().parse().unwrap();
                        n = n+1;

                        file.write_all(format!("{}) {} {}\n", n, movements, date).as_bytes()).expect("write failed");

                        println!("Movements added");
                    }
                }
            }
        }
    }
}
