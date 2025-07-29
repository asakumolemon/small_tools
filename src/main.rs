use small_tools::chat_mod::chat;
use small_tools::todo_mod::todo_list;
use std::io::stdin;


fn main() {
    let mut input = String::new();
    println!("用法:");
    println!("  1 或 todo_list - 进入待办事项列表");
    println!("  2 或 chat - 进入聊天模式");
    stdin().read_line(&mut input).expect("读取输入失败");
    let choice = Menu::form_handler(&input);
    match choice {
        Menu::TODO => {
            todo_list::todo_run();
        },
        Menu::CHAT => {
            chat::chat_run();
        },
        Menu::SHOW => {
            println!("用法:");
            println!("  1 或 todo_list - 进入待办事项列表");
            println!("  2 或 chat - 进入聊天模式");
        }
    }
}

enum Menu {
    TODO,
    CHAT,
    SHOW
}

impl Menu {
    fn form_handler(str: &String) -> Menu{
        match str.trim().to_lowercase().as_str() {
            "1"|"todo_list" => {
                Menu::TODO
            },
            "2"|"chat" => {
                Menu::CHAT
            },
            _ => {
                Menu::SHOW
            }
        }
    }
}