use bridge_component::config::HttpClientConfig;
use bridge_component::state::BridgeState;
use linked_template::config::TemplateLinkedConfig;
use linked_template::task::TemplateLinked;

#[tokio::test]
async fn test_linked() {
    let config_linked = TemplateLinkedConfig {
        http_client: HttpClientConfig { timeout: 1000 },
    };
    let config_state = Default::default();
    let state = BridgeState::new(config_state)
        .await
        .expect("failed to create bridge state");
    let _linked = TemplateLinked::new(config_linked, state).expect("failed to create linked");
}
