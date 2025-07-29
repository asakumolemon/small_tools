use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::io::stdin;
use std::process::Command;


#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    role: String,
    content: String
}
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
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
struct App {
    url: String,
    api_key: String,
    request_body: RequestBody
}

enum Menu {
    PROMPT,
    CHAT,
    BACK
}

impl Menu {
    fn form_handler(str: &String) -> Self{
        match str.trim().to_lowercase().as_str() {
            "1"|"add" => Menu::PROMPT,
            "2"|"edit" => Menu::CHAT,
            _ => Menu::BACK
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self { 
            url: String::from("https://api.deepseek.com/chat/completions"),
            api_key: String::from("sk-a9823c3f2a8d44869eea8422af8f7a92"),
            request_body: RequestBody::default() 
        }
    }
}

fn chat(app: &mut App) {
    app.request_body.model = String::from("deepseek-chat");

    println!("请输入对话内容：");
    let mut sm = String::new();
    if let Err(e) = stdin().read_line(&mut sm) {
        eprintln!("读取输入失败: {}", e);
        return;
    }
    sm = sm.trim().to_string();
    
    if sm.is_empty() {
        eprintln!("输入内容不能为空");
        return;
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
            return;
        }
    };

    let result: Result<ChatResponse, ureq::Error> = rm.into_body().read_json();
    let result = match result {
        Ok(data) => data,
        Err(e) => {
            eprintln!("解析响应失败: {}", e);
            return;
        }
    };
    
    // 提取需要的信息
    let model_name = result.model.clone();
    let usage = result.usage.clone();
    
    if result.choices.is_empty() {
        eprintln!("响应中没有找到任何选择项");
        return;
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
}

pub fn chat_run() {
    let mut app = App::default();
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
    println!("已进入聊天模式");
    loop {
        chat(&mut app)
    }
}