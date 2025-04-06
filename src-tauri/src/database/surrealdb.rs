#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration, sync::Mutex};

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

}

fn get_database_ip_port(){


}

pub async fn start_database_task(){






}




pub async fn start_database_connection(){


}


pub async fn stop_database_connection(){


}



