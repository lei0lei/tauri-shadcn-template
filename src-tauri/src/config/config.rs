use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};
use std::sync::{RwLock};  // 引入标准库中的 RwLock
use toml;
use tauri::AppHandle;
use std::{path::{Path, PathBuf}};
use lazy_static::lazy_static;
use std::collections::HashMap;
use tauri::path::BaseDirectory;
use toml::Value;  // 确保正确导入 toml::Value

// 子配置项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConfigData {
    pub data: HashMap<String, toml::Value>,
    pub file_path: Option<PathBuf>,
}


// 定义总体配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub run: RunConfig,
    pub recipes: RecipesConfig,
    pub hardware: HardwareConfig,
    pub algo: AlgoConfig,
}


lazy_static! {
    pub static ref CONFIG: Arc<RwLock<Option<Config>>> = Arc::new(RwLock::new(None));
}

/// **每个子配置项都是一个 TOML 对象**
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RunConfig {
    data: HashMap<String, toml::Value>,
    file_path: Option<PathBuf>,  // 保存配置文件的路径
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecipesConfig {
    data: HashMap<String, toml::Value>,
    file_path: Option<PathBuf>,  // 保存配置文件的路径
}


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HardwareConfig {
    data: HashMap<String, toml::Value>,
    file_path: Option<PathBuf>,  // 保存配置文件的路径
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AlgoConfig {
    data: HashMap<String, toml::Value>,
    file_path: Option<PathBuf>,  // 保存配置文件的路径
}

// ██████╗ ██████╗ ███╗   ██╗███████╗██╗ ██████╗ 
// ██╔════╝██╔═══██╗████╗  ██║██╔════╝██║██╔════╝ 
// ██║     ██║   ██║██╔██╗ ██║█████╗  ██║██║  ███╗
// ██║     ██║   ██║██║╚██╗██║██╔══╝  ██║██║   ██║
// ╚██████╗╚██████╔╝██║ ╚████║██║     ██║╚██████╔╝
//  ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝     ╚═╝ ╚═════╝ 
                                               

impl Config {
    /// **加载所有配置**
    pub fn load(run_path:PathBuf,
        recipes_path:PathBuf,
        hardware_path:PathBuf,
        algo_path:PathBuf    
    ) -> Result<Self, Box<dyn std::error::Error>> {

        Ok(Config {
            run: RunConfig::load(run_path)?,
            recipes: RecipesConfig::load(recipes_path)?,
            hardware: HardwareConfig::load(hardware_path)?,
            algo: AlgoConfig::load(algo_path)?,
        })
    }
}

// ██████╗ ██╗   ██╗███╗   ██╗
// ██╔══██╗██║   ██║████╗  ██║
// ██████╔╝██║   ██║██╔██╗ ██║
// ██╔══██╗██║   ██║██║╚██╗██║
// ██║  ██║╚██████╔╝██║ ╚████║
// ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝
                           
/// **RunConfig 子配置**
impl RunConfig {
    /// **加载配置**
    pub fn load(file_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        
        let data = if let Ok(content) = fs::read_to_string(&file_path) {
            let parsed: HashMap<String, Value> = toml::from_str(&content)?;  
            parsed  // 直接返回 HashMap
        } else {
            HashMap::new()  // 确保 data 是 HashMap
        };

        Ok(RunConfig {
            data,
            file_path: Some(file_path),
        })
    }

    /// **更新配置并写回 TOML 文件**
    pub fn update<F>(&mut self, modify: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut toml::Value),
    {
        if let Some(file_path) = &self.file_path {
            let mut toml_value = Value::Table(self.data.clone().into_iter().collect());
            modify(&mut toml_value);

            // 解析回 HashMap
        if let Some(new_table) = toml_value.as_table() {
            self.data = new_table.clone().into_iter().collect();
        }
             // 序列化并写入文件
        let new_toml = toml::to_string_pretty(&self.data)?;
            fs::write(file_path, new_toml)?;
        }
        Ok(())
    }
    
    pub fn get_value(&self, key: &str) -> Option<&toml::Value> {
        // 分割 key 来处理嵌套的表
        let keys: Vec<&str> = key.split('.').collect();

        // 递归访问嵌套的表
        self.get_nested_value(&keys)
    }

    fn get_nested_value(&self, keys: &[&str]) -> Option<&toml::Value> {
        if keys.is_empty() {
            return None; // 如果没有更多的键了，返回 None
        }

        let key = keys[0];
        let remaining_keys = &keys[1..];

        // 首先检查当前层级的键是否存在
        if let Some(value) = self.data.get(key) {
            // 如果还有剩余的键，且当前值是表格类型，则继续深入
            if !remaining_keys.is_empty() {
                if let toml::Value::Table(table) = value {
                    return HardwareConfig::get_table_value(table, remaining_keys);
                }
            } else {
                // 如果没有更多的键，则返回当前值
                return Some(value);
            }
        }
        None // 找不到键时返回 None
    }

    // 处理表格类型的值
    fn get_table_value<'a>(
        table: &'a toml::map::Map<String, toml::Value>,
        keys: &[&str],
    ) -> Option<&'a toml::Value> {
        if let Some(value) = table.get(keys[0]) {
            // 如果还有剩余的键，递归处理
            if keys.len() > 1 {
                if let toml::Value::Table(ref inner_table) = value {
                    return HardwareConfig::get_table_value(inner_table, &keys[1..]);
                }
            } else {
                // 如果没有更多的键，则返回当前值
                return Some(value);
            }
        }
        None // 如果找不到该键，则返回 None
    }

    /// 设置或更新配置项（支持嵌套）
    pub fn set_value(&mut self, key: &str, new_value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<&str> = key.split('.').collect(); // 修正这里
        self.update(|toml_value| {
            if let Value::Table(table) = toml_value {
                Self::modify_nested_value(table, &keys, Some(new_value.clone()));
            }
        })
    }

    /// 删除某个配置项（支持嵌套）
    pub fn remove_value(&mut self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<&str> = key.split('.').collect(); // 修正这里
        self.update(|toml_value| {
            if let Value::Table(table) = toml_value {
                Self::modify_nested_value(table, &keys, None);
            }
        })
    }

    fn modify_nested_value(table: &mut toml::map::Map<String, Value>, keys: &[&str], new_value: Option<Value>) {
        if let Some((first_key, rest_keys)) = keys.split_first() {
            if rest_keys.is_empty() {
                // **修改或删除最终的值**
                if let Some(value) = new_value {
                    table.insert(first_key.to_string(), value);
                } else {
                    table.remove(*first_key);
                }
            } else {
                // **递归进入嵌套 Table**
                match table.entry(first_key.to_string()) {
                    toml::map::Entry::Occupied(mut entry) => {
                        if let Value::Table(inner_table) = entry.get_mut() {
                            Self::modify_nested_value(inner_table, rest_keys, new_value);
                        }
                    }
                    toml::map::Entry::Vacant(entry) => {  // ✅ 这里确保使用的是 toml::map::Entry
                        if new_value.is_some() {
                            let mut new_table = toml::map::Map::new();
                            Self::modify_nested_value(&mut new_table, rest_keys, new_value);
                            entry.insert(Value::Table(new_table));
                        }
                    }
                }
            }
        }
    }
}

// ██████╗ ███████╗ ██████╗██╗██████╗ ███████╗███████╗
// ██╔══██╗██╔════╝██╔════╝██║██╔══██╗██╔════╝██╔════╝
// ██████╔╝█████╗  ██║     ██║██████╔╝█████╗  ███████╗
// ██╔══██╗██╔══╝  ██║     ██║██╔═══╝ ██╔══╝  ╚════██║
// ██║  ██║███████╗╚██████╗██║██║     ███████╗███████║
// ╚═╝  ╚═╝╚══════╝ ╚═════╝╚═╝╚═╝     ╚══════╝╚══════╝                                                  

/// **RecipesConfig 子配置**
impl RecipesConfig {
    pub fn load(file_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let data = if let Ok(content) = fs::read_to_string(&file_path) {
            let parsed: HashMap<String, Value> = toml::from_str(&content)?;  
            parsed  // 直接返回 HashMap
        } else {
            HashMap::new()  // 确保 data 是 HashMap
        };

        Ok(RecipesConfig {
            data,
            file_path: Some(file_path),
        })
    }

    pub fn update<F>(&mut self, modify: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut toml::Value),
    {
        if let Some(file_path) = &self.file_path {
            // **关键修正**
            // 需要把 `HashMap<String, Value>` 转换为 `toml::map::Map<String, Value>`
            let mut toml_value = Value::Table(self.data.clone().into_iter().collect());  
            modify(&mut toml_value);

            // **修正 toml_value.as_table() 类型**
            self.data = toml_value.as_table().unwrap().clone().into_iter().collect();

            let new_toml = toml::to_string_pretty(&self.data)?;
            fs::write(file_path, new_toml)?;
        }
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<&toml::Value> {
        // 分割 key 来处理嵌套的表
        let keys: Vec<&str> = key.split('.').collect();

        // 递归访问嵌套的表
        self.get_nested_value(&keys)
    }

    fn get_nested_value(&self, keys: &[&str]) -> Option<&toml::Value> {
        if keys.is_empty() {
            return None; // 如果没有更多的键了，返回 None
        }

        let key = keys[0];
        let remaining_keys = &keys[1..];

        // 首先检查当前层级的键是否存在
        if let Some(value) = self.data.get(key) {
            // 如果还有剩余的键，且当前值是表格类型，则继续深入
            if !remaining_keys.is_empty() {
                if let toml::Value::Table(table) = value {
                    return HardwareConfig::get_table_value(table, remaining_keys);
                }
            } else {
                // 如果没有更多的键，则返回当前值
                return Some(value);
            }
        }
        None // 找不到键时返回 None
    }

    // 处理表格类型的值
    fn get_table_value<'a>(
        table: &'a toml::map::Map<String, toml::Value>,
        keys: &[&str],
    ) -> Option<&'a toml::Value> {
        if let Some(value) = table.get(keys[0]) {
            // 如果还有剩余的键，递归处理
            if keys.len() > 1 {
                if let toml::Value::Table(ref inner_table) = value {
                    return HardwareConfig::get_table_value(inner_table, &keys[1..]);
                }
            } else {
                // 如果没有更多的键，则返回当前值
                return Some(value);
            }
        }
        None // 如果找不到该键，则返回 None
    }

    /// 设置或更新配置项（支持嵌套）
    pub fn set_value(&mut self, key: &str, new_value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<&str> = key.split('.').collect(); // 修正这里
        self.update(|toml_value| {
            if let Value::Table(table) = toml_value {
                Self::modify_nested_value(table, &keys, Some(new_value.clone()));
            }
        })
    }

    /// 删除某个配置项（支持嵌套）
    pub fn remove_value(&mut self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<&str> = key.split('.').collect(); // 修正这里
        self.update(|toml_value| {
            if let Value::Table(table) = toml_value {
                Self::modify_nested_value(table, &keys, None);
            }
        })
    }

    fn modify_nested_value(table: &mut toml::map::Map<String, Value>, keys: &[&str], new_value: Option<Value>) {
        if let Some((first_key, rest_keys)) = keys.split_first() {
            if rest_keys.is_empty() {
                // **修改或删除最终的值**
                if let Some(value) = new_value {
                    table.insert(first_key.to_string(), value);
                } else {
                    table.remove(*first_key);
                }
            } else {
                // **递归进入嵌套 Table**
                match table.entry(first_key.to_string()) {
                    toml::map::Entry::Occupied(mut entry) => {
                        if let Value::Table(inner_table) = entry.get_mut() {
                            Self::modify_nested_value(inner_table, rest_keys, new_value);
                        }
                    }
                    toml::map::Entry::Vacant(entry) => {  // ✅ 这里确保使用的是 toml::map::Entry
                        if new_value.is_some() {
                            let mut new_table = toml::map::Map::new();
                            Self::modify_nested_value(&mut new_table, rest_keys, new_value);
                            entry.insert(Value::Table(new_table));
                        }
                    }
                }
            }
        }
    }
}

// ██╗  ██╗ █████╗ ██████╗ ██████╗ ██╗    ██╗ █████╗ ██████╗ ███████╗
// ██║  ██║██╔══██╗██╔══██╗██╔══██╗██║    ██║██╔══██╗██╔══██╗██╔════╝
// ███████║███████║██████╔╝██║  ██║██║ █╗ ██║███████║██████╔╝█████╗  
// ██╔══██║██╔══██║██╔══██╗██║  ██║██║███╗██║██╔══██║██╔══██╗██╔══╝  
// ██║  ██║██║  ██║██║  ██║██████╔╝╚███╔███╔╝██║  ██║██║  ██║███████╗
// ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝  ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝
                                                                  
/// **HardwareConfig 子配置**
impl HardwareConfig {
    pub fn load(file_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let data = if let Ok(content) = fs::read_to_string(&file_path) {
            let parsed: HashMap<String, Value> = toml::from_str(&content)?;  
            parsed  // 直接返回 HashMap
        } else {
            HashMap::new()  // 确保 data 是 HashMap
        };

        Ok(HardwareConfig {
            data,
            file_path: Some(file_path),
        })
    }

    pub fn update<F>(&mut self, modify: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut toml::Value),
    {
        if let Some(file_path) = &self.file_path {
            // **关键修正**
            // 需要把 `HashMap<String, Value>` 转换为 `toml::map::Map<String, Value>`
            let mut toml_value = Value::Table(self.data.clone().into_iter().collect());  
            modify(&mut toml_value);

            // **修正 toml_value.as_table() 类型**
            self.data = toml_value.as_table().unwrap().clone().into_iter().collect();

            let new_toml = toml::to_string_pretty(&self.data)?;
            fs::write(file_path, new_toml)?;
        }
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<&toml::Value> {
        // 分割 key 来处理嵌套的表
        let keys: Vec<&str> = key.split('.').collect();

        // 递归访问嵌套的表
        self.get_nested_value(&keys)
    }

    fn get_nested_value(&self, keys: &[&str]) -> Option<&toml::Value> {
        if keys.is_empty() {
            return None; // 如果没有更多的键了，返回 None
        }

        let key = keys[0];
        let remaining_keys = &keys[1..];

        // 首先检查当前层级的键是否存在
        if let Some(value) = self.data.get(key) {
            // 如果还有剩余的键，且当前值是表格类型，则继续深入
            if !remaining_keys.is_empty() {
                if let toml::Value::Table(table) = value {
                    return HardwareConfig::get_table_value(table, remaining_keys);
                }
            } else {
                // 如果没有更多的键，则返回当前值
                return Some(value);
            }
        }
        None // 找不到键时返回 None
    }

    // 处理表格类型的值
    fn get_table_value<'a>(
        table: &'a toml::map::Map<String, toml::Value>,
        keys: &[&str],
    ) -> Option<&'a toml::Value> {
        if let Some(value) = table.get(keys[0]) {
            // 如果还有剩余的键，递归处理
            if keys.len() > 1 {
                if let toml::Value::Table(ref inner_table) = value {
                    return HardwareConfig::get_table_value(inner_table, &keys[1..]);
                }
            } else {
                // 如果没有更多的键，则返回当前值
                return Some(value);
            }
        }
        None // 如果找不到该键，则返回 None
    }

    /// 设置或更新配置项（支持嵌套）
    pub fn set_value(&mut self, key: &str, new_value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<&str> = key.split('.').collect(); // 修正这里
        self.update(|toml_value| {
            if let Value::Table(table) = toml_value {
                Self::modify_nested_value(table, &keys, Some(new_value.clone()));
            }
        })
    }

    /// 删除某个配置项（支持嵌套）
    pub fn remove_value(&mut self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<&str> = key.split('.').collect(); // 修正这里
        self.update(|toml_value| {
            if let Value::Table(table) = toml_value {
                Self::modify_nested_value(table, &keys, None);
            }
        })
    }

    fn modify_nested_value(table: &mut toml::map::Map<String, Value>, keys: &[&str], new_value: Option<Value>) {
        if let Some((first_key, rest_keys)) = keys.split_first() {
            if rest_keys.is_empty() {
                // **修改或删除最终的值**
                if let Some(value) = new_value {
                    table.insert(first_key.to_string(), value);
                } else {
                    table.remove(*first_key);
                }
            } else {
                // **递归进入嵌套 Table**
                match table.entry(first_key.to_string()) {
                    toml::map::Entry::Occupied(mut entry) => {
                        if let Value::Table(inner_table) = entry.get_mut() {
                            Self::modify_nested_value(inner_table, rest_keys, new_value);
                        }
                    }
                    toml::map::Entry::Vacant(entry) => {  // ✅ 这里确保使用的是 toml::map::Entry
                        if new_value.is_some() {
                            let mut new_table = toml::map::Map::new();
                            Self::modify_nested_value(&mut new_table, rest_keys, new_value);
                            entry.insert(Value::Table(new_table));
                        }
                    }
                }
            }
        }
    }
    
}

// █████╗ ██╗      ██████╗  ██████╗ 
// ██╔══██╗██║     ██╔════╝ ██╔═══██╗
// ███████║██║     ██║  ███╗██║   ██║
// ██╔══██║██║     ██║   ██║██║   ██║
// ██║  ██║███████╗╚██████╔╝╚██████╔╝
// ╚═╝  ╚═╝╚══════╝ ╚═════╝  ╚═════╝ 
                                  
/// **AlgoConfig 子配置**
impl AlgoConfig {
    pub fn load(file_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let data = if let Ok(content) = fs::read_to_string(&file_path) {
            let parsed: HashMap<String, Value> = toml::from_str(&content)?;  
            parsed  // 直接返回 HashMap
        } else {
            HashMap::new()  // 确保 data 是 HashMap
        };

        Ok(AlgoConfig {
            data,
            file_path: Some(file_path),
        })
    }

    pub fn update<F>(&mut self,modify: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut toml::Value),
    {
        if let Some(file_path) = &self.file_path {
            // **关键修正**
            // 需要把 `HashMap<String, Value>` 转换为 `toml::map::Map<String, Value>`
            let mut toml_value = Value::Table(self.data.clone().into_iter().collect());  
            modify(&mut toml_value);

            // **修正 toml_value.as_table() 类型**
            self.data = toml_value.as_table().unwrap().clone().into_iter().collect();

            let new_toml = toml::to_string_pretty(&self.data)?;
            fs::write(file_path, new_toml)?;
        }
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<&toml::Value> {
        // 分割 key 来处理嵌套的表
        let keys: Vec<&str> = key.split('.').collect();

        // 递归访问嵌套的表
        self.get_nested_value(&keys)
    }

    fn get_nested_value(&self, keys: &[&str]) -> Option<&toml::Value> {
        if keys.is_empty() {
            return None; // 如果没有更多的键了，返回 None
        }

        let key = keys[0];
        let remaining_keys = &keys[1..];

        // 首先检查当前层级的键是否存在
        if let Some(value) = self.data.get(key) {
            // 如果还有剩余的键，且当前值是表格类型，则继续深入
            if !remaining_keys.is_empty() {
                if let toml::Value::Table(table) = value {
                    return HardwareConfig::get_table_value(table, remaining_keys);
                }
            } else {
                // 如果没有更多的键，则返回当前值
                return Some(value);
            }
        }
        None // 找不到键时返回 None
    }

    // 处理表格类型的值
    fn get_table_value<'a>(
        table: &'a toml::map::Map<String, toml::Value>,
        keys: &[&str],
    ) -> Option<&'a toml::Value> {
        if let Some(value) = table.get(keys[0]) {
            // 如果还有剩余的键，递归处理
            if keys.len() > 1 {
                if let toml::Value::Table(ref inner_table) = value {
                    return HardwareConfig::get_table_value(inner_table, &keys[1..]);
                }
            } else {
                // 如果没有更多的键，则返回当前值
                return Some(value);
            }
        }
        None // 如果找不到该键，则返回 None
    }

    /// 设置或更新配置项（支持嵌套）
    pub fn set_value(&mut self, key: &str, new_value: toml::Value) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<&str> = key.split('.').collect(); // 修正这里
        self.update(|toml_value| {
            if let Value::Table(table) = toml_value {
                Self::modify_nested_value(table, &keys, Some(new_value.clone()));
            }
        })
    }

    /// 删除某个配置项（支持嵌套）
    pub fn remove_value(&mut self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let keys: Vec<&str> = key.split('.').collect(); // 修正这里
        self.update(|toml_value| {
            if let Value::Table(table) = toml_value {
                Self::modify_nested_value(table, &keys, None);
            }
        })
    }

    fn modify_nested_value(table: &mut toml::map::Map<String, Value>, keys: &[&str], new_value: Option<Value>) {
        if let Some((first_key, rest_keys)) = keys.split_first() {
            if rest_keys.is_empty() {
                // **修改或删除最终的值**
                if let Some(value) = new_value {
                    table.insert(first_key.to_string(), value);
                } else {
                    table.remove(*first_key);
                }
            } else {
                // **递归进入嵌套 Table**
                match table.entry(first_key.to_string()) {
                    toml::map::Entry::Occupied(mut entry) => {
                        if let Value::Table(inner_table) = entry.get_mut() {
                            Self::modify_nested_value(inner_table, rest_keys, new_value);
                        }
                    }
                    toml::map::Entry::Vacant(entry) => {  // ✅ 这里确保使用的是 toml::map::Entry
                        if new_value.is_some() {
                            let mut new_table = toml::map::Map::new();
                            Self::modify_nested_value(&mut new_table, rest_keys, new_value);
                            entry.insert(Value::Table(new_table));
                        }
                    }
                }
            }
        }
    }
}