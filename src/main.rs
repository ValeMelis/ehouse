//git push -u origin main
//-add 2021_09 +200
//-view 2021_10
use std::env::args;
mod lib;

fn main() {
    //Importing arguments from the cli
    let args: Vec<String> = args().collect();

    if args.len() >= 2 {
        //assigning a variable dor the command
        let command: String = args[1].parse().unwrap();

        if args.len() >= 3 {
            //saving the name of the file in input
            let current_file: String = args[2].parse().unwrap();

            //current directory
            let mut directory: String = args[2].parse().unwrap();
            directory = String::from(directory.replace("_", " ").split_whitespace().nth(0).unwrap());

            //name of the file with the right extension
            let filename: String =
                format!("/.ehouse/{}/{}.txt", directory, current_file);

            if command == "view" {
                    lib::view(&filename);
            }

            if args.len() == 4 {
                //implementation of the add command
                if command == "add" {
                    lib::add(&args, &filename, &directory);
                } else if command == "remove" {

                }
            }
        }
    }
}
