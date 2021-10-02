//import of the external crates
use chrono::prelude::*;
use easy_reader::EasyReader;
use std::fs::OpenOptions;
use std::io::Write;


pub fn add(args: &Vec<String>, filen: &String, dir: &String) {
    //movements of the money
    let movements: isize = args[3].parse().unwrap();
    //current date utc
    let date = Utc::today();

    //boolean for knowing if the file already exists or not
    let b = std::path::Path::new(&filen).exists();

    if b!=true {
        std::fs::create_dir_all(format!("/.ehouse/{}",dir)).expect("error creating the dir");
        //creating the file if not already existing and writing the first line

        let mut file1 = std::fs::File::create(&filen.as_str()).expect("create failed");

        file1.write_all(format!("1) {} {}\n",movements, date).as_bytes()).expect("write failed");

        println!("File created");

    } else {
        //opening the file
        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .write(true)
            .create(true)
            .open(&filen)
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