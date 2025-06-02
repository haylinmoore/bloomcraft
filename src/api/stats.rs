use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Mutex;

use crate::models::AppState;

#[derive(Serialize)]
pub struct Stats {
    pub elements_count: usize,
    pub bloom_filters_count: usize,
    pub bit_width: u32,
    pub length_index_map_size: usize,
    pub min_element_length: usize,
    pub max_element_length: usize,
    pub etag: u32,
}

pub async fn get_stats(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let app_state = data.lock().unwrap();

    let min_length = app_state
        .length_index_map
        .keys()
        .next()
        .copied()
        .unwrap_or(0);
    let max_length = app_state
        .length_index_map
        .keys()
        .rev()
        .next()
        .copied()
        .unwrap_or(0);

    HttpResponse::Ok().json(Stats {
        elements_count: app_state.elements.len(),
        bloom_filters_count: app_state.bloom_filters.len(),
        bit_width: 64,
        length_index_map_size: app_state.length_index_map.len(),
        min_element_length: min_length,
        max_element_length: max_length,
        etag: app_state.etag,
    })
}