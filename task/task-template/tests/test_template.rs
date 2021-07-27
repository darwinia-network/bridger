use lifeline::{Bus, Sender};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::task::BridgeTask;
use component_http_client::HttpClientConfig;
use component_state::state::BridgeStateComponent;
use linked_template::config::TemplateLinkedConfig;
use linked_template::task::TemplateLinked;
use task_template::config::TemplateTaskConfig;
use task_template::message::TemplateTaskMessage;
use task_template::task::TemplateTask;

#[tokio::test]
async fn test_task() {
    std::env::set_var(
        "RUST_LOG",
        r#"
        lifeline=debug,
        task-template=debug,
        linked-template=debug,
        "#,
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config_state = Default::default();
    let component_state = BridgeStateComponent::new(config_state);
    let state = component_state
        .component()
        .await
        .expect("failed to create bridge state");

    let config_linked = TemplateLinkedConfig {
        http_client: HttpClientConfig { timeout: 1000 },
    };
    let mut linked =
        TemplateLinked::new(config_linked, state.clone()).expect("failed to create linked");

    let config_task = TemplateTaskConfig {
        http_client: HttpClientConfig { timeout: 1000 },
    };
    let mut task = TemplateTask::new(config_task, state.clone()).expect("failed to create task");
    let stack_task = task.stack();
    let bus_template = stack_task.bus();

    let mut tx = bus_template
        .tx::<TemplateTaskMessage>()
        .expect("failed to get sender");

    // linked.stack().bus().carry_from()
    // let carry = linked
    //     .bus()
    //     .carry_from(bus_template)
    //     .expect("failed to carry from template task");
    // task.stack()
    //     .carry(carry)
    //     .expect("failed to linked carry template");

    linked
        .stack()
        .carry_from(stack_task)
        .expect("failed to linked carry template");

    tx.send(TemplateTaskMessage::SomeEvent)
        .await
        .expect("failed to send message");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
