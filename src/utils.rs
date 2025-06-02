use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

use crate::models::Element;

pub fn read_elements<P: AsRef<Path>>(path: P) -> io::Result<Vec<Element>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut elements = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let mut parts = Vec::new();
            let mut current_part = String::new();
            let mut escaped = false;

            for c in line.chars() {
                if c == '\x01' {
                    escaped = !escaped;
                    continue;
                }

                if c == '\t' && !escaped {
                    parts.push(current_part);
                    current_part = String::new();
                } else {
                    current_part.push(c);
                }
            }

            parts.push(current_part);

            if parts.len() == 3 {
                if let Ok(id) = parts[0].parse::<u32>() {
                    elements.push(Element {
                        id,
                        name: std::mem::take(&mut parts[1]),
                        emoji: std::mem::take(&mut parts[2]),
                    });
                }
            }
        }
    }

    Ok(elements)
}

pub fn format_duration(duration: Duration) -> String {
    let total_ns = duration.as_nanos();
    if total_ns < 1_000 {
        format!("{}ns", total_ns)
    } else if total_ns < 1_000_000 {
        format!("{:.2}µs", total_ns as f64 / 1_000.0)
    } else if total_ns < 1_000_000_000 {
        format!("{:.2}ms", total_ns as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", duration.as_secs_f64())
    }
}

pub fn generate_etag(elements: &[Element]) -> u32 {
    let mut hasher = DefaultHasher::new();

    elements.len().hash(&mut hasher);

    if !elements.is_empty() {
        elements[0].id.hash(&mut hasher);
        elements[0].name.hash(&mut hasher);
        elements[elements.len() - 1].id.hash(&mut hasher);
        elements[elements.len() - 1].name.hash(&mut hasher);
    }

    let sample_size = std::cmp::min(10, elements.len());
    for i in 0..sample_size {
        elements[i].id.hash(&mut hasher);
        elements[i].name.hash(&mut hasher);
    }

    hasher.finish() as u32
}