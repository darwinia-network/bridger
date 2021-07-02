use lifeline::{Bus, CarryFrom, Sender};

use bridge_component::config::HttpClientConfig;
use bridge_standard::bridge::task::{BridgeTask, BridgeTaskKeep};
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

    let config_linked = TemplateLinkedConfig {
        http_client: HttpClientConfig { timeout: 1000 },
    };
    let linked = TemplateLinked::new(config_linked).expect("failed to create linked");

    let config_task = TemplateTaskConfig {
        http_client: HttpClientConfig { timeout: 1000 },
    };
    let mut task = TemplateTask::new(config_task).expect("failed to create task");

    task.keep_carry(
        linked
            .bus()
            .carry_from(task.bus())
            .expect("failed to carry from template task"),
    );

    let mut tx = task
        .bus()
        .tx::<TemplateTaskMessage>()
        .expect("failed to get sender");
    tx.send(TemplateTaskMessage::SomeEvent)
        .await
        .expect("failed to send message");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
