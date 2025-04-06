#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::sync::Arc;
use tokio::{sync::mpsc, time::Duration, sync::Mutex};


use lazy_static::lazy_static;
lazy_static! {
    pub static ref SURREALDB_TX: Arc<Mutex<Option<mpsc::Sender<SurrealdbRequest>>>> = Arc::new(Mutex::new(None));
  }


pub enum SurrealdbRequest{

    // 写入log

    // 写入run_log


    // 写入某个hole结果


    // 写入某个型号

    // log查询


    // run_log查询


    // 某个零件结果查询

    // 所有零件查询

    
}

