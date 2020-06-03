use isolang::Language;
use reqwest::blocking::Client;
use crate::{
    Error,
    Result,
    Url,
};


const DEFAULT_ENV_PATH: &str = ".env";
const DEFAULT_ENV_TEST_PATH: &str = ".env.test";
const ENV_SERVER_URL: &str = "SERVER_URL";
const ENV_ACCESS_TOKEN: &str = "ACCESS_TOKEN";
const ENV_USER_AGENT: &str = "USER_AGENT";
const ENV_DEFAULT_LANGUAGE: &str = "DEFAULT_LANGUAGE";
const ENV_STATUS_MAX_CHARACTERS: &str = "STATUS_MAX_CHARACTERS";
const ENV_STATUS_MAX_MEDIAS: &str = "STATUS_MAX_MEDIAS";
const ENV_POLL_MAX_OPTIONS: &str = "POLL_MAX_OPTIONS";
const ENV_WHITELIST_MODE: &str = "WHITELIST_MODE";

const DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),
);
const DEFAULT_STATUS_MAX_CHARACTERS: &str = "500";
const DEFAULT_STATUS_MAX_MEDIAS: &str = "4";
const DEFAULT_POLL_MAX_OPTIONS: &str = "4";

/// A `Connection` contains HTTP client and some settings to use REST API of the Mastodon server.
/// 
/// The Connection loads following variables from `.env`:
/// 
/// - `SERVER_URL`: Mastodon server URL.
/// - `ACCESS_TOKEN`: A OAuth access token string generated by Mastodon server.
/// - `USER_AGENT`: A string that send to Mastodon server as User-Agent http header. This setting is optional.
/// - `DEFAULT_LANGUAGE`: Language set when status is posted. This setting must to be ISO639-1 compliant and is optional.
/// - `STATUS_MAX_CHARACTERS`: A max number of characters that can be included the status. This setting is optional and default value is 500.
/// - `STATUS_MAX_MEDIAS`: A max number of characters that can be included the status. This setting is optional and default value is 4.
/// - `POLL_MAX_OPTIONS`: A max number of options that can be included the poll. This setting is optional and default value is 4.
/// - `WHITELIST_MODE`: If defined, use the access_token if required. This setting is optional and disabled by default.
/// 
/// ```bash
/// SERVER_URL="https://mastodon.social"
/// ACCESS_TOKEN="ABCabc_ABCDEFG012345678_HIJKLMNhijklmn00000"
/// USER_AGENT="MyApp"
/// DEFAULT_LANGUAGE="ja"
/// STATUS_MAX_CHARACTERS="500"
/// STATUS_MAX_MEDIAS="4"
/// POLL_MAX_OPTIONS="4"
/// WHITELIST_MODE="true"
/// ```
/// 
/// The Connection holds a `reqwest::Client` internally, and `reqwest::Client` holds a connection pool, so it is recommended that you create one Connection and **reuse** it.
/// 
#[derive(Debug, Clone)]
pub struct Connection {
    server: Url,
    access_token: String,
    user_agent: String,
    default_language: Option<Language>,
    status_max_characters: usize,
    status_max_medias: usize,
    poll_max_options: usize,
    whitelist_mode: bool,
    client: Client,
}

impl Connection {
    /// Constructs a new `Connection` using `.env` file in the current directory.
    /// 
    /// # Panics
    /// 
    /// This function will panic if:
    /// 
    /// - Configuration file not found.
    /// - SERVER_URL is not set or cannot parse as URL
    /// - ACCESS_TOKEN is not set or is not a valid utf8 characters
    /// - DEFAULT_LANGUAGE is not ISO639-1 compliant
    /// - STATUS_MAX_CHARACTERS is not a number
    /// - STATUS_MAX_MEDIAS is not a number
    /// - POLL_MAX_OPTIONS is not a number
    pub fn new() -> Result<Self> {
        if cfg!(test) {
            Self::from_file(DEFAULT_ENV_TEST_PATH)
        } else {
            Self::from_file(DEFAULT_ENV_PATH)
        }
    }

    /// Constructs a new `Connection` using specified configuration file.
    pub fn from_file(env_path: &str) -> Result<Self> {
        use std::env;

        dotenv::from_filename(env_path).ok();

        let server = Url::from_env(ENV_SERVER_URL, "")?;

        let access_token = env::var(ENV_ACCESS_TOKEN)
            .map_err(|e| Error::EnvVarError {
                source: e,
                env_var: ENV_ACCESS_TOKEN,
            })?;

        let user_agent = env::var(ENV_USER_AGENT).unwrap_or_else(|_| DEFAULT_USER_AGENT.to_owned());

        let default_language = match env::var(ENV_DEFAULT_LANGUAGE).ok() {
            Some(lang) => {
                Some(Language::from_639_1(&lang).ok_or(Error::ParseIso639_1Error(lang))?)
            },
            None => None,
        };

        let status_max_characters = usize::from_env(
            ENV_STATUS_MAX_CHARACTERS,
            DEFAULT_STATUS_MAX_CHARACTERS
        )?;

        let status_max_medias = usize::from_env(
            ENV_STATUS_MAX_MEDIAS,
            DEFAULT_STATUS_MAX_MEDIAS
        )?;

        let poll_max_options = usize::from_env(
            ENV_POLL_MAX_OPTIONS,
            DEFAULT_POLL_MAX_OPTIONS
        )?;

        let whitelist_mode = env::var(ENV_WHITELIST_MODE).is_ok();

        let client = Client::builder()
            .gzip(true)
            .user_agent(&user_agent)
            .build()
            .map_err(Error::HttpClientError)?;

        Ok(Connection {
            server,
            access_token,
            user_agent,
            default_language,
            status_max_characters,
            status_max_medias,
            poll_max_options,
            whitelist_mode,
            client,
        })
    }

    /// Get the server URL.
    pub fn server_url(&self) -> &Url {
        &self.server
    }

    /// Get the URL that is joined server URL and `path`.
    pub fn url(&self, path: &str) -> Result<Url> {
        Ok(self.server.join(path)?)
    }

    /// Get the access token string.
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    /// Get the User-agent.
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Get the default language if present.
    pub fn default_language(&self) -> Option<Language> {
        self.default_language
    }

    /// Get the max number of characters of status.
    pub fn status_max_characters(&self) -> usize {
        self.status_max_characters
    }

    /// Get the max number of medias of status.
    pub fn status_max_medias(&self) -> usize {
        self.status_max_medias
    }

    /// Get the max number of options of poll.
    pub fn poll_max_options(&self) -> usize {
        self.poll_max_options
    }

    /// Get whether or not in the whitelist mode.
    pub fn whitelist_mode(&self) -> bool {
        self.whitelist_mode
    }

    // Get the reqwest::Client.
    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}

trait FromEnv<T> {
    fn from_env(env_name: &'static str, default: impl Into<String>) -> Result<T>;
}

impl FromEnv<usize> for usize {
    fn from_env(env_name: &'static str, default: impl Into<String>) -> Result<Self> {
        use std::env;
        use std::str::FromStr;

        usize::from_str(env::var(env_name)
            .map_or(default.into(), |s| s).as_str()
        )
        .map_err(|e| Error::ParseEnvVarError {
            source: e,
            env_var: env_name,
        })
    }
}

impl FromEnv<Url> for Url {
    fn from_env(env_name: &'static str, _default: impl Into<String>) -> Result<Self> {
        use std::env;

        let url = env::var(env_name).map_err(|e| Error::EnvVarError {
            source: e,
            env_var: env_name,
        })?;

        Ok(Url::parse(&url)?)
    }
}
