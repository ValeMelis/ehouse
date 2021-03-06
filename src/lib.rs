//import of the external crates
use chrono::prelude::*;
use easy_reader::EasyReader;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;
use whoami::username;
//use colored::*;

pub fn add(args: &Vec<String>) {
    if args.len() == 4 {
        //saving the name of the file in input
        let current_file: String = args[2].parse().unwrap();

        //current directory
        let mut dir: String = args[2].parse().unwrap();
        dir = String::from(dir.replace("_", " ").split_whitespace().nth(0).unwrap());

        //name of the file with the right extension
        let filen: String = format!("/Users/{}/.ehouse/{}/{}.txt",username(), dir, current_file);

        //movements of the money
        let movements: isize = args[3].parse().unwrap();
        //current date utc
        let date = Local::now().format("%e %b %T");

        //boolean for knowing if the file already exists or not
        let b = std::path::Path::new(&filen).exists();

        if !b {
            std::fs::create_dir_all(format!("/Users/{}/.ehouse/{}",username(), dir)).expect("error creating the dir");
            //creating the file if not already existing and writing the first line

            let mut file1 = std::fs::File::create(&filen).expect("create failed");

            file1
                .write_all(format!("1) {} | {}\n", movements, date).as_bytes())
                .expect("write failed");

            if movements>=0 {
                println!("1) {} | {}",movements/*.to_string().green()*/, date);
            } else {
                println!("1) {} | {}",movements/*.to_string().red()*/, date);
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

            file.write_all(format!("{}) {} | {}\n", n, movements, date).as_bytes())
                .expect("write failed");

            if movements>=0 {
                println!("{}) {} | {}",n,movements/*.to_string().green()*/, date);
            } else {
                println!("{}) {} | {}",n,movements/*.to_string().red()*/, date);
            }

            println!("Movements added");
        }    
    } else {
        println!("Number of arguments needed is 3 not {}", args.len()-1);
    }  
    
}

pub fn view(args: &Vec<String>) {
    if args.len() == 3 {
        //saving the name of the file in input
        let current_file: String = args[2].parse().unwrap();

        //current directory
        let mut directory: String = args[2].parse().unwrap();
        directory = String::from(directory.replace("_", " ").split_whitespace().nth(0).unwrap());

        //name of the file with the right extension
        let filen: String = format!("/Users/{}/.ehouse/{}/{}.txt", username(), directory, current_file);

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
                day=lines.split_whitespace().nth(3).unwrap();
                month=lines.split_whitespace().nth(4).unwrap();
                time=lines.split_whitespace().nth(5).unwrap();

                if movements>=0 {
                    println!("{} {} | {} {} {}",n,movements/*.to_string().green()*/, day,month,time);
                } else {
                    println!("{} {} | {} {} {}",n,movements/*.to_string().red()*/, day,month,time);
                }
            }
        }
    }  else {
        println!("Number of arguments needed is 2 not {}", args.len()-1);
    }  
}

pub fn total(args: &Vec<String>) {
    if args.len() == 3 {
        //saving the name of the file in input
        let current_file: String = args[2].parse().unwrap();

        //current directory
        let mut directory: String = args[2].parse().unwrap();
        directory = String::from(directory.replace("_", " ").split_whitespace().nth(0).unwrap());

        //name of the file with the right extension
        let filen: String = format!("/Users/{}/.ehouse/{}/{}.txt", username(), directory, current_file);

        let mut file = std::fs::File::open(filen).expect("File not found");

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut total: isize = 0;
        let mut movements: isize;
        let mut i = 0;

        for lines in contents.split("\n") {
            if lines.split_whitespace().nth(0).is_some() {
                movements=lines.split_whitespace().nth(1).unwrap().parse().unwrap();
            
                total += movements;
                i += 1;
            }
        }

        if i>=1 {
           println!("The total of the movements of {} is: {}",current_file,total);
        } else {
            println!("The file is empty");
        }    
    } else {
        println!("Number of arguments needed is 2 not {}", args.len()-1);
    }
}

pub fn list(args: &Vec<String>) {
    if args.len() == 3 {
        //current directory
        let mut directory: String = args[2].parse().unwrap();
        directory = String::from(directory.replace("_", " ").split_whitespace().nth(0).unwrap());
        println!("All the files in {}:",directory);

        let filen: String = format!("/Users/{}/.ehouse/{}", username(), directory);

        let directory = std::fs::read_dir(filen).expect("File not found");

        if directory.is_dir() {
            for file in directory {
                println!("{}",file.unwrap().path().display().to_string().replace("/", " ").replace("\\"," ").split_whitespace().nth(4).unwrap());
            }
        }
    }
}