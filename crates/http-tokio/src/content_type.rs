use std::fmt::Display;

pub enum ContentType {
    /// **Text**
    /// 
    /// _smime type_: text/plain
    TextPlain,
    /// **Hyper Text Markup Language**
    /// 
    /// _mime type_: text/html
    /// 
    /// _extension_: .html
    Html,
    /// **AAC audio**
    /// 
    /// _mime type_: audio/aac
    /// 
    /// _extension_: .aac
    Aac,
    /// **AbiWord document**
    /// 
    /// _mime type_: application/x-abiword
    /// 
    /// _extension_: .abw
    Abw,
    /// **Animated Portable Network Graphics (APNG) image**
    /// 
    /// _mime type_: image/apng
    /// 
    /// _extension_: .apng
    Apng,
    /// **Archive document (multiple files embedded)**
    /// 
    /// _mime type_: application/x-freearc
    /// 
    /// _extension_: .arc
    Arc,
    /// **AVIF image**
    /// 
    /// _mime type_: image/avif
    /// 
    /// _extension_: .avif
    Avif,
    /// **AVI: Audio Video Interleave**
    /// 
    /// _mime type_: video/x-msvideo
    /// 
    /// _extension_: .avi
    Avi,
    /// **Amazon Kindle eBook format**
    /// 
    /// _mime type_: application/vnd.amazon.ebook
    /// 
    /// _extension_: .azw
    Azw,
    /// **Any kind of binary data**
    /// 
    /// _mime type_: application/octet-stream
    /// 
    /// _extension_: .bin
    Bin,
    /// **Windows OS/2 Bitmap Graphics**
    /// 
    /// _mime type_: image/bmp
    /// 
    /// _extension_: .bmp
    Bmp,
    /// **BZip archive**
    /// 
    /// _mime type_: application/x-bzip
    /// 
    /// _extension_: .bz
    Bz,
    /// **BZip2 archive**
    /// 
    /// _mime type_: application/x-bzip2
    /// 
    /// _extension_: .bz2
    Bz2,
    /// **CD audio**
    /// 
    /// _mime type_: application/x-cdf
    /// 
    /// _extension_: .cda
    Cda,
    /// **C-Shell script**
    /// 
    /// _mime type_: application/x-csh
    /// 
    /// _extension_: .csh
    Csh,
    /// **Cascading Style Sheets (CSS)**
    /// 
    /// _mime type_: text/css
    /// 
    /// _extension_: .css
    Css,
    /// **Comma-separated values (CSV)**
    /// 
    /// _mime type_: text/csv
    /// 
    /// _extension_: .csv
    Csv,
    /// **Microsoft Word**
    /// 
    /// _mime type_: application/msword
    /// 
    /// _extension_: .doc
    Doc,
    /// **Microsoft Word (OpenXML)**
    /// 
    /// _mime type_: application/vnd.openxmlformats-officedocument.wordprocessingml.document
    /// 
    /// _extension_: .docx
    Docx,
    /// **MS Embedded OpenType fonts**
    /// 
    /// _mime type_: application/vnd.ms-fontobject
    /// 
    /// _extension_: .eot
    Eot,
    /// **Electronic publication (EPUB)**
    /// 
    /// _mime type_: application/epub+zip
    /// 
    /// _extension_: .epub
    Epub,
    /// **GZip Compressed Archive**
    /// 
    /// _mime type_: application/gzip
    /// 
    /// _extension_: .gz
    Gz,
    /// **Graphics Interchange Format (GIF)**
    /// 
    /// _mime type_: image/gif
    /// 
    /// _extension_: .gif
    Gif,
    /// **HyperText Markup Language (HTML)**
    /// 
    /// _mime type_: text/html
    /// 
    /// _extension_: .htm, .html
    Htm,
    /// **Icon format**
    /// 
    /// _mime type_: image/vnd.microsoft.icon
    /// 
    /// _extension_: .ico
    Ico,
    /// **iCalendar format**
    /// 
    /// _mime type_: text/calendar
    /// 
    /// _extension_: .ics
    Ics,
    /// **Java Archive (JAR)**
    /// 
    /// _mime type_: application/java-archive
    /// 
    /// _extension_: .jar
    Jar,
    /// **JPEG images**
    /// 
    /// _mime type_: image/jpeg
    /// 
    /// _extension_: .jpeg, .jpg
    Jpeg,
    /// **JavaScript**
    /// 
    /// _mime type_: text/javascript (Specifications: HTML and RFC 9239)
    /// 
    /// _extension_: .js
    Js,
    /// **        JSON format**
    /// 
    /// _mime type_: application/json
    /// 
    /// _extension_: .json
    Json,
    /// **    JSON-LD format**
    /// 
    /// _mime type_: application/ld+json
    /// 
    /// _extension_: .jsonld
    Jsonld,
    /// **Musical Instrument Digital Interface (MIDI)**
    /// 
    /// _mime type_: audio/midi, audio/x-midi
    /// 
    /// _extension_: .mid, .midi
    Mid,
    /// **        JavaScript module**
    /// 
    /// _mime type_: text/javascript
    /// 
    /// _extension_: .mjs
    Mjs,
    /// **MP3 audio**
    /// 
    /// _mime type_: audio/mpeg
    /// 
    /// _extension_: .mp3
    Mp3,
    /// **MP4 video**
    /// 
    /// _mime type_: video/mp4
    /// 
    /// _extension_: .mp4
    Mp4,
    /// **MPEG Video**
    /// 
    /// _mime type_: video/mpeg
    /// 
    /// _extension_: .mpeg
    Mpeg,
    /// **Apple Installer Package**
    /// 
    /// _mime type_: application/vnd.apple.installer+xml
    /// 
    /// _extension_: .mpkg
    Mpkg,
    /// **OpenDocument presentation document**
    /// 
    /// _mime type_: application/vnd.oasis.opendocument.presentation
    /// 
    /// _extension_: .odp
    Odp,
    /// **OpenDocument spreadsheet document**
    /// 
    /// _mime type_: application/vnd.oasis.opendocument.spreadsheet
    /// 
    /// _extension_: .ods
    Ods,
    /// **OpenDocument text document**
    /// 
    /// _mime type_: application/vnd.oasis.opendocument.text
    /// 
    /// _extension_: .odt
    Odt,
    /// **OGG audio**
    /// 
    /// _mime type_: audio/ogg
    /// 
    /// _extension_: .oga
    Oga,
    /// **OGG video**
    /// 
    /// _mime type_: video/ogg
    /// 
    /// _extension_: .ogv
    Ogv,
    /// **OGG**
    /// 
    /// _mime type_: application/ogg
    /// 
    /// _extension_: .ogx
    Ogx,
    /// **Opus audio**
    /// 
    /// _mime type_: audio/opus
    /// 
    /// _extension_: .opus
    Opus,
    /// **OpenType font**
    /// 
    /// _mime type_: font/otf
    /// 
    /// _extension_: .otf
    Otf,
    /// **Portable Network Graphics**
    /// 
    /// _mime type_: image/png
    /// 
    /// _extension_: .png
    Png,
    /// **Adobe Portable Document Format (PDF)**
    /// 
    /// _mime type_: application/pdf
    /// 
    /// _extension_: .pdf
    Pdf,
    /// **Hypertext Preprocessor (Personal Home Page)**
    /// 
    /// _mime type_: application/x-httpd-php
    /// 
    /// _extension_: .php
    Php,
    /// **Microsoft PowerPoint**
    /// 
    /// _mime type_: application/vnd.ms-powerpoint
    /// 
    /// _extension_: .ppt
    Ppt,
    /// **Microsoft PowerPoint (OpenXML)**
    /// 
    /// _mime type_: application/vnd.openxmlformats-officedocument.presentationml.presentation
    /// 
    /// _extension_: .pptx
    Pptx,
    /// **RAR archive**
    /// 
    /// _mime type_: application/vnd.rar
    /// 
    /// _extension_: .rar
    Rar,
    /// **Rich Text Format (RTF)**
    /// 
    /// _mime type_: application/rtf
    /// 
    /// _extension_: .rtf
    Rtf,
    /// **Bourne shell script**
    /// 
    /// _mime type_: application/x-sh
    /// 
    /// _extension_: .sh
    Sh,
    /// **Scalable Vector Graphics (SVG)**
    /// 
    /// _mime type_: image/svg+xml
    /// 
    /// _extension_: .svg
    Svg,
    /// **Tape Archive (TAR)**
    /// 
    /// _mime type_: application/x-tar
    /// 
    /// _extension_: .tar
    Tar,
    /// **Tagged Image File Format (TIFF)**
    /// 
    /// _mime type_: image/tiff
    /// 
    /// _extension_: .tif, .tiff
    Tif,
    /// **MPEG transport stream**
    /// 
    /// _mime type_: video/mp2t
    /// 
    /// _extension_: .ts
    Ts,
    /// **TrueType Font**
    /// 
    /// _mime type_: font/ttf
    /// 
    /// _extension_: .ttf
    Ttf,
    /// **Text, (generally ASCII or ISO 8859-n)**
    /// 
    /// _mime type_: text/plain
    /// 
    /// _extension_: .txt
    Txt,
    /// **Microsoft Visio**
    /// 
    /// _mime type_: application/vnd.visio
    /// 
    /// _extension_: .vsd
    Vsd,
    /// **Waveform Audio Format**
    /// 
    /// _mime type_: audio/wav
    /// 
    /// _extension_: .wav
    Wav,
    /// **WEBM audio**
    /// 
    /// _mime type_: audio/webm
    /// 
    /// _extension_: .weba
    Weba,
    /// **WEBM video**
    /// 
    /// _mime type_: video/webm
    /// 
    /// _extension_: .webm
    Webm,
    /// **WEBP image**
    /// 
    /// _mime type_: image/webp
    /// 
    /// _extension_: .webp
    Webp,
    /// **Web Open Font Format (WOFF)**
    /// 
    /// _mime type_: font/woff
    /// 
    /// _extension_: .woff
    Woff,
    /// **Web Open Font Format (WOFF)**
    /// 
    /// _mime type_: font/woff2
    /// 
    /// _extension_: .woff2
    Woff2,
    /// **XHTML**
    /// 
    /// _mime type_: application/xhtml+xml
    /// 
    /// _extension_: .xhtml
    Xhtml,
    /// **Microsoft Excel**
    /// 
    /// _mime type_: application/vnd.ms-excel
    /// 
    /// _extension_: .xls
    Xls,
    /// **Microsoft Excel (OpenXML)**
    /// 
    /// _mime type_: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
    /// 
    /// _extension_: .xlsx
    Xlsx,
    /// **XML**
    /// 
    /// _mime type_: application/xml is recommended as of RFC 7303 (section 4.1), but text/xml is still used sometimes. You can assign a specific MIME type to a file with .xml extension depending on how its contents are meant to be interpreted. For instance, an Atom feed is application/atom+xml, but application/xml serves as a valid default.
    /// 
    /// _extension_: .xml
    Xml,
    /// **XUL**
    /// 
    /// _mime type_: application/vnd.mozilla.xul+xml
    /// 
    /// _extension_: .xul
    Xul,
    /// **ZIP archive**
    /// 
    /// _mime type_: application/zip
    /// 
    /// _extension_: .zip
    Zip,
    /// **3GPP audio/video container**
    /// 
    /// _mime type_: video/3gpp; audio/3gpp if it doesn't contain video
    /// 
    /// _extension_: .3gp
    _3gp,
    /// **3GPP2 audio/video container**
    /// 
    /// _mime type_: video/3gpp2; audio/3gpp2 if it doesn't contain video
    /// 
    /// _extension_: .3g2
    _3g2,
    /// **7-zip archive**
    /// 
    /// _mime type_: application/x-7z-compressed
    /// 
    /// _extension_: .7z
    _7z,
}


  
impl ContentType {
    pub fn from_ext(extension: &str) -> ContentType {
        match extension {
            "aac" => ContentType::Aac,
            "abw" => ContentType::Abw,
            "apng" => ContentType::Apng,
            "arc" => ContentType::Arc,
            "avif" => ContentType::Avif,
            "avi" => ContentType::Avi,
            "azw" => ContentType::Azw,
            "bin" => ContentType::Bin,
            "bmp" => ContentType::Bmp,
            "bz" => ContentType::Bz,
            "bz2" => ContentType::Bz2,
            "cda" => ContentType::Cda,
            "csh" => ContentType::Csh,
            "css" => ContentType::Css,
            "csv" => ContentType::Csv,
            "doc" => ContentType::Doc,
            "docx" => ContentType::Docx,
            "eot" => ContentType::Eot,
            "epub" => ContentType::Epub,
            "gz" => ContentType::Gz,
            "gif" => ContentType::Gif,
            "htm" => ContentType::Htm,
            "html" => ContentType::Html,
            "ico" => ContentType::Ico,
            "ics" => ContentType::Ics,
            "jar" => ContentType::Jar,
            "jpeg" => ContentType::Jpeg,
            "js" => ContentType::Js,
            "json" => ContentType::Json,
            "jsonld" => ContentType::Jsonld,
            "mid" => ContentType::Mid,
            "mjs" => ContentType::Mjs,
            "mp3" => ContentType::Mp3,
            "mp4" => ContentType::Mp4,
            "mpeg" => ContentType::Mpeg,
            "mpkg" => ContentType::Mpkg,
            "odp" => ContentType::Odp,
            "ods" => ContentType::Ods,
            "odt" => ContentType::Odt,
            "oga" => ContentType::Oga,
            "ogv" => ContentType::Ogv,
            "ogx" => ContentType::Ogx,
            "opus" => ContentType::Opus,
            "otf" => ContentType::Otf,
            "png" => ContentType::Png,
            "pdf" => ContentType::Pdf,
            "php" => ContentType::Php,
            "ppt" => ContentType::Ppt,
            "pptx" => ContentType::Pptx,
            "rar" => ContentType::Rar,
            "rtf" => ContentType::Rtf,
            "sh" => ContentType::Sh,
            "svg" => ContentType::Svg,
            "tar" => ContentType::Tar,
            "tif" => ContentType::Tif,
            "ts" => ContentType::Ts,
            "ttf" => ContentType::Ttf,
            "txt" => ContentType::Txt,
            "vsd" => ContentType::Vsd,
            "wav" => ContentType::Wav,
            "weba" => ContentType::Weba,
            "webm" => ContentType::Webm,
            "webp" => ContentType::Webp,
            "woff" => ContentType::Woff,
            "woff2" => ContentType::Woff2,
            "xhtml" => ContentType::Xhtml,
            "xls" => ContentType::Xls,
            "xlsx" => ContentType::Xlsx,
            "xml" => ContentType::Xml,
            "xul" => ContentType::Xul,
            "zip" => ContentType::Zip,
            "3gp" => ContentType::_3gp,
            "3g2" => ContentType::_3g2,
            "7z" => ContentType::_7z,
            _ => ContentType::TextPlain,
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let str = match self {
            ContentType::TextPlain => "text/plain",
            ContentType::Aac => "audio/aac",
            ContentType::Abw => "application/x-abiword",
            ContentType::Apng => "image/apng",
            ContentType::Arc => "application/x-freearc",
            ContentType::Avif => "image/avif",
            ContentType::Avi => "video/x-msvideo",
            ContentType::Azw => "application/vnd.amazon.ebook",
            ContentType::Bin => "application/octet-stream",
            ContentType::Bmp => "image/bmp",
            ContentType::Bz => "application/x-bzip",
            ContentType::Bz2 => "application/x-bzip2",
            ContentType::Cda => "application/x-cdf",
            ContentType::Csh => "application/x-csh",
            ContentType::Css => "text/css",
            ContentType::Csv => "text/csv",
            ContentType::Doc => "application/msword",
            ContentType::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            ContentType::Eot => "application/vnd.ms-fontobject",
            ContentType::Epub => "application/epub+zip",
            ContentType::Gz => "application/gzip",
            ContentType::Gif => "image/gif",
            ContentType::Htm => "text/html",
            ContentType::Html => "text/html",
            ContentType::Ico => "image/vnd.microsoft.icon",
            ContentType::Ics => "text/calendar",
            ContentType::Jar => "application/java-archive",
            ContentType::Jpeg => "image/jpeg",
            ContentType::Js => "text/javascript",
            ContentType::Json => "application/json",
            ContentType::Jsonld => "application/ld+json",
            ContentType::Mid => "audio/midi, audio/x-midi",
            ContentType::Mjs => "text/javascript",
            ContentType::Mp3 => "audio/mpeg",
            ContentType::Mp4 => "video/mp4",
            ContentType::Mpeg => "video/mpeg",
            ContentType::Mpkg => "application/vnd.apple.installer+xml",
            ContentType::Odp => "application/vnd.oasis.opendocument.presentation",
            ContentType::Ods => "application/vnd.oasis.opendocument.spreadsheet",
            ContentType::Odt => "application/vnd.oasis.opendocument.text",
            ContentType::Oga => "audio/ogg",
            ContentType::Ogv => "video/ogg",
            ContentType::Ogx => "application/ogg",
            ContentType::Opus => "audio/opus",
            ContentType::Otf => "font/otf",
            ContentType::Png => "image/png",
            ContentType::Pdf => "application/pdf",
            ContentType::Php => "application/x-httpd-php",
            ContentType::Ppt => "application/vnd.ms-powerpoint",
            ContentType::Pptx => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            ContentType::Rar => "application/vnd.rar",
            ContentType::Rtf => "application/rtf",
            ContentType::Sh => "application/x-sh",
            ContentType::Svg => "image/svg+xml",
            ContentType::Tar => "application/x-tar",
            ContentType::Tif => "image/tiff",
            ContentType::Ts => "video/mp2t",
            ContentType::Ttf => "font/ttf",
            ContentType::Txt => "text/plain",
            ContentType::Vsd => "application/vnd.visio",
            ContentType::Wav => "audio/wav",
            ContentType::Weba => "audio/webm",
            ContentType::Webm => "video/webm",
            ContentType::Webp => "image/webp",
            ContentType::Woff => "font/woff",
            ContentType::Woff2 => "font/woff2",
            ContentType::Xhtml => "application/xhtml+xml",
            ContentType::Xls => "application/vnd.ms-excel",
            ContentType::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            ContentType::Xml => "application/xml",
            ContentType::Xul => "application/vnd.mozilla.xul+xml",
            ContentType::Zip => "application/zip",
            ContentType::_3gp => "video/3gpp; audio/3gpp if it doesn't contain video",
            ContentType::_3g2 => "video/3gpp2; audio/3gpp2 if it doesn't contain video",
            ContentType::_7z => "application/x-7z-compressed",
        };
        write!(f, "{str}; charset=utf-8")
    }
}