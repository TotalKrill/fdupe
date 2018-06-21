use std::path::PathBuf;
use std::cmp::Ordering;
use std::cmp::max;
use std::cell::RefCell;
use std::io;

use Metadata;
use Hasher;

#[derive(Debug)]
/// File content is efficiently compared using this struct's `PartialOrd` implementation
pub struct FileContent {
    path: PathBuf,
    metadata: Metadata,
    /// Hashes of content, calculated incrementally
    hashes: RefCell<Hasher>,
}

impl FileContent {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Result<Self, io::Error> {
        let path = path.into();
        let m = Metadata::from_path(&path)?;
        Ok(Self::new(path, m))
    }

    pub fn new<P: Into<PathBuf>>(path: P, metadata: Metadata) -> Self {
        let path = path.into();
        FileContent {
            path: path,
            metadata: metadata,
            hashes: RefCell::new(Hasher::new()),
        }
    }
}

impl Eq for FileContent {
}

impl PartialEq for FileContent {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).map(|o|o == Ordering::Equal).unwrap_or(false)
    }
}

impl Ord for FileContent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Error handling here sucks")
    }
}

/// That does the bulk of hasing and comparisons
impl PartialOrd for FileContent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Different file sizes mean they're obviously different.
        // Also different devices mean they're not the same as far as we're concerned
        // (since search is intended for hardlinking and hardlinking only works within the same device).
        let cmp = self.metadata.cmp(&other.metadata);
        if cmp != Ordering::Equal {
            return Some(cmp);
        }

        // Fast pointer comparison
        if self as *const _ == other as *const _ {
            return Some(Ordering::Equal);
        }

        let mut hashes1 = self.hashes.borrow_mut();
        let mut hashes2 = other.hashes.borrow_mut();

        hashes1.compare(&mut *hashes2, self.metadata.size, &self.path, &other.path).ok()
    }
}

