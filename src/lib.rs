//import of the external crates
use chrono::prelude::*;
use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use easy_reader::EasyReader;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

pub fn add(args: &Vec<String>, filen: &str, dir: &str) {
    //movements of the money
    let movements: isize = args[3].parse().unwrap();
    //current date utc
    let date = Local::now().format("%e %b %T");

    //boolean for knowing if the file already exists or not
    let b = std::path::Path::new(&filen).exists();

    if !b {
        std::fs::create_dir_all(format!("/.ehouse/{}", dir)).expect("error creating the dir");
        //creating the file if not already existing and writing the first line

        let mut file1 = std::fs::File::create(&filen).expect("create failed");

        file1
            .write_all(format!("1) {}{}\n", movements, date).as_bytes())
            .expect("write failed");

        if movements>=0 {
            println!("1) {}{}",Green.paint(movements.to_string()), date);
        } else {
            println!("1) {}{}",Red.paint(movements.to_string()), date);
        }
        

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
        let last_line = last_line
            .prev_line()
            .expect("read failed")
            .unwrap()
            .chars()
            .nth(0);

        let mut n: isize = last_line.unwrap().to_string().parse().unwrap();
        n += 1;

        file.write_all(format!("{}) {}{}\n", n, movements, date).as_bytes())
            .expect("write failed");

        if movements>=0 {
            println!("{}) {}{}",n,Green.paint(movements.to_string()), date);
        } else {
            println!("{}) {}{}",n,Red.paint(movements.to_string()), date);
        }

        println!("Movements added");
    }
}

pub fn view(filen: &str) {

    //let movements: isize = args[3].parse().unwrap();
    let mut file = std::fs::File::open(filen).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut n;
    let mut movements:isize;
    let mut day;
    let mut month;
    let mut time;

    for lines in contents.split("\n") {
        if lines.split_whitespace().nth(0).is_some() {
            n=lines.split_whitespace().nth(0).unwrap();
            movements=lines.split_whitespace().nth(1).unwrap().parse().unwrap();
            day=lines.split_whitespace().nth(2).unwrap();
            month=lines.split_whitespace().nth(3).unwrap();
            time=lines.split_whitespace().nth(4).unwrap();

            if movements>=0 {
                println!("{} {} {} {} {}",n,Green.paint(movements.to_string()), day,month,time);
            } else {
                println!("{} {} {} {} {}",n,Red.paint(movements.to_string()), day,month,time);
            }

        }
    }
}