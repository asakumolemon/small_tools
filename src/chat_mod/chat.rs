use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::io::{stdin, Write};
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

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct ChatResponseChunk {
    model: String,
    choices: Vec<ChoiceChunk>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct ChoiceChunk {
    delta: MessageDelta,
    finish_reason: Option<String>,
    index: u32,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
struct MessageDelta {
    role: Option<String>,
    content: Option<String>,
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

    println!("💬 请输入对话内容：");
    let mut sm = String::new();
    if let Err(e) = stdin().read_line(&mut sm) {
        eprintln!("❌ 读取输入失败: {}", e);
        return false;
    }
    sm = sm.trim().to_string();

    if sm.eq(":b") {
        return false;
    }
    
    if sm.is_empty() {
        eprintln!("⚠️ 输入内容不能为空");
        return false;
    }

    // 将用户消息添加到请求体中
    app.request_body.messages.push(Message {
        role: String::from("user"),
        content: sm,
    });
    app.request_body.stream = true; // 启用流式输出

    // 创建要发送的JSON数据
    let json_data = json!({
        "model": &app.request_body.model,
        "messages": &app.request_body.messages,
        "stream": &app.request_body.stream
    });

    // 使用异步运行时执行流式请求
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        stream_chat(app, json_data).await
    })
}

async fn stream_chat(app: &mut App, json_data: serde_json::Value) -> bool {
    let client = Client::new();
    
    // 发送包含请求体的POST请求
    let response = match client
        .post(&app.url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", app.api_key))
        .json(&json_data)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            eprintln!("❌ 发送请求失败: {}", e);
            return false;
        }
    };

    if !response.status().is_success() {
        eprintln!("❌ 请求失败，状态码: {}", response.status());
        return false;
    }

    let mut stream = response.bytes_stream();
    let mut full_content = String::new();
    let mut role = String::new();
    if !app.assistant_name.eq("user") {
        role = app.assistant_name.clone();     
    }else {
        role = String::from("🤖 Assistant");
    }

    println!("================================================================================");
    println!("👤 角色: {}", role);
    println!("--------------------------------------------------------------------------------");
    print!("💬 回复: ");
    
    // 实时处理流式响应
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                let chunk_str = String::from_utf8_lossy(&bytes);
                let lines: Vec<&str> = chunk_str.split('\n').collect();
                
                for line in lines {
                    if line.starts_with("data: ") {
                        let data = &line[6..]; // 移除 "data: " 前缀
                        
                        if data == "[DONE]" {
                            // 流完成
                            break;
                        }
                        
                        // 解析JSON数据
                        match serde_json::from_str::<ChatResponseChunk>(data) {
                             Ok(chunk_data) => {
                                if let Some(choice) = chunk_data.choices.first() {
                                    if let Some(ref delta) = choice.delta.content {
                                        print!("{}", delta);
                                        std::io::stdout().flush().unwrap(); // 立即刷新输出
                                        full_content.push_str(delta);
                                    }
                                }
                            }
                            Err(_) => {
                                // 忽略解析错误，可能是一些特殊格式的数据
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ 接收数据时出错: {}", e);
                return false;
            }
        }
    }
    
    println!("\n================================================================================");
    
    // 将助手的回复添加到消息历史中
    app.request_body.messages.push(Message { 
        role: "assistant".to_string(), 
        content: full_content.clone() 
    });
    
    true
}

pub fn chat_run() {
    let mut app = App::default();
    loop {
        println!("╔══════════════════════════════════════╗");
        println!("║          🤖 聊天模式菜单             ║");
        println!("╠══════════════════════════════════════╣");
        println!("║  选项  │ 功能说明                    ║");
        println!("║────────┼─────────────────────────────║");
        println!("║   1    │ 🛠️ Prompt配置 (prompt)       ║");
        println!("║   2    │ 💬 进入聊天 (chat)          ║");
        println!("║   0    │ 🚪 退出程序 (quit/exit)     ║");
        println!("╚══════════════════════════════════════╝");
        
        let mut flag = String::new();
        
        // 改进错误处理信息
        if let Err(e) = stdin().read_line(&mut flag) {
            eprintln!("❌ 读取输入失败: {}", e);
            continue;
        }

        let choice = Menu::form_handler(&flag);

        match choice {
            Menu::CHAT => {
                println!("💬 进入聊天模式（“:b”退出）");
                loop {
                    if !chat(&mut app) {
                        break;
                    }
                }
            },
            Menu::PROMPT => {
                println!("🛠️ 进入Prompt配置模式");
                prompt(&mut app);
            },
            Menu::BACK => {
                // 返回上级菜单
                println!("🚪 退出程序");
                break;
            }
        }
        
        // 等待用户按键继续
        println!("按回车键继续...");
        let _ = stdin().read_line(&mut String::new());
    }   
}