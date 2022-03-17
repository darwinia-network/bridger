use shadow_liketh::component::ShadowComponent;
use shadow_liketh::config::ShadowConfig;
use shadow_liketh::shadow::Shadow;
use shadow_liketh::types::BridgeName;

/// Get shadow client
pub fn shadow() -> Shadow {
    let shadow_config = ShadowConfig::default();
    ShadowComponent::component(shadow_config, BridgeName::PangolinRopsten).unwrap()
}
