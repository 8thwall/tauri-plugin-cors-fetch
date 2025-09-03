#[cfg(test)]
mod tests {
    use crate::commands::{get_response, build_request, RequestConfig};
    use url::Url;
    use tokio::sync::oneshot;
    use flate2::{write::GzEncoder, write::DeflateEncoder, Compression};
    use brotli::CompressorWriter;
    use std::io::Write;

    fn encode_gzip(data: &[u8]) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }

    fn encode_deflate(data: &[u8]) -> Vec<u8> {
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }

    fn encode_brotli(data: &[u8]) -> Vec<u8> {
        let mut encoder = CompressorWriter::new(Vec::new(), 4096, 5, 22);
        encoder.write_all(data).unwrap();
        encoder.into_inner()
    }

    #[tokio::test]
    async fn test_get_response_gzip() {
        use httpmock::MockServer;
        let server = MockServer::start_async().await;
        let data = b"hello gzip";
        let encoded = encode_gzip(data);
        let mock = server.mock_async(|when, then| {
            when.method("GET").path("/test_gzip");
            then.status(200)
                .header("content-encoding", "gzip")
                .body(encoded.clone());
        }).await;
        let url = Url::parse(&format!("{}test_gzip", server.url("/"))).unwrap();
        let request_config = RequestConfig::new(
            1,
            "GET".to_string(),
            url,
            vec![],
            None,
            None,
            None,
            None,
        );
        let request = build_request(request_config).unwrap();
        let (_tx, rx) = oneshot::channel();
        let response = get_response(request, rx).await.unwrap();
        assert_eq!(response.body().as_ref().unwrap(), data);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_response_deflate() {
        use httpmock::MockServer;
        let server = MockServer::start_async().await;
        let data = b"hello deflate";
        let encoded = encode_deflate(data);
        let mock = server.mock_async(|when, then| {
            when.method("GET").path("/test_deflate");
            then.status(200)
                .header("content-encoding", "deflate")
                .body(encoded.clone());
        }).await;
        let url = Url::parse(&format!("{}test_deflate", server.url("/"))).unwrap();
        let request_config = RequestConfig::new(
            2,
            "GET".to_string(),
            url,
            vec![],
            None,
            None,
            None,
            None,
        );
        let request = build_request(request_config).unwrap();
        let (_tx, rx) = oneshot::channel();
        let response = get_response(request, rx).await.unwrap();
        assert_eq!(response.body().as_ref().unwrap(), data);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_response_brotli() {
        use httpmock::MockServer;
        let server = MockServer::start_async().await;
        let data = b"hello brotli";
        let encoded = encode_brotli(data);
        let mock = server.mock_async(|when, then| {
            when.method("GET").path("/test_brotli");
            then.status(200)
                .header("content-encoding", "br")
                .body(encoded.clone());
        }).await;
        let url = Url::parse(&format!("{}test_brotli", server.url("/"))).unwrap();
        let request_config = RequestConfig::new(
            3,
            "GET".to_string(),
            url,
            vec![],
            None,
            None,
            None,
            None,
        );
        let request = build_request(request_config).unwrap();
        let (_tx, rx) = oneshot::channel();
        let response = get_response(request, rx).await.unwrap();
        assert_eq!(response.body().as_ref().unwrap(), data);
        mock.assert_async().await;
    }
}