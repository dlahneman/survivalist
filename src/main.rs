use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

// the entries in our todo list are stored in the ToDo struct.
#[derive(Serialize, Deserialize)]
struct ListEntry {
    priority: u32,
    entry_text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vector of structs that is our dynamic to do list.
    let mut to_do_list: Vec<ListEntry> = Vec::new();

    println!("Hello! Welcome to your To-Do list!");
    loop {
        println!("!***************************!");
        println!(
            "\ttype: 'l' to view current list\n\ttype: 'a' to add new item\n\ttype: 'exit' to exit."
        );
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Enter a command listed above.");

        // need to match to the input but the input is a String and
        // we match to a &str so we convert and trim off any /n... etc.
        let command_char = command.as_str().trim();
        if command_char == "a" {
            let new_entry = create_entry("a");
            to_do_list.push(new_entry);
            print_list(&to_do_list);
        } else if command_char == "l" {
            print_list(&to_do_list);
        } else if command_char == "s" {
            save_to_json(&to_do_list)?;
        } else if command_char == "exit" {
            break;
        }
    }
    Ok(())
}

fn create_entry(input: &str) -> ListEntry {
    println!("you entered {input}, Please write your list entry");
    let mut list_item = String::new();
    io::stdin()
        .read_line(&mut list_item)
        .expect("Entry was not formatted");

    let list_entry = ListEntry {
        priority: 1,
        entry_text: list_item,
    };
    return list_entry;
}

fn print_list(to_do_list: &Vec<ListEntry>) {
    println!("To-Do List:\n");
    for item in to_do_list {
        let (element_priority, element_task) = (item.priority, item.entry_text.clone());
        println!(
            "--\tPriority: {0}\tTask: {1}",
            element_priority, element_task
        );
    }
}

fn save_to_json(to_do_list: &Vec<ListEntry>) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize the vector to a JSON string
    let json_string = serde_json::to_string_pretty(&to_do_list)?;

    // Save the JSON string to a file
    fs::write("to_do_list.json", json_string)?;
    Ok(())
}
