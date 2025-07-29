use std::io::stdin;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::chat_mod::chat::{App, Message};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Prompt {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PromptList {
    pub prompts: Vec<Prompt>,
}

impl PromptList {
    pub fn new() -> Self {
        Self {
            prompts: Vec::new(),
        }
    }

    pub fn load_from_file() -> Self {
        let path = Path::new("prompts.json");
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(prompts) => prompts,
                        Err(_) => PromptList::new(),
                    }
                }
                Err(_) => PromptList::new(),
            }
        } else {
            PromptList::new()
        }
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("prompts.json", json)?;
        Ok(())
    }

    pub fn add_prompt(&mut self, prompt: Prompt) {
        self.prompts.push(prompt);
    }

    pub fn edit_prompt(&mut self, index: usize, prompt: Prompt) -> bool {
        if index < self.prompts.len() {
            self.prompts[index] = prompt;
            true
        } else {
            false
        }
    }

    pub fn delete_prompt(&mut self, index: usize) -> bool {
        if index < self.prompts.len() {
            self.prompts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_prompts(&self) {
        if self.prompts.is_empty() {
            println!("暂无prompt配置");
            return;
        }

        println!("当前prompt列表:");
        for (i, prompt) in self.prompts.iter().enumerate() {
            println!("{}. Role: {}, Content: {}", i + 1, prompt.role, prompt.content);
        }
    }

    pub fn get_prompt(&self, index: usize) -> Option<&Prompt> {
        if index < self.prompts.len() {
            Some(&self.prompts[index])
        } else {
            None
        }
    }
}

enum Menu {
    ADD,
    EDIT,
    DELETE,
    CHOOSE,
    BACK
}

impl Menu {
    fn form_handler(str: &String) -> Self{
        match str.trim().to_lowercase().as_str() {
            "1"|"add" => Menu::ADD,
            "2"|"edit" => Menu::EDIT,
            "3"|"delete" => Menu::DELETE,
            "4"|"choose" => Menu::CHOOSE,
            _ => {
                Menu::BACK
            }
        }
    }
}

pub fn prompt(app: &mut App) -> bool{
    let mut prompts = PromptList::load_from_file();
    
    println!("请选择操作:");
    println!("1. 添加Prompt (add)");
    println!("2. 编辑Prompt (edit)");
    println!("3. 删除Prompt (delete)");
    println!("4. 查看/选择Prompt (choose)");
    println!("其他. 返回上级菜单");

    let mut input = String::new();

    if let Err(error) = stdin().read_line(&mut input) {
        eprintln!("读取输入失败: {}", error);
        return false;
    }

    let choice = Menu::form_handler(&input);

    match choice {
        Menu::ADD => {
            println!("请输入Role:");
            let mut role = String::new();
            if stdin().read_line(&mut role).is_err() {
                eprintln!("读取Role失败");
                return false;
            }
            
            println!("请输入Content:");
            let mut content = String::new();
            if stdin().read_line(&mut content).is_err() {
                eprintln!("读取Content失败");
                return false;
            }
            
            let new_prompt = Prompt {
                role: role.trim().to_string(),
                content: content.trim().to_string(),
            };
            
            prompts.add_prompt(new_prompt);
            println!("Prompt添加成功!");
        },
        Menu::EDIT => {
            prompts.list_prompts();
            if prompts.prompts.is_empty() {
                return true;
            }
            
            println!("请输入要编辑的Prompt编号:");
            let mut index_input = String::new();
            if stdin().read_line(&mut index_input).is_err() {
                eprintln!("读取输入失败");
                return false;
            }
            
            let index: usize = match index_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("请输入有效的数字");
                    return false;
                }
            };
            
            if index == 0 || index > prompts.prompts.len() {
                eprintln!("无效的Prompt编号");
                return false;
            }
            
            println!("请输入新的Role:");
            let mut role = String::new();
            if stdin().read_line(&mut role).is_err() {
                eprintln!("读取Role失败");
                return false;
            }
            
            println!("请输入新的Content:");
            let mut content = String::new();
            if stdin().read_line(&mut content).is_err() {
                eprintln!("读取Content失败");
                return false;
            }
            
            let updated_prompt = Prompt {
                role: role.trim().to_string(),
                content: content.trim().to_string(),
            };
            
            if prompts.edit_prompt(index - 1, updated_prompt) {
                println!("Prompt编辑成功!");
            } else {
                eprintln!("编辑失败，无效的Prompt编号");
                return false;
            }
        },
        Menu::DELETE => {
            prompts.list_prompts();
            if prompts.prompts.is_empty() {
                return true;
            }
            
            println!("请输入要删除的Prompt编号:");
            let mut index_input = String::new();
            if stdin().read_line(&mut index_input).is_err() {
                eprintln!("读取输入失败");
                return false;
            }
            
            let index: usize = match index_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("请输入有效的数字");
                    return false;
                }
            };
            
            if index == 0 || index > prompts.prompts.len() {
                eprintln!("无效的Prompt编号");
                return false;
            }
            
            if prompts.delete_prompt(index - 1) {
                println!("Prompt删除成功!");
            } else {
                eprintln!("删除失败，无效的Prompt编号");
                return false;
            }
        },
        Menu::CHOOSE => {
            prompts.list_prompts();
            if prompts.prompts.is_empty() {
                return true;
            }
            
            println!("请输入要选择的Prompt编号作为对话的系统提示:");
            let mut index_input = String::new();
            if stdin().read_line(&mut index_input).is_err() {
                eprintln!("读取输入失败");
                return false;
            }
            
            let index: usize = match index_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("请输入有效的数字");
                    return false;
                }
            };
            
            if index == 0 || index > prompts.prompts.len() {
                eprintln!("无效的Prompt编号");
                return false;
            }
            
            if let Some(selected_prompt) = prompts.get_prompt(index - 1) {
                // 清空当前messages并添加选中的prompt作为第一条消息
                app.request_body.messages.clear();
                app.request_body.messages.push(Message {
                    role: String::from("user"),
                    content: selected_prompt.content.clone(),
                });
                app.assistant_name = selected_prompt.role.clone();
                println!("已选择Prompt并设置为对话上下文");
            }
        },
        Menu::BACK => {
            return false;
        }
    }
    
    // 保存到文件
    if let Err(e) = prompts.save_to_file() {
        eprintln!("保存Prompt到文件失败: {}", e);
    }
    
    true
}