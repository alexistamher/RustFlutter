use std::{collections::HashMap, sync::Mutex};

use crate::models::Task;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/tasks")]
async fn index(data: Data<Mutex<HashMap<u32, Task>>>) -> HttpResponse {
    let tasks_data = data.lock().unwrap();
    let mut values: Vec<&Task> = (*tasks_data).values().clone().collect();
    values.sort_by(|a, b| a.id.cmp(&b.id));
    HttpResponse::Ok().json(values)
}

#[get("/tasks/{id}")]
async fn get_task_by_id(data: Data<Mutex<HashMap<u32, Task>>>, path: Path<(u32,)>) -> HttpResponse {
    let data = data.lock().unwrap();

    let (id,): (u32,) = path.into_inner();
    if let Some(task) = (*data).get(&id) {
        return HttpResponse::Ok().json(task);
    }
    HttpResponse::NotFound().finish()
}

#[post("/tasks")]
async fn add(data: Data<Mutex<HashMap<u32, Task>>>, body: Json<Task>) -> HttpResponse {
    let mut data = data.lock().unwrap();
    if *(&body.id) == 0 {
        let max = (*data).iter().max_by(|a, b| a.0.cmp(b.0));
        let id: u32 = if max.is_none() { 1 } else { max.unwrap().0 + 1 };
        (*data).insert(id, Task { id: id, ..body.0 });
        return HttpResponse::Created().finish();
    }
    (*data).insert((*body).id, body.0);
    HttpResponse::Created().finish()
}

#[put("/tasks/{id}")]
async fn replace(
    data: Data<Mutex<HashMap<u32, Task>>>,
    body: Json<Task>,
    path: Path<(u32,)>,
) -> HttpResponse {
    let (id,) = path.into_inner();
    let mut data = data.lock().unwrap();
    if id == 0 || !(*data).contains_key(&id) {
        return HttpResponse::NotFound().finish();
    }
    (*data).insert(id, Task { id: id, ..body.0 });
    HttpResponse::Ok().finish()
}

#[delete("/tasks/{id}")]
async fn delete_task(data: Data<Mutex<HashMap<u32, Task>>>, path: Path<(u32,)>) -> HttpResponse {
    let (id,) = path.into_inner();
    let mut data = data.lock().unwrap();
    match (*data).remove_entry(&id) {
        Some(_) => HttpResponse::NoContent().finish(),
        None => HttpResponse::NotFound().finish(),
    }
}
