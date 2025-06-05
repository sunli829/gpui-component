use askama::Template;

/// ```askama
/// <?xml version="1.0" encoding="UTF-8"?>
/// <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
/// <plist version="1.0">
/// <dict>
///   <key>CFBundleDevelopmentRegion</key>
///   <string>en</string>
///   <key>CFBundleDisplayName</key>
///   <string>{{ name }}</string>
///   <key>CFBundleExecutable</key>
///   <string>{{ name }}</string>
///   <key>CFBundleIdentifier</key>
///   <string>{{ identifier }}</string>
///   <key>CFBundleInfoDictionaryVersion</key>
///   <string>6.0</string>
///   <key>CFBundleName</key>
///   <string>{{ name }}</string>
///   <key>CFBundlePackageType</key>
///   <string>APPL</string>
///   <key>CFBundleVersion</key>
///   <string></string>
///   <key>CFBundleShortVersionString</key>
///   <string></string>
/// </dict>
/// </plist>
/// ```
#[derive(Template)]
#[template(ext = "txt", in_doc = true)]
pub(crate) struct InfoPlist {
    pub(crate) name: String,
    pub(crate) identifier: String,
}
