#[cfg(feature = "ethlike-v1")]
use client_pangolin::types::EthereumAccount;

#[test]
#[cfg(feature = "ethlike-v1")]
fn test_ecdsa() {
    let hash =
        array_bytes::hex2bytes("71e2f60faf6c7264cca14fb1a01260a787b4d18039cd8cd680aaff1e118c711d")
            .unwrap();
    let account = EthereumAccount::new(
        "https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80".to_string(),
        Some("0x8bd012fd2433d4fea852f437d6bb22d1e57dee7657cc1e703460ddeaae1a67ca".to_string()),
    );
    let ecdsa_signature = account.ecdsa_sign(hash.as_slice()).unwrap();
    println!("{:x?}", ecdsa_signature.0);
}
