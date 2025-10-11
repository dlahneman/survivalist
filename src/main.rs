use std::io;

// the entries in our todo list are stored in the ToDo struct.
struct ToDo {
    priority: u32,
    entry_text: String,
}

fn main() {
    // Vector of structs that is our dynamic to do list.
    let mut to_do_list: Vec<ToDo> = Vec::new();

    println!("Hello! Welcome to your To-Do list!");
    println!("type: l to view current list, a to add new item.");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Enter a command listed above.");

    // need to match to the input but the input is a String and
    // we match to a &str so we convert and trim off any /n... etc.
    let command_char = command.as_str().trim();
    if command_char == "a" {
        let new_item = create_entry();
    };
}

fn create_entry() -> ToDo {
    println!("you entered l, Please write your list entry");
    let mut list_item = String::new();

    io::stdin()
        .read_line(&mut list_item)
        .expect("Entry was not formatted");

    let list_item = ToDo {
        priority: 1,
        entry_text: list_item,
    };
    return list_item;
}
