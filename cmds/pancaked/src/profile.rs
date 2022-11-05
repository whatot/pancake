pub const DEFAULT_LISTENING_ADDR: &str = "127.0.0.1:22560";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProfileOpt {
    pub addr: String,
    pub single_node_mode: bool,
}

impl Default for ProfileOpt {
    fn default() -> ProfileOpt {
        ProfileOpt {
            addr: DEFAULT_LISTENING_ADDR.to_string(),
            single_node_mode: false,
        }
    }
}

impl ProfileOpt {
    pub fn setup_single_node(&mut self) {
        self.single_node_mode = true;
    }
}
