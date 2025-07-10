use std::fs;
use serde::Serialize;
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct MediaFile {
    /// The file name, including extension (e.g., `video.mp4`)
    pub name: String,
    /// The size of the file in bytes
    pub size: u64,
    /// The file extension without the dot (e.g., `mp4`, `jpg`)
    pub extension: String,
    /// Full path to the file on disk
    pub path: String,
}

/// Scans the given folder recursively for all files and collects their metadata into `MediaFile` structs.
///
/// # Arguments
///
/// * `folder` - A string slice representing the root directory to start scanning from.
///
/// # Returns
///
/// A `Result` containing a `Vec<MediaFile>` on success, or an `std::io::Error` if any file metadata couldn't be read.
///
/// # Example
///
/// ```
/// let media = scan_media_files("media/")?;
/// for file in media {
///     println!("Found media file: {}", file.name);
/// }
/// ```
pub fn scan_media_files(folder: &str) -> Result<Vec<MediaFile>, std::io::Error> {
    let mut files = vec![];

    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let metadata = fs::metadata(path)?;
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        files.push(MediaFile {
            name,
            size: metadata.len(),
            extension,
            path: path.to_string_lossy().to_string(),
        });
    }

    Ok(files)
}
