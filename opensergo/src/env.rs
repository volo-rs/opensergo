use serde::Deserialize;

// https://github.com/opensergo/opensergo-specification/blob/main/specification/en/README.md#data-plane-and-control-plane-communication-configuration
const BOOTSTRAP_CONFIG_KEY: &str = "OPENSERGO_BOOTSTRAP_CONFIG";
const BOOTSTRAP_KEY: &str = "OPENSERGO_BOOTSTRAP";

#[derive(Deserialize)]
pub struct CommunicationConfig {
    pub endpoint: String,
}

/// Get the communication config from the environment.
pub fn get_communication_config() -> anyhow::Result<CommunicationConfig> {
    let s = get_config_string()?;
    Ok(serde_json::from_str(&s)?)
}

fn get_config_string() -> anyhow::Result<String> {
    if let Ok(s) = std::env::var(BOOTSTRAP_CONFIG_KEY) {
        Ok(s)
    } else if let Ok(p) = std::env::var(BOOTSTRAP_KEY) {
        let p = std::path::Path::new(&p);
        let s = std::fs::read_to_string(p)?;
        Ok(s)
    } else {
        anyhow::bail!("{} or {} must be set", BOOTSTRAP_CONFIG_KEY, BOOTSTRAP_KEY);
    }
}
