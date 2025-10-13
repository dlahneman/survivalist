use colorize::AnsiColor;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

// the entries in our todo list are stored in the ToDo struct.
#[derive(Serialize, Deserialize)]
struct ListEntry {
    number: u32,
    priority: u32,
    entry_text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vector of structs that is our dynamic to do list.

    let existing_to_do_list = read_from_json();
    //.expect("must have a 'to_do_list.json in root directory");
    let mut to_do_list = match existing_to_do_list {
        Ok(list) => list,
        Err(_) => Vec::new(),
    };

    //let mut to_do_list: Vec<ListEntry> = Vec::new();

    println!("Hello! Welcome to your To-Do list!");
    loop {
        println!("\ttype: 'l' to view current list");
        println!("\ttype: 'a' to add new item");
        println!("\ttype: 's' to save list");
        println!("\ttype: 'exit' to exit");

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
    Ok(())
}

fn create_entry(list: &Vec<ListEntry>, input: &str) -> ListEntry {
    println!("you entered {input}, Please write your list entry");
    let mut list_item = String::new();
    io::stdin()
        .read_line(&mut list_item)
        .expect("Entry was not formatted");

    let list_entry = ListEntry {
        number: list.len() as u32,
        priority: 1,
        entry_text: list_item,
    };
    return list_entry;
}

fn print_list(to_do_list: &Vec<ListEntry>) {
    println!("To-Do List:");
    println!(
        "{}\t  {}\t{}",
        "Item #:".bold(),
        "Priority:".bold(),
        "Task Description:".bold()
    );
    for item in to_do_list {
        let (element_number, element_priority, element_task) =
            (item.number, item.priority, item.entry_text.clone());
        println!(
            "  --{0}\t\t{1}\t{2}",
            element_number, element_priority, element_task
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

fn read_from_json() -> Result<Vec<ListEntry>, Box<dyn std::error::Error>> {
    // read the json file and places that into the Vec<ListEntry>.
    let data = fs::read_to_string("to_do_list.json").expect("json formatted incorrectly");
    let my_list_json: Vec<ListEntry> = serde_json::from_str(&data)?;
    return Ok(my_list_json);
}

fn delete_entry(list: &mut Vec<ListEntry>) {
    println!("What item # do you want to delete?");
    let mut delete_item = String::new();
    io::stdin()
        .read_line(&mut delete_item)
        .expect("Entry not formatted correct.");

    let item_str: &str = &delete_item;
    let item_number: u32 = item_str.trim().parse().expect("must be a number");

    if list.len() < 1 {
        list.clear();
    } else {
        list.remove(item_number as usize);
    };
}
