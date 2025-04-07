#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration, sync::Mutex};
use surrealdb::{Surreal, engine::remote::ws::Client, opt::auth::Root};
// use surrealdb::dbs::{Db};
// use surrealdb::opt::{ConnectOpts};
use surrealdb::engine::remote::ws::Ws;

use surrealdb::sql::Thing;

use tokio_modbus::prelude::*;

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


pub enum SurrealdbRequest{

    // 写入log
    InsertLog(oneshot::Sender<Result<u16, String>>),
    // 写入run_log
    InsertRunLog(oneshot::Sender<Result<u16, String>>),

    // 写入某个hole结果
    InsertHole(oneshot::Sender<Result<u16, String>>),

    // 写入某个型号
    InsertArtifact(oneshot::Sender<Result<u16, String>>),
    // log查询
    SearchLog(oneshot::Sender<Result<u16, String>>),

    // run_log查询
    SearchRunLog(oneshot::Sender<Result<u16, String>>),

    // 某个零件结果查询
    SearchArtifact(oneshot::Sender<Result<u16, String>>),
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
    if let Err(e) = db.use_ns("test").use_db("test").await {
        println!("Database selection failed: {}", e);
        return; // 如果数据库选择失败，返回
    }


  // let db_file = "file://D:/database/mydb.db";  // 数据库文件路径
  // let schema_file = "D:/database/surrealdb_schema";  // schema 文件路径

  // let opts = ConnectOpts::new().set_url(&addr).set_schema(schema_file.to_string());
  // let db = Surreal::new(db_file)
  //     .await
  //     .map_err(|e| e.to_string())
  //     .unwrap();
  
  // // 切换到命名空间和数据库
  // db.use_ns("namespace")
  //     .use_db("mydb")
  //     .await
  //     .map_err(|e| e.to_string())
  //     .unwrap();

  println!("数据库连接成功");
  while let Some(request) = rx.recv().await {
    match request {
        SurrealdbRequest::InsertLog(sender) => {
            // 在这里处理 InsertLog 请求
            sender.send(Ok(1)).unwrap();
        }
        SurrealdbRequest::SearchLog(sender) => {
            // 在这里处理 SearchLog 请求
            sender.send(Ok(42)).unwrap();
        }
        // 处理其他请求
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



