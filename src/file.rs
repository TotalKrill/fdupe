use std::path::PathBuf;
use std::cmp::Ordering;
use std::cmp::max;
use std::cell::RefCell;
use std::io;

use Hasher;

use std::fs::Metadata;
use std;

#[derive(Debug)]
/// File content is efficiently compared using this struct's `PartialOrd` implementation
pub struct FileContent {
    path: PathBuf,
    metadata: Metadata,
    /// Hashes of content, calculated incrementally
    hashes: RefCell<Hasher>,
}

impl FileContent {
    pub fn from_path(path: &PathBuf) -> Result<Self, io::Error> {
        let m = std::fs::metadata(&path)?;
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
        let cmp = self.metadata.len().cmp(&other.metadata.len());
        if cmp != Ordering::Equal {
            return Some(cmp);
        }

        // Fast pointer comparison
        if self as *const _ == other as *const _ {
            return Some(Ordering::Equal);
        }

        let mut hashes1 = self.hashes.borrow_mut();
        let mut hashes2 = other.hashes.borrow_mut();

        hashes1.compare(&mut *hashes2, self.metadata.len(), &self.path, &other.path).ok()
    }
}

