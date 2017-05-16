extern crate crypto;

use std::fs;
use std::io::{Read, ErrorKind};
use std::env;
use std::io;
use std::path;
use crypto::sha2;
use crypto::digest::Digest;

fn main() {
	for arg in env::args().skip(1) {
        let path = path::Path::new(&arg);
        let mut buf = [0; 32768];
        let mut hasher = sha2::Sha256::new();

        match walk(&path, &mut |dir| -> io::Result<()> {
            if dir.file_type()?.is_dir() {
                return Ok(())
            }

            let path = dir.path();
            let mut f = fs::File::open(&path)?;
            
            hasher.reset();
            loop {
                let len = match f.read(&mut buf) {
                    Ok(0) => break,
                    Ok(len) => len,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
                hasher.input(&buf[..len]);
            }

            println!("{}\t{}", hasher.result_str(), path.display());
            return Ok(())

        }) {
			Ok(()) => {},
			Err(e) => println!("error listing {}: {}", arg, e),
		}
	}
}

fn walk(dir: &path::Path, cb: &mut FnMut(&fs::DirEntry) -> io::Result<()>) -> io::Result<()> {
	if !dir.is_dir() {
        return Ok(())
	}

	for entry in fs::read_dir(dir)? {
		let entry = entry?;
        cb(&entry)?;

        match walk(&entry.path(), cb) {
            Ok(()) => {},
            r => return r,
        }
	}

	Ok(())
}
