#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FileConfig {
    pub path: String,
    pub data: String,
    #[serde(default)]
    pub symlink: bool,
    #[serde(default)]
    pub directory: bool,
    pub mode: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    #[serde(default)]
    pub recursive_chown: bool,
}

impl FileConfig {

}