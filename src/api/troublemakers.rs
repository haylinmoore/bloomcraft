use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Mutex;
use std::time::Instant;

use crate::models::{AppState, ElementMatch};

#[derive(Serialize)]
pub struct TroublemakersResponse {
    pub troublemakers: Vec<ElementMatch>,
    pub count: usize,
    pub percentage: f64,
    pub elapsed_ns: u64,
    pub etag: u32,
}

pub async fn get_troublemakers(
    data: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let start = Instant::now();
    
    let app_state = data.lock().unwrap();
    let total_elements = app_state.elements.len();
    
    let mut troublemakers = Vec::new();
    
    for i in 0..total_elements {
        if app_state.bloom_filters[i].is_all_ones() {
            troublemakers.push(ElementMatch {
                name: app_state.elements[i].name.clone(),
                emoji: app_state.elements[i].emoji.clone(),
            });
        }
    }
    
    let count = troublemakers.len();
    let percentage = if total_elements > 0 {
        (count as f64 / total_elements as f64) * 100.0
    } else {
        0.0
    };
    
    let elapsed = start.elapsed();
    
    HttpResponse::Ok().json(TroublemakersResponse {
        troublemakers,
        count,
        percentage,
        elapsed_ns: elapsed.as_nanos() as u64,
        etag: app_state.etag,
    })
}