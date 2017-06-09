use plist::create_info_plist;
use pkginfo::create_pkginfo;
use std::fs;
use std::path::Path;
use error::*;
use clap;

pub fn create_support_files(name: &str) -> Result<(), VstBundlerError> {
    let pkginfo = create_pkginfo(name);
    let plist = create_info_plist(name);
    if pkginfo.is_err() {
        pkginfo
    } else if plist.is_err() {
        plist
    } else {
        Ok(())
    }
}

//input -> validate paths -> prepare folders -> copy files [-> install]

pub fn create_folder_structure(name: &str) -> Result<(), VstBundlerError> {
    let _ = fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{}/Contents/MacOS/", name));
    let _ = fs::DirBuilder::new()
        .recursive(true)
        .create(format!("{}/Contents/Resources/", name));
    create_support_files(name)
}

pub fn verify_path_is_dylib(path: &str) -> Result<(), VstBundlerError> {
    let libpath = ::std::path::Path::new(path);
    if libpath.extension().unwrap() != "dylib" {
        Err(VstBundlerError::ExpectedDylib)
    } else {
        Ok(())
    }

}
pub fn install_lib_in_bundle(path: &str, vstname: &str, name: &str) -> Result<(), VstBundlerError> {
    if let Err(what) = fs::copy(path, format!("{}//Contents/MacOS/{}", vstname, name)) {
        Err(VstBundlerError::CopyError(what))
    } else {
        Ok(())
    }
}
pub fn install_resource_in_bundle(r: &str, vstname: &str) -> Result<(), VstBundlerError> {
    let md = fs::metadata(r).unwrap();
    let file_name = Path::new(r).file_name().unwrap().to_str().unwrap();
    if md.is_file() {
        if let Err(what) = fs::copy(r, format!("{}/Contents/Resources/{}", vstname, file_name)) {
            Err(VstBundlerError::CopyError(what))
        } else {
            Ok(())
        }
    } else if md.is_dir() {
        let options = ::fs_extra::dir::CopyOptions::new();
        use fs_extra::dir::copy;
        if let Err(what) = copy(r,
                                format!("{}/Contents/Resources/{}", vstname, file_name),
                                &options) {
            Err(VstBundlerError::CopyErrorFsExtra(what))
        } else {
            Ok(())
        }
    } else {
        unreachable!()
    }
}
pub fn install_resources(rs: clap::Values, vstname: &str) -> Result<(), VstBundlerError> {
    for r in rs {
        let r = install_resource_in_bundle(r, vstname);
        if r.is_err() {
            return r;
        }
    }
    Ok(())
}
pub fn install_vst(path: &str, vstname: &str) -> Result<(), VstBundlerError> {
    let options = ::fs_extra::dir::CopyOptions::new();
    use fs_extra::dir::copy;
    if let Err(what) = copy(vstname, format!("{}/{}", path, vstname), &options) {
        Err(VstBundlerError::CopyErrorFsExtra(what))
    } else {
        Ok(())
    }
}
