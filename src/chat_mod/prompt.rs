use crate::chat_mod::chat::Message;

struct Prompt {
    role: String,
    content: String,
}

enum Menu {
    ADD,
    EDIT,
    DELETE,
    BACK
}

impl Menu {
    fn form_handler(str: &String) -> Self{
        match str.trim().to_lowercase().as_str() {
            "1"|"add" => Menu::ADD,
            "2"|"edit" => Menu::EDIT,
            "3"|"delete" => Menu::DELETE,
            _ => Menu::BACK
        }
    }
}

fn prompt(chara: &mut Message) {

}