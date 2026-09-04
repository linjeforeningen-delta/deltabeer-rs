use axum::{Router, routing::get};
use rcgen::generate_simple_self_signed;
use std::io::Write;
use std::net::TcpListener;
use std::time::Duration;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_server_starts_and_serves_https() {
    let subject_alt_names = vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "::1".to_string(),
    ];
    let cert_key = generate_simple_self_signed(subject_alt_names).unwrap();
    let cert_pem = cert_key.cert.pem();
    let key_pem = cert_key.signing_key.serialize_pem();

    let mut cert_file = NamedTempFile::new().unwrap();
    cert_file.write_all(cert_pem.as_bytes()).unwrap();

    let mut key_file = NamedTempFile::new().unwrap();
    key_file.write_all(key_pem.as_bytes()).unwrap();

    let tls_config =
        axum_server::tls_rustls::RustlsConfig::from_pem_file(cert_file.path(), key_file.path())
            .await
            .unwrap();

    let app = Router::new().route("/health", get(|| async { "ok" }));

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let local_addr = listener.local_addr().unwrap();

    let handle = axum_server::Handle::new();
    let handle_clone = handle.clone();

    let server_task = tokio::spawn(async move {
        axum_server::from_tcp_rustls(listener, tls_config)
            .unwrap()
            .handle(handle_clone)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // Give server a moment to start
    tokio::time::sleep(Duration::from_millis(50)).await;

    // 1. Client with trusted CA should succeed
    let root_cert = reqwest::Certificate::from_pem(cert_pem.as_bytes()).unwrap();
    let client_trusted = reqwest::Client::builder()
        .add_root_certificate(root_cert)
        .build()
        .unwrap();

    let res = client_trusted
        .get(format!("https://127.0.0.1:{}/health", local_addr.port()))
        .send()
        .await;
    assert!(
        res.is_ok(),
        "Request with trusted root cert should succeed: {:?}",
        res.err()
    );
    let response = res.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let body = response.text().await.unwrap();
    assert_eq!(body, "ok");

    // 2. Untrusted client without the self-signed CA should fail certificate validation
    let client_untrusted = reqwest::Client::builder().build().unwrap();
    let untrusted_res = client_untrusted
        .get(format!("https://127.0.0.1:{}/health", local_addr.port()))
        .send()
        .await;
    assert!(
        untrusted_res.is_err(),
        "Request without trusted root cert should fail TLS handshake"
    );

    // Shutdown server
    handle.shutdown();
    let _ = server_task.await;
}

#[tokio::test]
async fn test_tls_config_loading_errors() {
    let cert_key = generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    let cert_pem = cert_key.cert.pem();
    let key_pem = cert_key.signing_key.serialize_pem();

    let mut valid_cert_file = NamedTempFile::new().unwrap();
    valid_cert_file.write_all(cert_pem.as_bytes()).unwrap();

    let mut valid_key_file = NamedTempFile::new().unwrap();
    valid_key_file.write_all(key_pem.as_bytes()).unwrap();

    // 1. Missing certificate file: cert path does not exist, valid key file
    let missing_cert = NamedTempFile::new().unwrap();
    let missing_cert_path = missing_cert.path().to_path_buf();
    drop(missing_cert); // file is deleted

    let err = axum_server::tls_rustls::RustlsConfig::from_pem_file(
        &missing_cert_path,
        valid_key_file.path(),
    )
    .await;
    assert!(
        err.is_err(),
        "Should fail when cert file is missing with valid key"
    );

    // 2. Missing private key file: valid cert file, key path does not exist
    let missing_key = NamedTempFile::new().unwrap();
    let missing_key_path = missing_key.path().to_path_buf();
    drop(missing_key); // file is deleted

    let err = axum_server::tls_rustls::RustlsConfig::from_pem_file(
        valid_cert_file.path(),
        &missing_key_path,
    )
    .await;
    assert!(
        err.is_err(),
        "Should fail when key file is missing with valid cert"
    );

    // 3. Malformed certificate file: malformed cert, valid key
    let mut bad_cert_file = NamedTempFile::new().unwrap();
    bad_cert_file
        .write_all(b"not a valid certificate pem")
        .unwrap();

    let err = axum_server::tls_rustls::RustlsConfig::from_pem_file(
        bad_cert_file.path(),
        valid_key_file.path(),
    )
    .await;
    assert!(
        err.is_err(),
        "Should fail when cert file is malformed with valid key"
    );

    // 4. Malformed private key file: valid cert, malformed key
    let mut bad_key_file = NamedTempFile::new().unwrap();
    bad_key_file
        .write_all(b"not a valid private key pem")
        .unwrap();

    let err = axum_server::tls_rustls::RustlsConfig::from_pem_file(
        valid_cert_file.path(),
        bad_key_file.path(),
    )
    .await;
    assert!(
        err.is_err(),
        "Should fail when key file is malformed with valid cert"
    );
}
