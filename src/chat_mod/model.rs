use std::io::stdin;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Model {
    pub api_key: String,
    pub model_name: String,
    pub url: String,
    pub default: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelList {
    pub models: Vec<Model>,
}

impl ModelList {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
        }
    }

    pub fn load_from_file() -> Self {
        let path = if cfg!(windows) {
            // Windows系统使用AppData目录
            dirs::data_local_dir().map(|mut p| {
                p.push("SmallTool");
                p.push("models.json");
                p
            }).unwrap_or_else(|| {
                // 如果无法获取AppData目录，则使用当前目录
                Path::new("models.json").to_path_buf()
            })
        } else {
            // 非Windows系统保持原逻辑
            dirs::data_dir().map(|mut p| {
                p.push("small_tools");
                p.push("models.json");
                p
            }).unwrap_or_else(|| {
                // 如果无法获取数据目录，则使用当前目录
                Path::new("models.json").to_path_buf()
            })
        };
        
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(models) => models,
                        Err(_) => ModelList::new(),
                    }
                }
                Err(_) => ModelList::new(),
            }
        } else {
            ModelList::new()
        }
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = if cfg!(windows) {
            // Windows系统使用AppData目录
            dirs::data_local_dir().map(|mut p| {
                p.push("SmallTool");
                p.push("models.json");
                p
            }).unwrap_or_else(|| {
                // 如果无法获取AppData目录，则使用当前目录
                Path::new("models.json").to_path_buf()
            })
        } else {
            // 非Windows系统保持原逻辑
            dirs::data_dir().map(|mut p| {
                p.push("small_tools");
                p.push("models.json");
                p
            }).unwrap_or_else(|| {
                // 如果无法获取数据目录，则使用当前目录
                Path::new("models.json").to_path_buf()
            })
        };
        
        // 确保目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn edit_model(&mut self, index: usize, model: Model) -> bool {
        if index < self.models.len() {
            self.models[index] = model;
            true
        } else {
            false
        }
    }

    pub fn delete_model(&mut self, index: usize) -> bool {
        if index < self.models.len() {
            self.models.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_models(&self) {
        if self.models.is_empty() {
            println!("📭 暂无模型配置");
            return;
        }

        println!("📋 当前模型列表:");
        for (i, model) in self.models.iter().enumerate() {
            if model.default {
                println!("{}. Model: {}, URL: {} [默认]", i + 1, model.model_name, model.url);
            } else {
                println!("{}. Model: {}, URL: {}", i + 1, model.model_name, model.url);
            }
        }
    }

    pub fn get_model(&self, index: usize) -> Option<&Model> {
        if index < self.models.len() {
            Some(&self.models[index])
        } else {
            None
        }
    }

    pub fn set_default(&mut self, index: usize) -> bool {
        if index < self.models.len() {
            // 将所有模型的default设置为false
            for model in self.models.iter_mut() {
                model.default = false;
            }
            // 将指定模型的default设置为true
            self.models[index].default = true;
            true
        } else {
            false
        }
    }

    pub fn get_default_model(&self) -> Option<&Model> {
        self.models.iter().find(|model| model.default)
    }
}

enum Menu {
    ADD,
    EDIT,
    DELETE,
    CHOOSE,
    SETDEFAULT,
    BACK
}

impl Menu {
    fn form_handler(str: &String) -> Self{
        match str.trim().to_lowercase().as_str() {
            "1"|"add" => Menu::ADD,
            "2"|"edit" => Menu::EDIT,
            "3"|"delete" => Menu::DELETE,
            "4"|"choose" => Menu::CHOOSE,
            "5"|"default" => Menu::SETDEFAULT,
            _ => {
                Menu::BACK
            }
        }
    }
}

pub fn model_management() -> Option<Model> {
    let mut models = ModelList::load_from_file();
    
    loop {
        println!("================================================================================");
        println!("🔧 模型配置菜单");
        println!("--------------------------------------------------------------------------------");
        println!("请选择操作:");
        println!("1. ➕ 添加模型 (add)");
        println!("2. ✏️  编辑模型 (edit)");
        println!("3. 🗑️  删除模型 (delete)");
        println!("4. 📋 查看/选择模型 (choose)");
        println!("5. 🎯 设置默认模型 (default)");
        println!("其他. 🔙 返回上级菜单");
        println!("================================================================================");
        println!();

        let mut input = String::new();

        if let Err(error) = stdin().read_line(&mut input) {
            eprintln!("❌ 读取输入失败: {}", error);
            return None;
        }

        let choice = Menu::form_handler(&input);

        match choice {
            Menu::ADD => {
                println!("🔑 请输入API Key:");
                let mut api_key = String::new();
                if stdin().read_line(&mut api_key).is_err() {
                    eprintln!("❌ 读取API Key失败");
                    continue;
                }
                
                println!("🤖 请输入模型名称:");
                let mut model_name = String::new();
                if stdin().read_line(&mut model_name).is_err() {
                    eprintln!("❌ 读取模型名称失败");
                    continue;
                }
                
                println!("🌐 请输入URL:");
                let mut url = String::new();
                if stdin().read_line(&mut url).is_err() {
                    eprintln!("❌ 读取URL失败");
                    continue;
                }
                
                let new_model = Model {
                    api_key: api_key.trim().to_string(),
                    model_name: model_name.trim().to_string(),
                    url: url.trim().to_string(),
                    default: false
                };
                
                models.add_model(new_model);
                println!("✅ 模型添加成功!");
            },
            Menu::EDIT => {
                models.list_models();
                if models.models.is_empty() {
                    continue;
                }
                
                println!("✏️  请输入要编辑的模型编号:");
                let mut index_input = String::new();
                if stdin().read_line(&mut index_input).is_err() {
                    eprintln!("❌ 读取输入失败");
                    continue;
                }
                
                let index: usize = match index_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("❌ 请输入有效的数字");
                        continue;
                    }
                };
                
                if index == 0 || index > models.models.len() {
                    eprintln!("❌ 无效的模型编号");
                    continue;
                }
                
                println!("🔑 请输入新的API Key:");
                let mut api_key = String::new();
                if stdin().read_line(&mut api_key).is_err() {
                    eprintln!("❌ 读取API Key失败");
                    continue;
                }
                
                println!("🤖 请输入新的模型名称:");
                let mut model_name = String::new();
                if stdin().read_line(&mut model_name).is_err() {
                    eprintln!("❌ 读取模型名称失败");
                    continue;
                }
                
                println!("🌐 请输入新的URL:");
                let mut url = String::new();
                if stdin().read_line(&mut url).is_err() {
                    eprintln!("❌ 读取URL失败");
                    continue;
                }
                
                let updated_model = Model {
                    api_key: api_key.trim().to_string(),
                    model_name: model_name.trim().to_string(),
                    url: url.trim().to_string(),
                    default: models.models[index-1].default // 保持原来的默认设置
                };
                
                if models.edit_model(index - 1, updated_model) {
                    println!("✅ 模型编辑成功!");
                } else {
                    eprintln!("❌ 编辑失败，无效的模型编号");
                    continue;
                }
            },
            Menu::DELETE => {
                models.list_models();
                if models.models.is_empty() {
                    continue;
                }
                
                println!("🗑️  请输入要删除的模型编号:");
                let mut index_input = String::new();
                if stdin().read_line(&mut index_input).is_err() {
                    eprintln!("❌ 读取输入失败");
                    continue;
                }
                
                let index: usize = match index_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("❌ 请输入有效的数字");
                        continue;
                    }
                };
                
                if index == 0 || index > models.models.len() {
                    eprintln!("❌ 无效的模型编号");
                    continue;
                }
                
                // 棑查是否要删除默认模型
                if models.models[index-1].default {
                    println!("⚠️  您正在删除默认模型，确认删除吗？(y/N)");
                    let mut confirm = String::new();
                    if stdin().read_line(&mut confirm).is_err() {
                        eprintln!("❌ 读取输入失败");
                        continue;
                    }
                    if !["y", "yes", "Y", "Yes"].contains(&confirm.trim()) {
                        println!("❌ 取消删除操作");
                        continue;
                    }
                }
                
                if models.delete_model(index - 1) {
                    println!("✅ 模型删除成功!");
                } else {
                    eprintln!("❌ 删除失败，无效的模型编号");
                    continue;
                }
            },
            Menu::CHOOSE => {
                models.list_models();
                if models.models.is_empty() {
                    continue;
                }
                
                println!("🔍 请输入要选择的模型编号:");
                let mut index_input = String::new();
                if stdin().read_line(&mut index_input).is_err() {
                    eprintln!("❌ 读取输入失败");
                    continue;
                }
                
                let index: usize = match index_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("❌ 请输入有效的数字");
                        continue;
                    }
                };
                
                if index == 0 || index > models.models.len() {
                    eprintln!("❌ 无效的模型编号");
                    continue;
                }
                
                if let Some(selected_model) = models.get_model(index - 1) {
                    println!("✅ 已选择模型: {}", selected_model.model_name);
                    return Some(selected_model.clone());
                }
            },
            Menu::SETDEFAULT => {
                models.list_models();
                if models.models.is_empty() {
                    continue;
                }
                
                println!("🎯 请输入要设置为默认的模型编号:");
                let mut index_input = String::new();
                if stdin().read_line(&mut index_input).is_err() {
                    eprintln!("❌ 读取输入失败");
                    continue;
                }
                
                let index: usize = match index_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("❌ 请输入有效的数字");
                        continue;
                    }
                };
                
                if index == 0 || index > models.models.len() {
                    eprintln!("❌ 无效的模型编号");
                    continue;
                }
                
                if models.set_default(index - 1) {
                    println!("✅ 默认模型设置成功!");
                } else {
                    eprintln!("❌ 设置失败，无效的模型编号");
                    continue;
                }
            },
            Menu::BACK => {
                break;
            }
        }
        
        // 保存到文件
        if let Err(e) = models.save_to_file() {
            eprintln!("❌ 保存模型到文件失败: {}", e);
        }
    }
    
    None
}