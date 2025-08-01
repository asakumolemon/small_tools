use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::io::{stdin, Write};
use crate::chat_mod::model::ModelList;
use crate::chat_mod::prompt::prompt;
use crate::chat_mod::model::model_management;
use crate::chat_mod::model::Model;
use reqwest::Client;
use futures::StreamExt;
use std::fs::File;
use std::path::Path;



#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct RequestBody {
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
    model: Model,
    models: ModelList,
    pub request_body: RequestBody
}

enum Menu {
    MODEL,
    PROMPT,
    CHAT,
    BACK
}

impl Menu {
    fn form_handler(str: &String) -> Self{
        match str.trim().to_lowercase().as_str() {
            "1"|"model" => Menu::MODEL,
            "2"|"prompt" => Menu::PROMPT,
            "3"|"chat" => Menu::CHAT,
            "0"|"quit"|"exit" => Menu::BACK,
            _ => Menu::BACK
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let mut model = Model::default();
        let models = ModelList::load_from_file();
        if models.models.is_empty() {
            model.model_name = String::from("deepseek-chat");
            model.url = std::env::var("CHAT_URL").unwrap_or_default();
            model.api_key = std::env::var("CHAT_API_KEY").unwrap_or_default();
        } else {
            model = models.get_default_model()
                .or_else(|| models.get_model(0))
                .cloned()
                .unwrap_or_default();
        }
        let mut request_body = RequestBody::default();
        request_body.stream = true;
        Self { 
            assistant_name : String::from("user"),
            model,
            models,
            request_body
        }
    }
}

impl App {
    fn save(&self, file_name: &String) -> Result<(), std::io::Error> {

        if self.request_body.messages.is_empty() {
            return Ok(());
        }

        let path = if cfg!(windows) {
            // Windows系统使用AppData目录
            dirs::data_local_dir().map(|mut p| {
                p.push("SmallTool");
                p.push("History");
                p.push(format!("{}.md", file_name));
                p
            }).unwrap_or_else(|| {
                // 如果无法获取AppData目录，则使用当前目录
                Path::new(format!("{}.md", file_name).as_str()).to_path_buf()
            })
        } else {
            // 非Windows系统保持原逻辑
            dirs::data_dir().map(|mut p| {
                p.push("small_tools");
                p.push("history");
                p.push(format!("{}.md", file_name));
                p
            }).unwrap_or_else(|| {
                // 如果无法获取数据目录，则使用当前目录
                Path::new(format!("{}.md", file_name).as_str()).to_path_buf()
            })
        };
        
        // 确保目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let mut file = File::create(path)?;

        let mut mh = String::new();
        for message in &self.request_body.messages {
            mh += &format!("{}\n{}\n", message.role, message.content);
        }
        file.write_all(mh.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    fn load_history_file(&mut self, file_name: &String) {
        let path = if cfg!(windows) {
            // Windows系统使用AppData目录
            dirs::data_local_dir().map(|mut p| {
                p.push("SmallTool");
                p.push("History");
                p.push(format!("{}.md", file_name));
                p
            }).unwrap_or_else(|| {
                // 如果无法获取AppData目录，则使用当前目录
                Path::new(format!("{}.md", file_name).as_str()).to_path_buf()
            })
        } else {
            // 非Windows系统保持原逻辑
            dirs::data_dir().map(|mut p| {
                p.push("small_tools");
                p.push("history");
                p.push(format!("{}.md", file_name));
                p
            }).unwrap_or_else(|| {
                // 如果无法获取数据目录，则使用当前目录
                Path::new(format!("{}.md", file_name).as_str()).to_path_buf()
            })
        };

        if !path.exists() {
            println!("⚠️ 历史文件不存在");
            return;
        }

        let mh = std::fs::read_to_string(&path);

        let messages: Vec<Message> = mh.unwrap_or_default()
            .split("\n")
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(": ").collect();
                if parts.len() != 2 {
                    return None;
                }
                let role = parts[0].trim();
                let content = parts[1].trim();
                Some(Message {
                    role: role.to_string(),
                    content: content.to_string(),
                })
            }).collect();

        self.request_body.messages = messages;

        println!("📜 历史记录：");
        for message in &self.request_body.messages {
            println!("{}:\n{}", message.role, message.content);
        }

    }
}

fn chat(app: &mut App) -> bool{
    // app.request_body.model = String::from("deepseek-chat");

    println!("💬 请输入对话内容：");
    let mut sm = String::new();
    if let Err(e) = stdin().read_line(&mut sm) {
        eprintln!("❌ 读取输入失败: {}", e);
        return false;
    }
        
    if sm.is_empty() {
        eprintln!("⚠️ 输入内容不能为空");
        return false;
    }

    sm = sm.trim().to_string();

    if sm.eq(":b") {
        return false;
    }

    if sm.eq(":c") {
        app.request_body.messages.clear();
        return true;
    }
    
    if sm.eq(":cls") {
        print!("\x1B[2J\x1B[1;1H");
        return true;
    }

    if sm.eq(":revert") {
        if app.request_body.messages.len() < 2 {
            eprintln!("⚠️ 没有可撤销的消息");
            return true;
        }
        app.request_body.messages.pop();
        app.request_body.messages.pop();
        return true;
    }

    if sm.starts_with(":save:") {
        let file_name = sm.trim_start_matches(":save:").trim().to_string();
        app.save(&file_name).expect("保存失败");
        return true;
    }

    if sm.starts_with(":load:") {
        // 从命令中提取文件名
        let file_name = sm.trim_start_matches(":load:").trim().to_string();
        // 加载历史记录文件
        app.load_history_file(&file_name);
        return true;
    }

    // 将用户消息添加到请求体中
    app.request_body.messages.push(Message {
        role: String::from("user"),
        content: sm,
    });
    app.request_body.stream = true; // 启用流式输出

    // 创建要发送的JSON数据
    let json_data = json!({
        "model": &app.model.model_name,
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
        .post(&app.model.url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", app.model.api_key))
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
    let role = if !app.assistant_name.eq("user") {
        app.assistant_name.clone()     
    } else {
        String::from("🤖 Assistant")
    };

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
        println!("║          🤖 问答模式菜单             ║");
        println!("╠══════════════════════════════════════╣");
        println!("║  选项  │ 功能说明                    ║");
        println!("║────────┼─────────────────────────────║");
        println!("║   1    │ 🤖 模型配置 (model)         ║");
        println!("║   2    │ 🛠️ Prompt配置 (prompt)     ║");
        println!("║   3    │ 💬 进入问答 (chat)          ║");
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
            Menu::MODEL => {
                println!("🤖 进入模型配置模式");
                model_management();
            },
            Menu::PROMPT => {
                println!("🛠️ 进入Prompt配置模式");
                prompt(&mut app);
            },
            Menu::CHAT => {
                println!("💬 进入问答模式（“:b”退出）");
                loop {
                    if !chat(&mut app) {
                        break;
                    }
                }
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