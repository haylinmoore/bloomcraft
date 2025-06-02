use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Instant;

use crate::bloomfilter::BloomFilter;
use crate::models::AppState;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub matches: usize,
    pub total: usize,
    pub percentage: f64,
    pub elapsed_ns: u64,
    pub etag: u32,
}

pub async fn count_matches(
    query: web::Query<SearchQuery>,
    data: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let start = Instant::now();
    let query_str = &query.q;

    let app_state = data.lock().unwrap();
    let query_filter = BloomFilter::from_string(query_str);
    let query_length = query_str.len();

    let length_start_index = app_state
        .length_index_map
        .get(&query_length)
        .copied()
        .unwrap_or(0);

    let mut matches_count = 0;
    let total = app_state.bloom_filters.len();

    for i in length_start_index..total {
        if app_state.bloom_filters[i].contains(&query_filter) {
            matches_count += 1;
        }
    }

    let elapsed = start.elapsed();
    let percentage = if total > length_start_index {
        (matches_count as f64 / (total - length_start_index) as f64) * 100.0
    } else {
        0.0
    };

    HttpResponse::Ok().json(SearchResponse {
        matches: matches_count,
        total: app_state.elements.len(),
        percentage,
        elapsed_ns: elapsed.as_nanos() as u64,
        etag: app_state.etag,
    })
}