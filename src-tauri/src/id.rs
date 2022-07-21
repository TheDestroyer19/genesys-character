use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Serialize, Deserialize};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub(crate) struct Id(usize);

impl Id {
    pub fn new() -> Id {
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}