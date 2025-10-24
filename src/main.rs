use std::io;
use survivalist::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let existing_to_do_list = read_from_json();
    //.expect("must have a 'to_do_list.json in root directory");
    let mut to_do_list = match existing_to_do_list {
        Ok(list) => list,
        Err(_) => Vec::new(),
    };

    println!("Hello! Welcome to your To-Do list!");
    loop {
        println!(
            "\ttype: 'l' -view list, 'a' - add item, 'd' -delete item, 's' -save, 'exit' -exit"
        );

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Enter a command listed above.");

        // need to match to the input but the input is a String and
        // we match to a &str so we convert and trim off any /n... etc.
        let command_char = command.as_str().trim();
        if command_char == "a" {
            let new_entry = create_entry(&to_do_list, "a");
            to_do_list.push(new_entry);
            print_list(&to_do_list);
        } else if command_char == "l" {
            print_list(&to_do_list);
        } else if command_char == "s" {
            save_to_json(&to_do_list)?;
        } else if command_char == "d" {
            delete_entry(&mut to_do_list);
        } else if command_char == "exit" {
            break;
        }
    }
    Ok(()) // allows the exit to Result type for error propagation.
}
