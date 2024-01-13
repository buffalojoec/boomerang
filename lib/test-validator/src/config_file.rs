use {
    serde::{Deserialize, Serialize},
    std::{fs::File, io::Write},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Commitment {
    Processed,
    Confirmed,
    Finalized,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfigFile {
    pub json_rpc_url: String,
    pub websocket_url: String,
    pub keypair_path: String,
    pub commitment: Commitment,
}
impl ConfigFile {
    pub fn write_to_yaml(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let yaml = serde_yaml::to_string(self).unwrap();
        file.write_all(yaml.as_bytes()).unwrap();
    }
}
