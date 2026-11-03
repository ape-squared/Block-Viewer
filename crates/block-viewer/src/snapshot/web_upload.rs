//! Anonymous photo upload for the viewfinder's Web destination.
//!
//! Hosted on [catbox.moe](https://catbox.moe) (no account, no API key). The
//! response is a plain URL the user can share. Failures stay local: nothing
//! is logged beyond the HTTP status / a short error string.

use std::time::Duration;

const ENDPOINT: &str = "https://catbox.moe/user/api.php";
const BOUNDARY: &str = "----BlockViewerCatboxBoundary7MA4YWxkTrZu0gW";
const TIMEOUT: Duration = Duration::from_secs(90);
/// Soft cap before we refuse to post (catbox allows more; keep photos light).
pub const MAX_BYTES: usize = 25 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    TooLarge(usize),
    Network,
    Rejected(u16),
    BadReply,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TooLarge(n) => write!(f, "image too large ({n} bytes)"),
            Error::Network => write!(f, "network error"),
            Error::Rejected(code) => write!(f, "HTTP {code}"),
            Error::BadReply => write!(f, "unexpected reply"),
        }
    }
}

/// Encode RGBA as PNG (same path as the profile feed).
pub fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>, String> {
    super::profile::encode_png(width, height, rgba)
}

fn multipart_body(png: &[u8], file_name: &str) -> Vec<u8> {
    let mut body = Vec::with_capacity(png.len() + 512);
    body.extend_from_slice(format!("--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"reqtype\"\r\n\r\nfileupload\r\n").as_bytes());
    body.extend_from_slice(
        format!(
            "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"fileToUpload\"; filename=\"{file_name}\"\r\nContent-Type: image/png\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(png);
    body.extend_from_slice(format!("\r\n--{BOUNDARY}--\r\n").as_bytes());
    body
}

/// POST the PNG; returns the public https URL on success.
pub async fn upload_png(png: Vec<u8>, file_name: &str) -> Result<String, Error> {
    if png.len() > MAX_BYTES {
        return Err(Error::TooLarge(png.len()));
    }
    let http = reqwest::Client::builder().timeout(TIMEOUT).build().map_err(|_| Error::Network)?;
    let body = multipart_body(&png, file_name);
    let response = http
        .post(ENDPOINT)
        .header("User-Agent", crate::media::user_agent())
        .header("Content-Type", format!("multipart/form-data; boundary={BOUNDARY}"))
        .body(body)
        .send()
        .await
        .map_err(|_| Error::Network)?;
    let status = response.status();
    if !status.is_success() {
        return Err(Error::Rejected(status.as_u16()));
    }
    let text = response.text().await.map_err(|_| Error::Network)?;
    let url = text.trim();
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err(Error::BadReply);
    }
    Ok(url.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_oversize_before_network() {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        let big = vec![0u8; MAX_BYTES + 1];
        let err = rt.block_on(upload_png(big, "x.png")).unwrap_err();
        assert!(matches!(err, Error::TooLarge(_)));
    }

    #[test]
    fn multipart_includes_reqtype_and_file() {
        let body = multipart_body(b"PNG", "shot.png");
        let text = String::from_utf8_lossy(&body);
        assert!(text.contains("name=\"reqtype\""));
        assert!(text.contains("fileupload"));
        assert!(text.contains("filename=\"shot.png\""));
        assert!(text.contains("PNG"));
    }
}
