pub mod schemes;

use crate::download::Download;
use crate::github::schemes::release::Release;
use reqwest::StatusCode;
use schemes::releases::Releases;

static DEFAULT_PROVIDER: &str = "https://github.com";
static DEFAULT_API_PROVIDER: &str = "https://api.github.com";
static APP_USER_AGENT: &str = concat!("Bulut Explorer ", env!("CARGO_PKG_VERSION"));

#[derive(Debug)]
pub struct GitHub {
    owner: String,
    repo: String,
    client: reqwest::blocking::Client,
}

#[derive(Debug)]
pub enum GitHubError {
    ClientInstance,
    SerializeError,
    DownloadError,
    ParseError,
    VersionNotFound,
}

impl GitHub {
    pub fn new(owner: &str, repo: &str) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            client: reqwest::blocking::Client::builder()
                .user_agent(APP_USER_AGENT)
                .build()
                .unwrap(),
        }
    }

    // Example:
    // https://github.com/osmon-lang/havo/releases/download/v0.0.3/libhavo.h
    pub fn download_file(&self, version: &str, file: &str) -> Result<(), GitHubError> {
        let mut downloader = Download::new(format!(
            "{}/{}/{}/releases/download/{}/{}",
            DEFAULT_PROVIDER, self.owner, self.repo, version, file
        ));

        match downloader.download() {
            Ok(_) => Ok(()),
            Err(_) => Err(GitHubError::DownloadError),
        }
    }

    pub fn release(&self, version: &String) -> Result<Release, GitHubError> {
        // Result<Release, GitHubError>
        let response = self
            .client
            .get(format!(
                "{}/repos/{}/{}/releases/tags/{}",
                DEFAULT_API_PROVIDER, self.owner, self.repo, version
            ))
            .send()
            .unwrap();

        if response.status() != StatusCode::OK {
            return Err(GitHubError::VersionNotFound);
        }

        let parse = response.text().unwrap();

        match serde_json::from_str::<Release>(&parse) {
            Ok(r) => Ok(r),
            Err(_) => Err(GitHubError::SerializeError),
        }
    }

    // Example:
    // https://api.github.com/repos/osmon-lang/havo/releases
    pub fn releases(&self) -> Result<Vec<String>, GitHubError> {
        let response = self
            .client
            .get(format!(
                "{}/repos/{}/{}/releases",
                DEFAULT_API_PROVIDER, self.owner, self.repo
            ))
            .send()
            .unwrap()
            .text()
            .unwrap();

        let release: Vec<Releases> = serde_json::from_str(&response).unwrap();

        Ok(release
            .iter()
            .map(|r| r.tag_name.clone())
            .collect::<Vec<String>>())
    }
}
