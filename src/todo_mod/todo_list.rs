use serde::{ Deserialize, Serialize };
use std::path::Path;
use std::fs::File;
use std::io::{ Write };
use std::process::Command;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Todo {
    pub title: String,
    pub content: String,
    pub create_time: i64,
    pub dead_line: i64
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Todos {
    pub todos: Vec<Todo>
}

impl Todo {
    fn show(&self) {
        let ct = chrono::DateTime::from_timestamp(self.create_time, 0)
            .expect("Failed to convert create_time timestamp");
        let ddl = chrono::DateTime::from_timestamp(self.dead_line, 0)
            .expect("Failed to convert dead_line timestamp");
        println!("{{ Title : {} }}\n{{ Content : {} , Create Time : {} , Deadline : {} }}\n",self.title,self.content,ct,ddl);
    }
}

enum Handler {
    INSERT,
    EDIT,
    REMOVE,
    SHOW,
}

impl Handler {

    fn analyse_flag(flag: &String) -> Handler{
        match flag.trim().to_lowercase().as_str() {
            "1" | "insert" | "add" => Handler::INSERT,
            "2" | "edit" => Handler::EDIT,
            "3" | "remove" | "delete" => Handler::REMOVE,
            "4" | "show" | "list" => Handler::SHOW,
            _ => {
                println!("无效选项，显示待办事项列表");
                Handler::SHOW
            }
        }
    }
}


// 添加这个函数到你的代码中
fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}


impl Todos {

    fn save_todos(&self) -> std::io::Result<()>{
        let path = Path::new("data.json");
        let mut file = File::create(path)?;
        let json_data = serde_json::to_string_pretty(&self.todos).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        file.write_all(json_data.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    fn load_todos(&mut self) -> std::io::Result<()> {
        let path = Path::new("data.json");
        if !path.exists() {
            // 创建文件并写入空数组，避免后续解析错误
            std::fs::write(path, "[]")?;
        }
        let json = std::fs::read_to_string(path)?;
        let todos: Vec<Todo> = serde_json::from_str(&json)?;
        self.todos = todos;
        Ok(())
    }

    fn insert_todo(&mut self) {
        let mut todo = Todo::default();
        println!("输入待办标题：");
        todo.title = self.read_user_input("读取标题失败");
        if todo.title.is_empty() {
            println!("警告：标题为空");
        }
        println!("输入待办内容：");
        todo.content = self.read_user_input("读取内容失败");
        todo.create_time = chrono::Local::now().timestamp();
        println!("输入时间限制（天）：");
        let mut ddl = String::new();
        std::io::stdin().read_line(&mut ddl).expect("读取时间限制失败");
        let ddl: i32 = ddl.trim().parse().expect("请输入一个有效的数字");
        todo.dead_line = chrono::Local::now().timestamp() + (ddl as i64) * 24 * 3600;
        self.todos.push(todo);
        let _ = self.save_todos();
    }
    
    // 辅助函数：读取用户输入并处理
    fn read_user_input(&self, error_msg: &str) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect(error_msg);
        input.trim_end().to_string()
    }

    fn remove_todo(&mut self) {
        let mut todo = Todo::default();
        println!("输入待办标题：");
        std::io::stdin().read_line(&mut todo.title).expect("msg");
        todo.title = todo.title.trim_end().to_string();
        let index = self.todos.iter().position(|t| t.title.eq(&todo.title));
        if let Some(index) = index {
            self.todos.remove(index);
        }
        let _ = self.save_todos();
    }

    fn edit_todo(&mut self) {
        let mut todo = Todo::default();
    
        println!("输入待办标题：");
        match std::io::stdin().read_line(&mut todo.title) {
            Ok(_) => {
                todo.title = todo.title.trim_end().to_string();
                let index = self.todos.iter().position(|t| t.title.eq(&todo.title));
                match index {
                    Some(index) => {
                        println!("输入待办内容：");
                        match std::io::stdin().read_line(&mut todo.content) {
                            Ok(_) => {
                                todo.content = todo.content.trim_end().to_string();
                                println!("输入时间限制（天）：");
                                let mut ddl = String::new();
                                match std::io::stdin().read_line(&mut ddl) {
                                    Ok(_) => {
                                        // 验证时间输入是否为有效数字
                                        if ddl.trim().parse::<u32>().is_ok() {
                                            // 直接替换而不是先删后加，保持顺序
                                            self.todos[index] = todo;
                                        } else {
                                            println!("时间限制必须为有效数字");
                                        }
                                    },
                                    Err(error) => {
                                        println!("读取时间限制失败: {}", error);
                                    }
                                }
                            },
                            Err(error) => {
                                println!("读取内容失败: {}", error);
                            }
                        }
                    },
                    None => {
                        println!("请输入正确的标题")
                    }
                }
            },
            Err(error) => {
                println!("读取标题失败: {}", error);
            }
        }
    }

    fn show_todos(&self) {
        for t in self.todos.iter() {
            t.show();
        }
    }
}

pub fn todo_run() {
    let mut todos = Todos::default();
    let _ = todos.load_todos();
    loop {
        todos.show_todos();
        println!("\n请选择操作:");
        println!("1. 添加待办事项 (insert/add)");
        println!("2. 编辑待办事项 (edit)");
        println!("3. 删除待办事项 (remove/delete)");
        println!("4. 显示待办事项 (show/list)");
        let mut flag = String::new();
        std::io::stdin().read_line(&mut flag).expect("test");
        match Handler::analyse_flag(&flag) {
            Handler::INSERT => {
                todos.insert_todo();
            },
            Handler::EDIT => {
                todos.edit_todo();
            },
            Handler::SHOW => {
            },
            Handler::REMOVE => {
                todos.remove_todo();
            }
        }
        clear_screen();
    }
}