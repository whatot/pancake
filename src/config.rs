
pub const DEFAULT_LISTENING_ADDR: &str = "127.0.0.1:22560";

#[derive(Debug, Clone, Serialize)]
#[serde(default)]
pub struct PancakeConfig {
    pub addr: String,
}

impl Default for PancakeConfig {
    fn default() -> PancakeConfig {
        PancakeConfig {
            addr: DEFAULT_LISTENING_ADDR.to_owned(),
        }
    }
}
