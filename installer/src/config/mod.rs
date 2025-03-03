use std::collections::BTreeMap;
use std::path::PathBuf;

mod general;
mod package;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub general: general::GeneralConfig,
    #[serde(default)]
    pub include: Vec<PathBuf>,
    pub packages: BTreeMap<String, package::PackageConfig>,
}