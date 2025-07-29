use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::io::stdin;
use crate::chat_mod::prompt::prompt;
use reqwest::Client;
use futures::StreamExt;
use std::error::Error;


#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String
}
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct RequestBody {
    model: String,
    pub messages: Vec<Message>,
    stream: bool
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct ChatResponse {
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub assistant_name: String,
    url: String,
    api_key: String,
    pub request_body: RequestBody
}

enum Menu {
    PROMPT,
    CHAT,
    BACK
}

impl Menu {
    fn form_handler(str: &String) -> Self{
        match str.trim().to_lowercase().as_str() {
            "1"|"prompt" => Menu::PROMPT,
            "2"|"chat" => Menu::CHAT,
            "0"|"quit"|"exit" => Menu::BACK,
            _ => Menu::BACK
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let url = std::env::var("CHAT_URL").unwrap_or_else(|_| String::from("https://api.deepseek.com/chat/completions"));
        let api_key = std::env::var("CHAT_API_KEY").expect("CHAT_API_KEY environment variable not set");
        let mut request_body = RequestBody::default();
        request_body.stream = true;
        Self { 
            assistant_name : String::from("user"),
            url,
            api_key,
            request_body
        }
    }
}

fn chat(app: &mut App) -> bool{
    app.request_body.model = String::from("deepseek-chat");

    println!("请输入对话内容：");
    if !app.assistant_name.eq("user") {
        println!("当前角色：{}", app.assistant_name);
    }
    let mut sm = String::new();
    if let Err(e) = stdin().read_line(&mut sm) {
        eprintln!("读取输入失败: {}", e);
        return false;
    }
    sm = sm.trim().to_string();

    if sm.eq(":b") {
        return false;
    }
    
    if sm.is_empty() {
        eprintln!("输入内容不能为空");
        return false;
    }

    // 将用户消息添加到请求体中
    app.request_body.messages.push(Message {
        role: String::from("user"),
        content: sm,
    });
    app.request_body.stream = false;

    // 创建要发送的JSON数据
    let json_data = json!({
        "model": &app.request_body.model,
        "messages": &app.request_body.messages,
        "stream": &app.request_body.stream
    });

    // 发送包含请求体的POST请求
    let rm = match ureq::post(&app.url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", app.api_key))
        .send_json(json_data)
    {
        Ok(response) => response,
        Err(e) => {
            eprintln!("发送请求失败: {}", e);
            return false;
        }
    };

    let result: Result<ChatResponse, ureq::Error> = rm.into_body().read_json();
    let result = match result {
        Ok(data) => data,
        Err(e) => {
            eprintln!("解析响应失败: {}", e);
            return false;
        }
    };
    
    // 提取需要的信息
    let model_name = result.model.clone();
    let usage = result.usage.clone();
    
    if result.choices.is_empty() {
        eprintln!("响应中没有找到任何选择项");
        return false;
    }
    
    let message = &result.choices[0].message;
    let role = message.role.clone();
    let content = message.content.clone();

    app.request_body.messages.push(Message { role: role.clone(), content: content.clone() });
    
    // 打印解析后的信息
    println!("========================================");
    println!("模型名称: {}", model_name);
    println!("角色:     {}", role);
    println!("回复内容: {}", content);
    println!("========================================");
    println!("Token 使用情况:");
    println!("┌───────────────────────────────────────┐");
    println!("│ 提示 Token 数:     {:>18} │", usage.prompt_tokens);
    println!("│ 完成 Token 数:     {:>18} │", usage.completion_tokens);
    println!("│ 总 Token 数:       {:>18} │", usage.total_tokens);
    println!("└───────────────────────────────────────┘");

    return true;
}

pub fn chat_run() {
    let mut app = App::default();
    loop {
        println!("==============================");
        println!("请选择操作:");
        println!("1. Prompt配置 (add)");
        println!("2. 进入聊天 (edit)");
        println!("0. 退出程序 (quit/exit)");
        println!("==============================");
        
        let mut flag = String::new();
        
        // 改进错误处理信息
        if let Err(e) = stdin().read_line(&mut flag) {
            eprintln!("读取输入失败: {}", e);
            continue;
        }

        let choice = Menu::form_handler(&flag);


        match choice {
            Menu::CHAT => {
                println!("进入聊天模式");
                loop {
                    if !chat(&mut app) {
                        break;
                    }
                }
            },
            Menu::PROMPT => {
                println!("进入Prompt配置模式");
                prompt(&mut app);
            },
            Menu::BACK => {
                // 返回上级菜单
                println!("退出程序");
                break;
            }
        }
        
        // 等待用户按键继续
        println!("按回车键继续...");
        let _ = stdin().read_line(&mut String::new());
    }   
}