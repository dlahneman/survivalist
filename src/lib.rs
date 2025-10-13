use colorize::AnsiColor;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

// the entries in our todo list are stored in the ToDo struct.
#[derive(Serialize, Deserialize)]
pub struct ListEntry {
    number: u32,
    priority: u32,
    entry_text: String,
}

pub fn create_entry(list: &Vec<ListEntry>, input: &str) -> ListEntry {
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

pub fn print_list(to_do_list: &Vec<ListEntry>) {
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

pub fn save_to_json(to_do_list: &Vec<ListEntry>) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize the vector to a JSON string
    let json_string = serde_json::to_string_pretty(&to_do_list)?;
    // Save the JSON string to a file
    fs::write("to_do_list.json", json_string)?;
    println!("List saved");
    Ok(())
}

pub fn read_from_json() -> Result<Vec<ListEntry>, Box<dyn std::error::Error>> {
    // read the json file and places that into the Vec<ListEntry>.
    let data = fs::read_to_string("to_do_list.json").expect("json formatted incorrectly");
    let my_list_json: Vec<ListEntry> = serde_json::from_str(&data)?;
    return Ok(my_list_json);
}

pub fn delete_entry(list: &mut Vec<ListEntry>) {
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
        item_renumber(list);
    };
}

fn item_renumber(list: &mut Vec<ListEntry>) -> &mut Vec<ListEntry> {
    if list.len() >= 1 {
        for item in 0..list.len() {
            list[item].number = item as u32;
        }
        return list;
    } else {
        return list;
    }
}
