use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::Date;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::SystemTime;
use chrono::prelude::*;
use chrono::offset::LocalResult;

struct AppState {
    todolist: Mutex<Vec<Todo>>,
    max_size: Mutex<usize>,
}

//static MAX_ID: Mutex<usize> = Mutex::new(Default::default());

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    username: String,
    description: String,
    time: String,
    date: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct CreateEntryData {
    username: String,
    description: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct UpdateEntryData {
    username: String,
    description: String,
}

#[get("/")]
async fn index() -> String {
    "normal".to_string()
}

#[get("/todolist/data")]
async fn get_all_data(d: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(d.todolist.lock().unwrap().to_vec())
}

#[post("/todolist/data")]
async fn create_data(d: web::Data<AppState>, param_obj: web::Json<CreateEntryData>) -> impl Responder {
    let mut todolist = d.todolist.lock().unwrap();
    // let max_id: usize = *MAX_ID.lock().unwrap() + (1 as usize);
    // let max_id2:usize = *MAX_ID.lock().unwrap() + (1 as usize);
    // *MAX_ID.lock().unwrap() = max_id;
    let mut maxsize = *d.max_size.lock().unwrap() + (1 as usize);
    *d.max_size.lock().unwrap() = maxsize;

    todolist.push(Todo {
        id: maxsize,
        username: param_obj.username.clone(),
        description: param_obj.description.clone() ,
        time: Utc::now().timestamp().to_string(),
        date: Utc::now().date_naive().to_string(),
    });
    HttpResponse::Ok().json(todolist.to_vec())
}

#[put("/todolist/data/{id}")]
async fn update_entry(d: web::Data<AppState>, path: web::Path<usize>, param_obj: web::Json<UpdateEntryData>) -> impl Responder {
    let id = path.into_inner();     //pulls value from path
    let mut todolist = d.todolist.lock().unwrap();

    for i in 0..todolist.len() {
        if todolist[i].id == id {
            todolist[i].username = param_obj.username.clone();
            todolist[i].description = param_obj.description.clone();
            break;
        }
    }
    HttpResponse::Ok().json(todolist.to_vec())
}

#[delete("/todolist/data/{id}")]
async fn delete_entry(d: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let mut todolist = d.todolist.lock().unwrap();
    let id = path.into_inner();

    *todolist = todolist.to_vec().into_iter().filter(|x| x.id != id).collect();

    HttpResponse::Ok().json(todolist.to_vec())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = web::Data::new(AppState {
        todolist: Mutex::new(vec![]),
        max_size: Mutex::new(Default::default()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(get_all_data)
            .service(create_data)
            .service(update_entry)
            .service(delete_entry)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}