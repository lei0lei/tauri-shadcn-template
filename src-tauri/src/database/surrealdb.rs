#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration, sync::Mutex};
use surrealdb::{Surreal, engine::remote::ws::Client, opt::auth::Root};


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

fn get_database_ip_port(){

  Some("ws://127.0.0.1:8011".to_string())
}

pub async fn start_database_task(addr: String, mut rx: mpsc::Receiver<SurrealdbRequest>){


}


pub async fn start_database_connect(addr:String)-> Result<bool, String>{
  let (tx, rx) = mpsc::channel::<ModbusRequest>(32);
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
      println!("PLC ip_port 格式错误: {}", plc_addr);
    }

  })

}


pub async fn stop_database_connection(){


}



