use std::io;

// the entries in our todo list are stored in the ToDo struct.
#[derive(Debug)]
struct ListEntry {
    priority: u32,
    entry_text: String,
}

fn main() {
    // Vector of structs that is our dynamic to do list.
    let mut to_do_list: Vec<ListEntry> = Vec::new();

    println!("Hello! Welcome to your To-Do list!");
    loop {
        println!("!***************************!");
        println!(
            "type: 'l' to view current list\ntype: 'a' to add new item\ntype: 'exit' to exit. "
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
        } else if command_char == "exit" {
            std::process::exit(1);
        }
    }
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
    println!("To-Do List:");
    for item in to_do_list {
        let (element_priority, element_task) = (item.priority, item.entry_text.clone());
        println!("Priority: {0}\tTask: {1}", element_priority, element_task);
    }
}
