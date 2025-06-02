mod api;
mod bloomfilter;
mod models;
mod utils;

use actix_web::{web, App, HttpServer};
use std::collections::BTreeMap;
use std::env;
use std::io;
use std::sync::Mutex;
use std::time::Instant;

use bloomfilter::BloomFilter;
use models::AppState;
use utils::{format_duration, generate_etag, read_elements};

async fn run_server(elements_file: &str, host: &str, port: u16) -> io::Result<()> {
    println!("Initializing server with 64-bit bloom filters");

    let start = Instant::now();
    let mut elements = read_elements(elements_file)?;
    let read_duration = start.elapsed();
    println!(
        "Read {} elements from {} in {}",
        elements.len(),
        elements_file,
        format_duration(read_duration)
    );

    let start = Instant::now();
    elements.sort_by_key(|element| element.name.len());
    let sort_duration = start.elapsed();
    println!(
        "Sorted elements by length in {}",
        format_duration(sort_duration)
    );

    let start = Instant::now();
    let mut length_index_map = BTreeMap::new();
    let mut current_length = 0;

    for (i, element) in elements.iter().enumerate() {
        let length = element.name.len();
        if length > current_length {
            for len in (current_length + 1)..=length {
                length_index_map.insert(len, i);
            }
            current_length = length;
        }
    }

    let map_creation_duration = start.elapsed();
    println!(
        "Created length index map with {} entries in {}",
        length_index_map.len(),
        format_duration(map_creation_duration)
    );

    let etag = generate_etag(&elements);
    println!("Generated dataset etag: {}", etag);

    let start = Instant::now();
    let mut bloom_filters = Vec::with_capacity(elements.len());
    for element in &elements {
        let filter = BloomFilter::from_string(&element.name);
        bloom_filters.push(filter);
    }
    let filter_creation_duration = start.elapsed();
    println!(
        "Created {} bloom filters in {}",
        bloom_filters.len(),
        format_duration(filter_creation_duration)
    );

    let app_state = web::Data::new(Mutex::new(AppState {
        elements,
        bloom_filters,
        length_index_map,
        etag,
    }));

    println!("Starting HTTP server at http://{}:{}", host, port);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/stats", web::get().to(api::get_stats))
            .route("/search", web::get().to(api::count_matches))
            .route("/search/paginated", web::get().to(api::paginated_search))
            .route("/random", web::get().to(api::get_random_elements))
            .route("/troublemakers", web::get().to(api::get_troublemakers))
    })
    .bind((host, port))?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let elements_file = if args.len() > 1 {
        args[1].as_str()
    } else {
        "elements.tsv"
    };

    let host = if args.len() > 2 {
        args[2].as_str()
    } else {
        "127.0.0.1"
    };

    let port = if args.len() > 3 {
        args[3].parse::<u16>().unwrap_or(8080)
    } else {
        8080
    };

    run_server(elements_file, host, port).await
}