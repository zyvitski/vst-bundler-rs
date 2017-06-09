use std::fs::*;
use std::io::prelude::*;
pub fn create_pkginfo(name: &str) -> bool {
    let contents = "BNDL????";
    if let Ok(mut f) = File::create(format!("{}/Contents/PkgInfo", name)) {
        let _ = f.write(contents.as_bytes());
        true
    } else {
        false
    }
}
