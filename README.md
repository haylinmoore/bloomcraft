# BloomCraft

A high-performance string search service using a custom AMQ (Approximate Membership Query) filter to optimize substring searches for Infinite Craft elements.

## Overview

BloomCraft uses a custom AMQ filter designed specifically for substring search optimization. The AMQ acts as a fast pre-filter that can definitively say "no match" or "maybe match", dramatically reducing the number of actual substring comparisons needed for large datasets.

## How It Works

### The Custom AMQ Algorithm

At its core, BloomCraft uses a 64-bit AMQ filter for each element in the dataset:

1. **Pre-processing Phase**:
   - For each element in the dataset, we create a 64-bit AMQ filter.
   - For each character in the element, we map it to a bit position: `(char_utf32_value) % 64`.
   - We set the corresponding bit in the 64-bit filter.
   - These AMQ filters are created once during startup and stored in memory.

2. **Search Phase**:
   - When a search query comes in, we create an AMQ filter for the query string.
   - We then check each element's pre-calculated AMQ filter to see if it contains all the bits from the query's filter.
   - If an element's filter doesn't contain all the bits from the query's filter, we know with 100% certainty that the element cannot contain the query as a substring.
   - Only elements that pass this AMQ filter check need to undergo an actual substring search.

### Why This Works

The key insight is that if a character appears in a substring, it must also appear in the containing string. By tracking character presence in our AMQ filters, we can quickly eliminate elements that couldn't possibly match.

- **False positives**: An element may pass the AMQ filter check but not actually contain the substring.
- **False negatives**: Never occur - if the AMQ says an element doesn't match, it definitely doesn't.

## API Endpoints

### GET /stats

Returns statistics about the dataset.

Response:
```json
{
  "elements_count": 500000,
  "amq_filters_count": 500000,
  "bit_width": 64,
  "etag": 12345678901234567
}
```

### GET /search?q=query

Performs an AMQ filter search and returns the count of potential matches.

Response:
```json
{
  "matches": 1200,
  "total": 500000,
  "percentage": 0.24,
  "elapsed_ms": 5,
  "etag": 12345678901234567
}
```

### GET /search/paginated?q=query&start=0&limit=10&etag=12345678901234567

Returns paginated results with substring verification.

- `q`: The search query
- `start`: Starting index (default: 0)
- `limit`: Maximum number of results to return (default: 10)
- `etag`: Optional etag to verify dataset hasn't changed

Response:
```json
{
  "matches": ["Element1", "Element2", "Element3"],
  "total_matches": 1200,
  "next_index": 10,
  "elapsed_ms": 8,
  "etag": 12345678901234567
}
```

### GET /troublemakers

Returns a list of elements that have AMQ filters with all bits set to 1. These elements will always be queried in the AMQ filter algorithm since they contain all possible character hashes.

Response:
```json
{
  "troublemakers": [
    {
      "name": "ElementWithAllBits",
      "emoji": "🔍"
    }
  ],
  "count": 1,
  "percentage": 0.0002,
  "elapsed_ns": 1200000,
  "etag": 12345678901234567
}
```

## Running the Server

```
cargo run -- <elements_file> <host> <port>
```

For example:
```
cargo run -- elements_subset.csv 127.0.0.1 8080
```
