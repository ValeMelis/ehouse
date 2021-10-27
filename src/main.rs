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

        match command.as_str() {
            "add" => lib::add(&args),
            "view" => lib::view(&args),
            _ => println!("Command not valid\n
                    Enter one of:\n
                    -view to visualize a file\n
                    -add to add movements"),
        }
    }
}
