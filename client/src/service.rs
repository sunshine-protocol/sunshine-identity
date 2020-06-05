use crate::error::Result;
use crate::github;
use core::str::FromStr;
use libipld::DagCbor;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, DagCbor)]
pub enum Service {
    Github(String),
}

impl Service {
    pub fn username(&self) -> &str {
        match self {
            Self::Github(username) => &username,
        }
    }

    pub fn service(&self) -> &str {
        match self {
            Self::Github(_) => "github",
        }
    }

    pub async fn verify(&self, signature: &str) -> Result<String> {
        match self {
            Self::Github(user) => github::verify(&user, signature).await
        }
    }

    pub async fn resolve(&self) -> Result<Vec<String>> {
        match self {
            Self::Github(user) => github::resolve(&user).await
        }
    }

    pub fn proof(&self, account_id: &str, object: &str, signature: &str) -> String {
        match self {
            Self::Github(user) => github::proof(&user, account_id, object, signature),
        }
    }
}

impl core::fmt::Display for Service {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}@{}", self.username(), self.service())
    }
}

impl FromStr for Service {
    type Err = ServiceParseError;

    fn from_str(string: &str) -> core::result::Result<Self, Self::Err> {
        let mut parts = string.split("@");
        let username = parts.next().ok_or(ServiceParseError::Invalid)?;
        if username.is_empty() {
            return Err(ServiceParseError::Invalid);
        }
        let service = parts.next().ok_or(ServiceParseError::Invalid)?;
        if service.is_empty() {
            return Err(ServiceParseError::Invalid);
        }
        if parts.next().is_some() {
            return Err(ServiceParseError::Invalid);
        }
        match service {
            "github" => Ok(Self::Github(username.into())),
            _ => Err(ServiceParseError::Unknown(service.into())),
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ServiceParseError {
    #[error("Expected a service description of the form username@service.")]
    Invalid,
    #[error("Unknown service '{0}'")]
    Unknown(String),
}
