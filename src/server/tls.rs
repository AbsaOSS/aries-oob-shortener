use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub fn load_rustls_config(cert_path: &str, key_path: &str) -> rustls::ServerConfig {
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut std::io::BufReader::new(std::fs::File::open(cert_path).unwrap());
    let key_file = &mut std::io::BufReader::new(std::fs::File::open(key_path).unwrap());

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
