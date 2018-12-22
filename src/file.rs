use std::cell::RefCell;
use std::cmp::Ordering;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use std::fs::Metadata;

use hasher::Hasher;

#[derive(Debug)]
/// File content is efficiently compared using this struct's `PartialOrd` implementation
pub struct FileContent {
    pub path: PathBuf,
    metadata: Metadata,
    /// Hashes of content, calculated incrementally
    hashes: RefCell<Hasher>,
}

impl FileContent {
    pub fn from_path(path: &Path) -> Result<Self, io::Error> {
        //let path = path.canonicalize()?;
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
    pub fn len(&self) -> u64 {
        self.metadata.len()
    }
}

impl Eq for FileContent {}

impl PartialEq for FileContent {
    fn eq(&self, other: &Self) -> bool {
        let retval = self
            .partial_cmp(other)
            .map(|o| o == Ordering::Equal)
            .unwrap_or(false);

        // Same canonical path mean they are the same file, for the usecase of this software
        // the same file shall not be treated as a duplicate.
        let selfcanonical = self.path.canonicalize().unwrap();

        let othercanonical = other.path.canonicalize().unwrap();

        if othercanonical == selfcanonical {
            return false;
        }
        retval
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

        //Fast pointer comparison
        if self as *const _ == other as *const _ {
            return Some(Ordering::Equal);
        }

        let mut hashes1 = self.hashes.borrow_mut();
        let mut hashes2 = other.hashes.borrow_mut();

        hashes1
            .compare(&mut *hashes2, self.metadata.len(), &self.path, &other.path)
            .ok()
    }
}
