mod models;
mod services;
use std::{collections::HashMap, sync::Mutex};

use actix_web::{web::Data, App, HttpServer};

use models::Task;

use crate::services::{add, delete_task, get_task_by_id, index, replace};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tasks: Mutex<HashMap<u32, Task>> = Mutex::new(HashMap::new());
    let data = Data::new(tasks);
    println!("🚀 Servidor iniciado en http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(add)
            .service(get_task_by_id)
            .service(replace)
            .service(delete_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
