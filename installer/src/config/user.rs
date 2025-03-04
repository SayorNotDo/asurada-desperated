#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserConfig {
    pub password: Option<String>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub name: Option<String>,
    pub home: Option<String>,
    pub shell: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GroupConfig {
    pub gid: Option<u32>,
    pub members: Vec<String>,
}
