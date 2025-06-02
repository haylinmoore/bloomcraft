use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Instant;

use crate::bloomfilter::BloomFilter;
use crate::models::{AppState, ElementMatch};

#[derive(Deserialize)]
pub struct PaginatedSearchQuery {
    pub q: String,
    pub start: Option<usize>,
    pub limit: Option<usize>,
    pub etag: Option<u32>,
}

#[derive(Serialize)]
pub struct PaginatedSearchResponse {
    pub matches: Vec<ElementMatch>,
    pub bloom_hits: u64,
    pub next_index: usize,
    pub elapsed_ns: u64,
    pub etag: u32,
}

pub async fn paginated_search(
    query: web::Query<PaginatedSearchQuery>,
    data: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let start = Instant::now();
    let query_str = &query.q;
    let start_index = query.start.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let app_state = data.lock().unwrap();

    if let Some(client_etag) = query.etag {
        if client_etag != app_state.etag {
            return HttpResponse::PreconditionFailed().json(serde_json::json!({
                "error": "Dataset has changed since your last request",
                "current_etag": app_state.etag
            }));
        }
    }

    let query_filter = BloomFilter::from_string(query_str);
    let query_length = query_str.len();

    let length_start_index = app_state
        .length_index_map
        .get(&query_length)
        .copied()
        .unwrap_or(0);

    let effective_start_index = std::cmp::max(start_index, length_start_index);

    let lowercase_query = query_str.to_lowercase();
    let mut matches = Vec::with_capacity(limit);
    let next_index;

    let total = app_state.bloom_filters.len();
    let mut i = effective_start_index;
    let mut bloom_hits = 0;

    while i < total && matches.len() < limit {
        if app_state.bloom_filters[i].contains(&query_filter) {
            bloom_hits += 1;
            if app_state.elements[i]
                .name
                .to_lowercase()
                .contains(&lowercase_query)
            {
                matches.push(ElementMatch {
                    name: app_state.elements[i].name.clone(),
                    emoji: app_state.elements[i].emoji.clone(),
                });
            }
        }
        i += 1;
    }

    if i < total {
        next_index = i;
    } else {
        next_index = total;
    }

    let elapsed = start.elapsed();

    HttpResponse::Ok().json(PaginatedSearchResponse {
        matches,
        next_index,
        bloom_hits,
        elapsed_ns: elapsed.as_nanos() as u64,
        etag: app_state.etag,
    })
}