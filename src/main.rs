extern crate clap;
extern crate rand;
use clap::{Arg, App};

mod plist;
mod pkginfo;
use plist::create_info_plist;
use pkginfo::create_pkginfo;

use std::fs;

fn create_support_files(name: &str) -> bool {
    create_info_plist(name) && create_pkginfo(name)
}

fn main() {
    // --name | -n = name of bundle
    // --lib | -l = path to dylib
    // --outdir -o = install dir
    let arg_parser = App::new("VST-Bundler")
        .version("0.1.0")
        .author("Alex Z. <alexander.zywicki@gmail.com>")
        .about("Bundles VST Plugins")
        .arg(Arg::with_name("name")
                 .short("n")
                 .long("name")
                 .value_name("NAME")
                 .help("The name of the VST Plugin")
                 .required(true)
                 .takes_value(true))
        .arg(Arg::with_name("libpath")
                 .short("l")
                 .long("libpath")
                 .value_name("LIBPATH")
                 .help("The path to the plugin dylib")
                 .required(true))
        .arg(Arg::with_name("installdir")
                 .short("i")
                 .long("installdir")
                 .value_name("INSTALLDIR")
                 .required(false)
                 .help("The location to install the bundle to"))
        .get_matches();

    let plugname = arg_parser.value_of("name");
    let libpath = arg_parser.value_of("libpath");
    let installdir = arg_parser.value_of("installdir");

    if let (Some(name), Some(path)) = (plugname, libpath) {
        let vstname = format!("{}.vst", name);
        let _ = fs::DirBuilder::new()
            .recursive(true)
            .create(format!("{}/Contents/MacOS/", vstname.as_str()));
        create_support_files(vstname.as_str());
        let _ = fs::copy(path,
                         format!("{}//Contents/MacOS/{}", vstname.as_str(), name));

        if let Some(idir) = installdir {
            let _ = fs::soft_link(vstname.as_str(), format!("{}/{}", idir, vstname.as_str()));
        }
    } else {
        unreachable!();
    }
}
