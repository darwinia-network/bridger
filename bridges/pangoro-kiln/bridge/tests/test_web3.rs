use relay_e2e::types::ethereum::FastEthereumAccount;

#[test]
fn test_signing() {
    let message = array_bytes::hex2bytes(
        "0x331a5c39bad492d36b8306eb45792c3198c374eb0dc188bc704729f9330093f3",
    )
    .unwrap();
    let seed = "0x40b50cd43ccbfe7da7e594216710eac2ab0036fa59a957a85c5d8ee4f3761f49";
    let eth_account = FastEthereumAccount::new(seed);
    let signature = eth_account.sign(message.as_slice()).unwrap();
    let expected = "0x9d534608bb6a55ebf900e4835e90d0355aa4e30830ba3e3f6f3fdf913b59fec138412bc5957975f23370ea2a035e9b4d6a69a9effc4a32de0789490b4a0947d701";
    let compare = array_bytes::bytes2hex("0x", &signature);
    assert_eq!(&compare[..], expected);
}
