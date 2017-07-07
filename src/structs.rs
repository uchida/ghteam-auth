use serde_json;
use reqwest;
use std;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
    pub org: String,
    pub team: String,
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_home")]
    pub home: String,
    #[serde(default = "default_gid")]
    pub gid: u64,
    #[serde(default = "default_sh")]
    pub sh: String,
    pub group: Option<String>,
    #[serde(default = "default_cache_duration")]
    pub cache_duration: u64,
    #[serde(default = "default_cert_path")]
    pub cert_path: String,
}

fn default_endpoint() -> String { String::from("https://api.github.com") }
fn default_home() -> String { String::from("/home/{}") }
fn default_gid() -> u64 { 2000 }
fn default_sh() -> String { String::from("/bin/bash") }
fn default_cache_duration() -> u64 { 3600 }
fn default_cert_path() -> String { String::from("/etc/ssl/certs/ca-certificates.crt") }

impl Config {
    pub fn new(configpath: &str) -> Result<Self, CliError> {
        let mut file = File::open(configpath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(toml::from_str::<Config>(&contents)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonalConfig {
    pub sh: Option<String>,
}

impl PersonalConfig {
    pub fn new(configpath: &str) -> Result<Self, CliError> {
        let mut file = File::open(configpath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(toml::from_str::<PersonalConfig>(&contents)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: u64,
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub id: u64,
    pub key: String,
}

#[derive(Debug)]
pub enum CliError {
    Serde(serde_json::Error),
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> CliError { CliError::Serde(err) }
}
impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> CliError { CliError::Reqwest(err) }
}
impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError { CliError::Io(err) }
}
impl From<toml::de::Error> for CliError {
    fn from(err: toml::de::Error) -> CliError { CliError::Toml(err) }
}
