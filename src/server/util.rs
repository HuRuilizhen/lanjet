use mime_guess::{Mime, from_path};

pub fn file_icon(path: String) -> &'static str {
    let mime: Mime = from_path(path).first_or_octet_stream();

    match (mime.type_().as_str(), mime.subtype().as_str()) {
        // Images
        ("image", _) => "🖼️",

        // Video
        ("video", _) => "🎞️",

        // Audio
        ("audio", _) => "🎵",

        // PDF
        ("application", "pdf") => "📄",

        // Compressed / archives
        ("application", "zip") => "🗜️",
        ("application", "x-7z-compressed") => "🗜️",
        ("application", "x-rar-compressed") => "🗜️",
        ("application", "x-tar") => "🗜️",
        ("application", "gzip") => "🗜️",

        // Text / code / JSON / HTML / XML
        ("text", _) => "📄",
        ("application", "json") => "📄",
        ("application", "xml") => "📄",
        ("application", "html") => "📄",
        ("application", "javascript") => "📄",
        ("application", "x-yaml") => "📄",

        // Fonts
        ("font", _) => "🔤",

        // Binary / executables
        ("application", "octet-stream") => "⚙️",

        // Everything else
        _ => "📄",
    }
}
