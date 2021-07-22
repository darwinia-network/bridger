use bridge_traits::bridge::component::BridgeComponent;
use component_http_client::HttpClientConfig;
use component_state::state::BridgeStateComponent;
use linked_template::config::TemplateLinkedConfig;
use linked_template::task::TemplateLinked;

#[tokio::test]
async fn test_linked() {
    let config_linked = TemplateLinkedConfig {
        http_client: HttpClientConfig { timeout: 1000 },
    };
    let config_state = Default::default();
    let component_state = BridgeStateComponent::new(config_state);
    let state = component_state
        .component()
        .await
        .expect("failed to create bridge state");
    let _linked = TemplateLinked::new(config_linked, state).expect("failed to create linked");
}
