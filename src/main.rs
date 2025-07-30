use small_tools::chat_mod::chat;
use small_tools::todo_mod::todo_list;
use std::io::stdin;


fn show_menu() {
    println!("╔════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              欢迎使用多功能工具                                            ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════════════════╣");
    println!("║  选项  │ 功能说明                                                                          ║");
    println!("║────────┼───────────────────────────────────────────────────────────────────────────────────║");
    println!("║   1    │ 待办事项列表 (todo_list)                                                          ║");
    println!("║   2    │ 聊天模式 (chat)                                                                   ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════════════════╝");
    println!();
    println!("请选择功能（输入数字或命令）：");
}

fn main() {
    let mut input = String::new();
    
    show_menu();
    
    loop {
        input.clear();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let choice = Menu::form_handler(&input);
                match choice {
                    Menu::TODO => {
                        todo_list::todo_run();
                    },
                    Menu::CHAT => {
                        chat::chat_run();
                    },
                    Menu::SHOW => {
                        show_menu();
                    }
                }
            },
            Err(_) => {
                println!("读取输入失败，请重试：");
            }
        }
        show_menu();

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