//! Anonymous photo upload for the viewfinder's Web destination.
//!
//! Tries [catbox.moe](https://catbox.moe) first (no account). When that host
//! pauses uploads (HTTP 412) or fails, falls back to
//! [pomf2.lain.la](https://pomf2.lain.la). The response is a plain URL the
//! user can share.

use std::time::Duration;

const CATBOX: &str = "https://catbox.moe/user/api.php";
const POMF: &str = "https://pomf2.lain.la/upload.php";
const BOUNDARY: &str = "----BlockViewerWebUploadBoundary7MA4YWxkTrZu0gW";
const TIMEOUT: Duration = Duration::from_secs(90);
/// Soft cap before we refuse to post.
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

fn multipart(fields: &[(&str, &str)], file_field: &str, file_name: &str, png: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(png.len() + 1024);
    for (name, value) in fields {
        body.extend_from_slice(format!("--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n").as_bytes());
    }
    body.extend_from_slice(
        format!(
            "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{file_field}\"; filename=\"{file_name}\"\r\nContent-Type: image/png\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(png);
    body.extend_from_slice(format!("\r\n--{BOUNDARY}--\r\n").as_bytes());
    body
}

fn url_from_plain(text: &str) -> Option<String> {
    let url = text.trim();
    (url.starts_with("https://") || url.starts_with("http://")).then(|| url.to_owned())
}

fn url_from_pomf(text: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(text).ok()?;
    if v.get("success") != Some(&serde_json::Value::Bool(true)) {
        return None;
    }
    v.get("files")?
        .as_array()?
        .first()?
        .get("url")?
        .as_str()
        .map(|u| u.replace("\\/", "/"))
        .filter(|u| u.starts_with("https://") || u.starts_with("http://"))
}

async fn post(http: &reqwest::Client, url: &str, body: Vec<u8>) -> Result<(u16, String), Error> {
    let response = http
        .post(url)
        .header("User-Agent", "BlockViewer-PhotoUpload/1.0")
        .header("Content-Type", format!("multipart/form-data; boundary={BOUNDARY}"))
        .body(body)
        .send()
        .await
        .map_err(|_| Error::Network)?;
    let status = response.status().as_u16();
    let text = response.text().await.map_err(|_| Error::Network)?;
    Ok((status, text))
}

/// POST the PNG; returns the public https URL on success.
pub async fn upload_png(png: Vec<u8>, file_name: &str) -> Result<String, Error> {
    if png.len() > MAX_BYTES {
        return Err(Error::TooLarge(png.len()));
    }
    let http = reqwest::Client::builder().timeout(TIMEOUT).build().map_err(|_| Error::Network)?;

    // Catbox first (when it is accepting uploads).
    let catbox = multipart(&[("reqtype", "fileupload")], "fileToUpload", file_name, &png);
    match post(&http, CATBOX, catbox).await {
        Ok((200..=299, text)) => {
            if let Some(url) = url_from_plain(&text) {
                return Ok(url);
            }
            log::warn!("photo web: catbox unexpected reply: {}", text.chars().take(120).collect::<String>());
        }
        Ok((code, text)) => {
            log::warn!("photo web: catbox HTTP {code}: {}", text.chars().take(120).collect::<String>());
        }
        Err(e) => log::warn!("photo web: catbox {e}"),
    }

    // pomf2 when Catbox is paused or otherwise refused.
    let pomf = multipart(&[], "files[]", file_name, &png);
    let (code, text) = post(&http, POMF, pomf).await?;
    if !(200..300).contains(&code) {
        return Err(Error::Rejected(code));
    }
    url_from_pomf(&text).ok_or(Error::BadReply)
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
    fn parses_pomf_json_url() {
        let body = r#"{"success":true,"files":[{"url":"https:\/\/pomf2.lain.la\/f\/abc.png"}]}"#;
        assert_eq!(url_from_pomf(body).as_deref(), Some("https://pomf2.lain.la/f/abc.png"));
    }

    #[test]
    fn multipart_includes_file_field() {
        let body = multipart(&[("reqtype", "fileupload")], "fileToUpload", "shot.png", b"PNG");
        let text = String::from_utf8_lossy(&body);
        assert!(text.contains("name=\"reqtype\""));
        assert!(text.contains("fileupload"));
        assert!(text.contains("filename=\"shot.png\""));
        assert!(text.contains("PNG"));
    }
}
