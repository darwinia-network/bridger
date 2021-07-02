use bridge_component::config::HttpClientConfig;
use linked_template::config::TemplateLinkedConfig;
use linked_template::task::TemplateLinked;

#[tokio::test]
async fn test_linked() {
    let config = TemplateLinkedConfig {
        http_client: HttpClientConfig { timeout: 1000 },
    };
    let _linked = TemplateLinked::new(config).expect("failed to create linked");
}
