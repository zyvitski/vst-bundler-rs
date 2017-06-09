extern crate clap;
extern crate rand;
extern crate fs_extra;

mod plist;
mod pkginfo;
mod arg_parser;
mod error;
mod file_utils;

use arg_parser::create_arg_parser;
use file_utils::*;

const NAME: &str = "name";
const LIBPATH: &str = "libpath";
const INSTALLDIR: &str = "installdir";
const RESOURCE: &str = "resource";

fn main() {
    let arg_parser = create_arg_parser();
    let plugname = arg_parser.value_of(NAME);
    let libpath = arg_parser.value_of(LIBPATH);
    let installdir = arg_parser.value_of(INSTALLDIR);
    let resources = arg_parser.values_of(RESOURCE);

    if let (Some(name), Some(path)) = (plugname, libpath) {
        let vstname: &str = &format!("{}.vst", name);
        verify_path_is_dylib(path).unwrap();
        create_folder_structure(vstname).unwrap();
        install_lib_in_bundle(path, vstname, name).unwrap();
        if let Some(rs) = resources {
            install_resources(rs, vstname).unwrap();
        }
        if let Some(idir) = installdir {
            install_vst(idir, vstname).unwrap();
        }
    }
}
