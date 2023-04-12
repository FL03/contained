/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::prelude::{Multiaddr, Resultant, DEFAULT_MULTIADDR};
use config::{Config, Environment};
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Type alias for [config::File]
type ConfigFile<Src, Fmt> = config::File<Src, Fmt>;
/// Type alias for a collection of [crate::ConfigFile]
type ConfigFileVec = Vec<ConfigFile<config::FileSourceFile, config::FileFormat>>;

/// A generic function wrapper extending glob::glob
pub fn collect_files_as<T>(f: &dyn Fn(PathBuf) -> T, pat: &str) -> anyhow::Result<Vec<T>> {
    let mut files = Vec::<T>::new();
    for r in glob::glob(pat)? {
        files.push(f(r?))
    }
    Ok(files)
}
/// Gather configuration files following the specified pattern and collect them into a vector
pub fn collect_config_files(pattern: &str, required: bool) -> ConfigFileVec {
    let f = |p: std::path::PathBuf| ConfigFile::from(p).required(required);
    collect_files_as(&f, pattern).expect("Failed to find any similar files...")
}

/// [package_name] is a simple functional wrapper for [env("CARGO_PKG_NAME")]
pub fn package_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}
/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn project_root() -> std::path::PathBuf {
    std::path::Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
/// Attempts to collect configuration files, following the given pattern, into a ConfigFileVec
pub fn try_collect_config_files(pattern: &str, required: bool) -> anyhow::Result<ConfigFileVec> {
    let f = |p: std::path::PathBuf| ConfigFile::from(p).required(required);
    collect_files_as(&f, pattern)
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Settings {
    pub logger: Logger,
    pub mode: String,
    pub network: NetworkSettings,
    pub system: SystemSettings,
}

impl Settings {
    pub fn new(mode: Option<String>) -> Self {
        Self {
            logger: Default::default(),
            mode: mode.unwrap_or_else(|| String::from("production")),
            network: NetworkSettings::default(),
            system: SystemSettings::default(),
        }
    }
    pub fn builder() -> config::ConfigBuilder<config::builder::DefaultState> {
        Config::builder()
    }
    pub fn build() -> Result<Self, config::ConfigError> {
        let mut builder = {
            Self::builder()
                .set_default("logger.level", "info")?
                .set_default("mode", "production")?
                .set_default("network.subnet.addr", DEFAULT_MULTIADDR)?
                .set_default("system.workdir", project_root().to_str().unwrap())?
        };
        // Try loading in environment variables; prefixed with the package name and separated by "__"
        builder = builder.add_source(
            Environment::default()
                .separator("__")
                .prefix(env!("CARGO_PKG_NAME").to_ascii_uppercase().as_str()),
        );
        // Try overriding configuration values with specific environment variables...
        if let Ok(log) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", log)?;
        };
        if let Ok(addr) = std::env::var("SUBNET_ADDR") {
            builder = builder.set_override("network.subnet.addr", addr)?;
        };
        // Try gathering valid configuration files...
        // builder = builder.add_source(config::File::from(PathBuf::from("./Contained.toml")));
        if let Ok(files) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(files);
        }
        builder.build()?.try_deserialize()
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(None);
        Self::build().unwrap_or(d)
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: tracing::Level::INFO.to_string(),
        }
    }
    pub fn set_level(mut self, level: impl ToString) {
        self.level = level.to_string();
    }
    pub fn setup_env(mut self, level: Option<&str>) -> Self {
        let key = level.unwrap_or("RUST_LOG");
        if let Some(v) = std::env::var_os(key) {
            self.level = v.into_string().expect("Failed to convert into string...");
        } else {
            std::env::set_var(key, self.level.clone());
        }
        self
    }
    pub fn init_tracing(self) {
        tracing_subscriber::fmt::init();
        tracing::debug!("Success: tracing layer initialized...");
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl From<tracing::Level> for Logger {
    fn from(level: tracing::Level) -> Self {
        Self {
            level: level.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemSettings {
    pub workdir: PathBuf,
}

impl SystemSettings {
    pub fn new() -> Self {
        Self {
            workdir: PathBuf::from("./"),
        }
    }
    pub fn setup_env(mut self, workdir: Option<&str>) -> Resultant<Self> {
        let key = workdir.unwrap_or("WORKDIR");
        if let Some(v) = std::env::var_os(key) {
            self.workdir = v.into();
        } else {
            std::env::set_var(key, self.workdir.to_str().unwrap());
        }
        Ok(self)
    }
    pub fn set_workdir(mut self, workdir: impl AsRef<std::path::Path>) {
        self.workdir = workdir.as_ref().to_path_buf();
    }
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SystemSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct NetworkSettings {
    pub subnet: SubnetConfig,
}

impl std::fmt::Display for NetworkSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubnetConfig {
    pub addr: Multiaddr,
    pub seed: Option<u8>,
}

impl SubnetConfig {
    pub fn new() -> Self {
        Self {
            addr: DEFAULT_MULTIADDR.parse().unwrap(),
            seed: None,
        }
    }
    pub fn setup_env(mut self, addr: Option<&str>, seed: Option<&str>) -> Resultant<Self> {
        let addr_key = addr.unwrap_or("CLUSTER_ADDR");
        let seed_key = seed.unwrap_or("CLUSTER_SEED");
        if let Some(v) = std::env::var_os(addr_key) {
            self.addr = v.into_string().unwrap().parse().unwrap();
        } else {
            std::env::set_var(addr_key, self.addr.to_string());
        }
        if let Some(v) = std::env::var_os(seed_key) {
            let seed = v
                .into_string()
                .unwrap()
                .parse()
                .expect("Failed to parse the seed value...");
            self.seed = Some(seed);
        } else {
            let seed = self.seed.unwrap_or(0);
            std::env::set_var(seed_key, seed.to_string());
        }
        Ok(self)
    }
    pub fn set_addr(mut self, addr: Multiaddr) {
        self.addr = addr;
    }
    pub fn set_seed(mut self, seed: u8) {
        self.seed = Some(seed);
    }
}

impl Default for SubnetConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SubnetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
