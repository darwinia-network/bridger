use bridge_component::http_client::{HttpClientComponent, HttpClientConfig};
use bridge_standard::component::BridgeComponent;

#[test]
fn test_http_client_component() {
	let config = HttpClientConfig { timeout: 30 };
	let component = HttpClientComponent::new(config).unwrap();
	let _client = component.component().unwrap();
	let _config = component.config();
}
