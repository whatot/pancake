pub const DEFAULT_LISTENING_ADDR: &str = "127.0.0.1:22560";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PancakeConfig {
    pub addr: String,
    pub single_node_mode: bool,
}

impl Default for PancakeConfig {
    fn default() -> PancakeConfig {
        PancakeConfig {
            addr: DEFAULT_LISTENING_ADDR.to_string(),
            single_node_mode: false,
        }
    }
}

impl PancakeConfig {
    pub fn setup_single_node(&mut self) {
        self.single_node_mode = true;
    }
}
