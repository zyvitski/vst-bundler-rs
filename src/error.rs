use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VstBundlerError {
    PkgInfoCreateFailure,
    PlistCreateFailure,
    ExpectedDylib,
    CopyError(::std::io::Error),
    CopyErrorFsExtra(::fs_extra::error::Error),
}
impl fmt::Display for VstBundlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            VstBundlerError::PkgInfoCreateFailure => write!(f, "Failed to create PkgInfo."),
            VstBundlerError::PlistCreateFailure => write!(f, "Failed to create Info.plist."),
            VstBundlerError::ExpectedDylib => {
                write!(f,
                       "Expected a .dylib file for libpath argument (-l, --libpath).")
            }
            VstBundlerError::CopyError(ref what) => write!(f, "{}", what),
            VstBundlerError::CopyErrorFsExtra(ref what) => write!(f, "{}", what),
        }
    }
}

impl Error for VstBundlerError {
    fn description(&self) -> &str {
        "Vst Bundler Error"
    }
}
