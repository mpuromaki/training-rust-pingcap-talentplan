#![deny(missing_docs)]

//! Basic key-value store.
//!
//! # Examples
//! ```rust
//! let mut store = kvs::KvStore::new();
//!
//! store.set("key1".to_owned(), "value1".to_owned());
//! assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
//! ```
pub use kv::KvStore;

mod kv;
