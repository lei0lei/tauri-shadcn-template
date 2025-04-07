#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration, sync::Mutex};
use surrealdb::{Surreal, engine::remote::ws::Client, opt::auth::Root};
// use surrealdb::dbs::{Db};
// use surrealdb::opt::{ConnectOpts};
use surrealdb::engine::remote::ws::Ws;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use chrono::{DateTime, Utc};
use tokio_modbus::prelude::*;
use surrealdb::sql::{Datetime};
use tokio::sync::oneshot;


// ███████╗██╗   ██╗██████╗ ██████╗ ███████╗ █████╗ ██╗     
// ██╔════╝██║   ██║██╔══██╗██╔══██╗██╔════╝██╔══██╗██║     
// ███████╗██║   ██║██████╔╝██████╔╝█████╗  ███████║██║     
// ╚════██║██║   ██║██╔══██╗██╔══██╗██╔══╝  ██╔══██║██║     
// ███████║╚██████╔╝██║  ██║██║  ██║███████╗██║  ██║███████╗
// ╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚══════╝     
use lazy_static::lazy_static;
lazy_static! {
    pub static ref SURREALDB_TX: Arc<Mutex<Option<mpsc::Sender<SurrealdbRequest>>>> = Arc::new(Mutex::new(None));
  }

#[derive(Debug, Serialize, Deserialize)]
pub struct LogRecord{
  artifact_id: i32,
  artifact_name: String,
  level: i32,
  message: String,
  role: i32,
  update_time: Datetime,
}

impl LogRecord {
  // new 方法，允许通过它来创建一个新的 LogRecord 实例
  pub fn new(
    artifact_id: i32,
    artifact_name: String,
    level: i32,
    message: String,
    role: i32,
    update_time: DateTime<Utc>,
  ) -> Self {
    LogRecord {
        artifact_id,
        artifact_name,
        level,
        message,
        role,
        update_time: update_time.into(),  // 将 DateTime 转换为字符串
    }
  }

  // 你可以添加一些 getter 方法来访问字段，但仍然保持字段本身私有
  pub fn artifact_id(&self) -> i32 {
      self.artifact_id
  }

  pub fn artifact_name(&self) -> &str {
      &self.artifact_name
  }

  pub fn level(&self) -> i32 {
      self.level
  }

  pub fn message(&self) -> &str {
      &self.message
  }

  pub fn role(&self) -> i32 {
      self.role
  }

  pub fn update_time(&self) -> &Datetime {
      &self.update_time
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunLogRecord{
  level: i32,
  message: String,
  role: i32,
  update_time: DateTime<Utc>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ArtifactRecord{
  artifact_id: i32,
  artifact_name: String,
  artifact_type: i32,
  result: Option<bool>,
  update_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoleRecord{
  action1: Vec<f32>,
  action2: Vec<f32>,
  action3: serde_json::Value,
  action4: serde_json::Value,
  artifact_id: i32,
  artifact_name: String,
  depth: f32,
  depth_result: Option<bool>,
  diameter: f32,
  diameter_orig_path: String,
  diameter_result_path: String,
  dimeter_result: f32,
  face_id: i32,
  hole_id: i32,
  luowen_orig_path: String,
  luowen_result: Option<bool>,
  luowen_result_path: String,
  update_time: DateTime<Utc>,
}


pub enum SurrealdbRequest{

    // 写入log
    InsertLog(LogRecord, oneshot::Sender<Result<u16, String>>),

    // 写入run_log
    InsertRunLog(RunLogRecord, oneshot::Sender<Result<u16, String>>),

    // 写入某个hole结果
    InsertHole(HoleRecord, oneshot::Sender<Result<u16, String>>),

    // 写入某个型号
    InsertArtifact(ArtifactRecord, oneshot::Sender<Result<u16, String>>),
    // log查询
    SearchLog(oneshot::Sender<Result<u16, String>>),

    // run_log查询
    SearchRunLog(oneshot::Sender<Result<u16, String>>),

    // 某个零件结果查询
    SearchArtifact(oneshot::Sender<Result<u16, String>>),

    // 查询某个零件的所有结果
    SearchArtifactHoles(oneshot::Sender<Result<u16, String>>),

    // 所有零件查询
    SearchAllArtifact(oneshot::Sender<Result<u16, String>>),

    // 更新产品检测结果
    UpdateArtifactResult(oneshot::Sender<Result<u16, String>>),

    // 获取零件id
    GetNewArtifactId(oneshot::Sender<Result<u16, String>>),

    STOP(oneshot::Sender<Result<(), String>>),

}

fn get_database_ip_port()->Option<String>{

  Some("127.0.0.1:8011".to_string())
}

pub async fn start_database_task(addr: String, mut rx: mpsc::Receiver<SurrealdbRequest>){
  
  let db = match Surreal::new::<Ws>("127.0.0.1:8011").await {
    Ok(db) => db,
    Err(e) => {
        println!("Failed to connect to the database: {}", e);
        return; // 如果连接失败，直接返回
        }
    };

    // 使用 signin 和数据库操作
    if let Err(e) = db.signin(Root {
        username: "lei0lei",
        password: "12345678",
    }).await {
        println!("Signin failed: {}", e);
        return; // 如果登录失败，返回
    }

    // 使用命名空间和数据库
    if let Err(e) = db.use_ns("rs").use_db("artifact").await {
        println!("Database selection failed: {}", e);
        return; // 如果数据库选择失败，返回
    }

    let log_record = LogRecord::new(
          1, 
          "Artifact1".to_string(), 
          2, 
          "This is a log message.".to_string(), 
          3, 
          Utc::now()
      );
      let result: Result<Option<LogRecord>, surrealdb::Error> = db
      .create("Log") // 创建一个 "Log" 表的记录
      .content(log_record) // 使用 LogRecord 内容
      .await;

  match result {
      Ok(Some(record)) => {
          println!("Log inserted successfully: {:?}", record);
      }
      Ok(None) => {
          println!("Log creation failed: No result returned");
      }
      Err(e) => {
          println!("Error inserting log: {}", e);
      }
  }

  println!("数据库连接成功");
  while let Some(request) = rx.recv().await {
    match request {
        SurrealdbRequest::InsertLog(record, resp_tx) => {
          println!("line 191");
          let result: Result<Option<LogRecord>, surrealdb::Error> = db.create("Log").content(record).await;
          let _ = resp_tx.send(result.map(|_| 1).map_err(|e| e.to_string()));
        }
        SurrealdbRequest::InsertRunLog(record, resp_tx) => {
          let result: Result<Option<RunLogRecord>, surrealdb::Error> = db.create("Run_log").content(record).await;
          let _ = resp_tx.send(result.map(|_| 1).map_err(|e| e.to_string()));
        }
        SurrealdbRequest::InsertHole(record, resp_tx) => {
            let result: Result<Option<HoleRecord>, surrealdb::Error> = db.create("Hole_library").content(record).await;
            let _ = resp_tx.send(result.map(|_| 1).map_err(|e| e.to_string()));
        }
        SurrealdbRequest::InsertArtifact(record, resp_tx) => {
            let result: Result<Option<ArtifactRecord>, surrealdb::Error> = db.create("Artifact_library").content(record).await;
            let _ = resp_tx.send(result.map(|_| 1).map_err(|e| e.to_string()));
        }
        _ => {}
    }
  }

}


pub async fn start_database_connect(addr:String)-> Result<bool, String>{
  let (tx, rx) = mpsc::channel::<SurrealdbRequest>(32);
  let tx = Arc::new(Mutex::new(Some(tx))); // 用 Mutex 包装 tx
  *SURREALDB_TX.lock().await = Some(tx.lock().await.clone().unwrap());
  tokio::spawn(start_database_task(addr.clone(), rx));
  Ok(true) 
}

pub fn start_database_connection(){
  tauri::async_runtime::spawn(async {

    println!("database: 创建数据库连接...");
    if let Some(addr) = get_database_ip_port() {
      let _ = start_database_connect(addr).await;
      println!("database:数据库连接创建完毕");
    } else {
      println!("PLC ip_port 格式错误");
    }
  });
}


pub async fn stop_database_connection(){


}



