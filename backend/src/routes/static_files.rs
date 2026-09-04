use rocket::fs::{FileServer, NamedFile};
use rocket::http::Method;
use rocket::{catch, Request};
use std::path::PathBuf;

pub fn frontend_dist_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../frontend/dist")
}

pub fn index_html_path() -> PathBuf {
    frontend_dist_root().join("index.html")
}

pub async fn spa_fallback(_path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(index_html_path()).await.ok()
}

#[catch(404)]
pub async fn spa_fallback_catcher(request: &Request<'_>) -> Option<NamedFile> {
    let path = request.uri().path();

    if request.method() != Method::Get || path.starts_with("/api") || path.starts_with("/auth") {
        return None;
    }

    NamedFile::open(index_html_path()).await.ok()
}

pub fn file_server() -> FileServer {
    FileServer::from(frontend_dist_root())
}

#[cfg(test)]
mod tests {
    use super::{index_html_path, spa_fallback};
    use std::path::PathBuf;

    #[tokio::test]
    async fn spa_fallback_returns_frontend_index_when_present() {
        let result = spa_fallback(PathBuf::from("/some/deep/route")).await;
        assert!(result.is_some());
        assert!(index_html_path().exists());
    }
}
