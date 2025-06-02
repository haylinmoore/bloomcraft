use serde::Serialize;
use std::collections::BTreeMap;

use crate::bloomfilter::BloomFilter;

#[derive(Debug, Clone, Serialize)]
pub struct Element {
    pub id: u32,
    pub name: String,
    pub emoji: String,
}

pub struct AppState {
    pub elements: Vec<Element>,
    pub bloom_filters: Vec<BloomFilter>,
    pub length_index_map: BTreeMap<usize, usize>,
    pub etag: u32,
}

#[derive(Serialize)]
pub struct ElementMatch {
    pub name: String,
    pub emoji: String,
}