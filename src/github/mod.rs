mod schemes;

use schemes::release::Release;

static DEFAULT_API_PROVIDER: &str = "https://api.github.com";
static APP_USER_AGENT: &str = concat!("Bulut Explorer ", env!("CARGO_PKG_VERSION"));

pub struct GitHub {
    owner: String,
    repo: String,
    client: reqwest::blocking::Client,
}

pub enum GitHubError {
    ClientInstance,
    SerializeError
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

    pub fn releases(&self) -> Result<Vec<Release>, GitHubError> {
        let resp = self
            .client
            .get(format!(
                "{}/repos/{}/{}/releases",
                DEFAULT_API_PROVIDER, self.owner, self.repo
            ))
            .send()
            .unwrap()
            .text()
            .unwrap();
        let release: Vec<Release> = serde_json::from_str(&resp).unwrap();

        Ok(release)
    }
}
