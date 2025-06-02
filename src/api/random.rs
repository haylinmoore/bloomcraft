use actix_web::{web, HttpResponse, Responder};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;
use std::time::Instant;

use crate::models::{AppState, ElementMatch};

#[derive(Deserialize)]
pub struct RandomQuery {
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct RandomResponse {
    pub elements: Vec<ElementMatch>,
    pub elapsed_ns: u64,
    pub etag: u32,
}

pub async fn get_random_elements(
    query: web::Query<RandomQuery>,
    data: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let start = Instant::now();
    let limit = query.limit.unwrap_or(10);

    let app_state = data.lock().unwrap();
    let total_elements = app_state.elements.len();
    
    let mut rng = thread_rng();
    let mut seen_indices = HashSet::with_capacity(limit);
    let mut elements = Vec::with_capacity(limit);
    
    let actual_limit = std::cmp::min(limit, total_elements);
    
    while elements.len() < actual_limit {
        let idx = rng.gen_range(0..total_elements);
        
        if !seen_indices.insert(idx) {
            continue;
        }
        
        elements.push(ElementMatch {
            name: app_state.elements[idx].name.clone(),
            emoji: app_state.elements[idx].emoji.clone(),
        });
    }

    let elapsed = start.elapsed();

    HttpResponse::Ok().json(RandomResponse {
        elements,
        elapsed_ns: elapsed.as_nanos() as u64,
        etag: app_state.etag,
    })
}