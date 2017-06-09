use std::fs::*;
use std::io::prelude::*;

use rand;
use rand::Rng;
pub fn create_info_plist(name: &str) -> bool{
    let mut rng = rand::thread_rng();
    let plist_contents = format!(r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>

    <key>CFBundleExecutable</key>
    <string>{}</string>

    <key>CFBundleGetInfoString</key>
    <string>vst</string>

    <key>CFBundleIconFile</key>
    <string></string>

    <key>CFBundleIdentifier</key>
    <string>com.rust-vst2.{}</string>

    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>

    <key>CFBundleName</key>
    <string>{}</string>

    <key>CFBundlePackageType</key>
    <string>BNDL</string>

    <key>CFBundleVersion</key>
    <string>1.0</string>

    <key>CFBundleSignature</key>
    <string>{}</string>

    <key>CSResourcesFileMapped</key>
    <string></string>

    </dict>
    </plist>"##,name,name,name,rng.gen::<u16>()%9999u16);
    if let Ok(mut f) = File::create(format!("{}/Contents/Info.plist",name)) {
        let _ = f.write(plist_contents.as_bytes()).unwrap();
        true
    } else {
        false
    }
}
