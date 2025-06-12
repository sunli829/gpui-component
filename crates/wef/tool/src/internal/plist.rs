use askama::Template;
use serde::Deserialize;

/// ```askama
/// <?xml version="1.0" encoding="UTF-8"?>
/// <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
/// <plist version="1.0">
/// <dict>
///   <key>CFBundleDevelopmentRegion</key>
///   {% if let Some(region) = region %}
///   <string>{{ region }}</string>
///   {% else %}
///   <string>en</string>
///   {% endif %}
///   <key>CFBundleDisplayName</key>
///   {% if let Some(display_name) = display_name %}
///   <string>{{ display_name | e }}</string>
///   {% else %}
///   <string>{{ name | e }}</string>
///   {% endif %}
///   <key>CFBundleExecutable</key>
///   {% if let Some(executable_name) = executable_name %}
///   <string>{{ executable_name | e }}</string>
///   {% else %}
///   <string>{{ name | e }}</string>
///   {% endif %}
///   <key>CFBundleIdentifier</key>
///   <string>{{ identifier | e }}</string>
///   <key>CFBundleInfoDictionaryVersion</key>
///   <string>6.0</string>
///   <key>CFBundleName</key>
///   <string>{{ name | e }}</string>
///   <key>CFBundlePackageType</key>
///   <string>APPL</string>
///   <key>CFBundleVersion</key>
///   {% if let Some(bundle_version) = bundle_version %}
///   <string>{{ bundle_version | e }}</string>
///   {% else %}
///   <string></string>
///   {% endif %}
///   <key>CFBundleShortVersionString</key>
///   {% if let Some(bundle_short_version) = bundle_short_version %}
///   <string>{{ bundle_short_version | e }}</string>
///   {% else %}
///   <string></string>
///   {% endif %}
///   {% if let Some(category) = category %}
///   <key>LSApplicationCategoryType</key>
///   <string>{{ category | e }}</string>
///   {% endif %}
///   {% if let Some(osx_minimum_system_version) = osx_minimum_system_version %}
///   <key>LSMinimumSystemVersion</key>
///   <string>{{ osx_minimum_system_version | e }}</string>
///   {% endif %}
///   {% if osx_url_schemes.len() > 0 %}
///   <key>CFBundleURLTypes</key>
///   <array>
///     <dict>
///        <key>CFBundleURLName</key>
///        <string>{{ name | e }}</string>
///        <key>CFBundleTypeRole</key>
///        <string>Viewer</string>
///        <key>CFBundleURLSchemes</key>
///        <array>
///        {% for scheme in osx_url_schemes %}
///          <string>{{ scheme | e }}</string>
///        {% endfor %}
///        </array>
///     <dict>
///   </array>
///   {% endif %}
/// </dict>
/// </plist>
/// ```
#[derive(Debug, Template, Deserialize, PartialEq, Eq)]
#[template(ext = "xml", in_doc = true)]
pub(crate) struct InfoPlist {
    pub(crate) name: String,
    pub(crate) identifier: String,
    pub(crate) display_name: Option<String>,
    pub(crate) executable_name: Option<String>,
    pub(crate) region: Option<String>,
    pub(crate) bundle_version: Option<String>,
    pub(crate) bundle_short_version: Option<String>,
    #[serde(default)]
    pub(crate) icons: Vec<String>,
    pub(crate) category: Option<String>,
    pub(crate) osx_minimum_system_version: Option<String>,
    #[serde(default)]
    pub(crate) osx_url_schemes: Vec<String>,
}

impl InfoPlist {
    pub(crate) fn new(name: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            identifier: identifier.into(),
            display_name: None,
            executable_name: None,
            region: None,
            bundle_version: None,
            bundle_short_version: None,
            icons: vec![],
            category: None,
            osx_minimum_system_version: None,
            osx_url_schemes: vec![],
        }
    }
}
