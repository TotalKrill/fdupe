extern crate md5;

use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::fmt;
use std::path;


#[derive(PartialEq, Eq)]
pub struct HashedPath {
    pub path: String,
    pub hash: md5::Digest,
}

impl fmt::Display for HashedPath
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:x}", self.path, self.hash)
    }
}

pub fn hash_path( _path : &path::Path ) -> Result< HashedPath, std::io::Error>
{
    let f = File::open( _path )?;

    let mut s = String::new();

    BufReader::new( f ).read_to_string(&mut s)?;

    let bytes =  s.into_bytes();

    let hash = md5::compute( bytes );

    let res = HashedPath { path: _path.display().to_string(), hash: hash };
    Ok(res)
}
