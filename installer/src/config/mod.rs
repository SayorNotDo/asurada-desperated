use std::collections::BTreeMap;
use std::path::PathBuf;

pub mod general;
pub mod package;
pub mod file;
mod user;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub general: general::GeneralConfig,
    #[serde(default)]
    pub include: Vec<PathBuf>,
    #[serde(default)]
    pub packages: BTreeMap<String, package::PackageConfig>,
    #[serde(default)]
    pub files: Vec<file::FileConfig>,
    #[serde(default)]
    pub users: BTreeMap<String, user::GroupConfig>,
}