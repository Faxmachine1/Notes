mod note_handler;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateParams {
    title: String,
    body: String,
}

#[derive(Deserialize)]
pub struct ReadParams {
    key: String,
}

// Создать новую заметку
pub async fn create(params: web::Query<CreateParams>) -> impl Responder {
    let title = &params.title;
    let body = &params.body;

    let json = note_handler::Note::create(title.to_string(), body.to_string());

    HttpResponse::Ok().json(json)
}

// Прочитать заметку
pub async fn read(params: web::Query<ReadParams>) -> impl Responder {
    let key = &params.key;

    let json = note_handler::Note::read(key.to_string()).unwrap();

    HttpResponse::Ok().json(json)
}