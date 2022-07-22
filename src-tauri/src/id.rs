use std::{
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::{Deserialize, Serialize};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub(crate) struct Id(usize);

impl Id {
    pub fn new() -> Id {
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
