use std::fs::*;
use std::io::prelude::*;
use error::*;
pub fn create_pkginfo(name: &str) -> Result<(), VstBundlerError> {
    let contents = "BNDL????";
    if let Ok(mut f) = File::create(format!("{}/Contents/PkgInfo", name)) {
        let _ = f.write(contents.as_bytes());
        Ok(())
    } else {
        Err(VstBundlerError::PkgInfoCreateFailure)
    }
}
