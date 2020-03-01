pub mod api;

use async_trait::async_trait;
use serde::{
    self,
    Serialize,
};
use crate::{
    Connection,
    Error,
    Result,
    entities::Entity,
};

#[async_trait]
pub trait Method<'a, E: 'a + Entity>: std::marker::Sized + Serialize {
    const ENDPOINT: &'a str;

   async fn get(&'a self) -> Result<E> {
        Ok(
            send_request(
                build_request(self, reqwest::Method::GET)?.query(&self)
            )
            .await?
            .json::<E>()
            .await
            .unwrap()
        )
    }

    async fn post(&'a self) -> crate::Result<E> {
        Ok(
            send_request(
                build_request(self, reqwest::Method::POST)?.json(&self)
            )
            .await?
            .json::<E>()
            .await
            .unwrap()
        )
    }

    async fn send(&'a self) -> crate::Result<E>;

    fn path(&self) -> String {
        Self::ENDPOINT.to_string()
    }

    fn connection(&'a self) -> &'a Connection;

    fn authorization_code(&'a self) -> Option<&'a str>;
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FileForm<'a> {
    formname: &'a str,
    filename: &'a str,
}

#[async_trait]
pub trait Upload<'a, E: 'a + Entity>: Method<'a, E> {

    fn fileform(&self) -> FileForm;

    fn textforms(&self) -> Vec<(&str, &str)>;
 
    async fn post_with_media(&'a self) -> crate::Result<E> {
        use std::convert::TryFrom;
        use reqwest::multipart::{ Form, Part };
        use tokio::{
            fs::File,
            prelude::*,
        };

        // The documentation of reqwest says that `body () can receive std :: fs :: File`, but isn't code implement From<File>?
        let multipart = self.textforms().iter().fold(Form::new(), |mp, (name, value)| {
            mp.part((*name).to_owned(), Part::text((*value).to_owned()))
        });

        let mut file = File::open(self.fileform().filename).await?;
        let meta = file.metadata().await?;

        if !meta.is_file() {
            return Err(
                Error::NotFileError(self.fileform().filename.to_owned())
            );
        }

        if !meta.len() == 0 {
            return Err(
                Error::BlankFileError(self.fileform().filename.to_owned())
            );
        }

        let mut buf: Vec<u8> = match usize::try_from(meta.len()) {
            Ok(len) => Vec::with_capacity(len),
            Err(_) => Vec::new(),
        };

        file.read_to_end(&mut buf).await?;

        let multipart = multipart.part(
            self.fileform().formname.to_owned(),
            Part::bytes(buf).file_name(self.fileform().filename.to_owned())
        );

        Ok(
            send_request(
                build_request(self, reqwest::Method::POST)?.multipart(multipart)
            )
            .await?
            .json::<E>()
            .await
            .unwrap()
        )
    }
}

fn build_request<'a, E: Entity + 'a, M: Method<'a, E>>(
    implementer: &'a M,
    method: reqwest::Method
) -> crate::Result<reqwest::RequestBuilder> {

    let mut req = implementer.connection().client().request(
        method,
        implementer.connection().url(&implementer.path())?
    );

    if let Some(ac) = implementer.authorization_code() {
        req = req.bearer_auth(ac);
    }
    Ok(req)
}

async fn send_request(rb: reqwest::RequestBuilder) -> crate::Result<reqwest::Response> {
    let resp = rb.send().await?;

    if resp.status().is_success() {
        Ok(resp)
    } else {
        Err(
            Error::HttpStatusError(resp.status().as_u16(), resp.text().await?)
        )
    }
}

/*
use std::collections::HashSet;
use std::convert::TryFrom;
use serde::{Deserializer, de::Error};
use crate::scope::Scope;

#[derive(Deserialize, Debug, Clone)]
pub struct AccessToken {
    token: String,
    r#type: String,
    #[serde(deserialize_with = "transform_string_to_scope")]
    scopes: HashSet<Scope>,
    created_at: u32,
}

impl AccessToken {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn scopes(&self) -> &HashSet<Scope> {
        &self.scopes
    }
 
    pub fn scopes_string(&self) -> String {
        self.scopes.iter().map(
            |scope| scope.to_string()
        ).collect::<Vec<String>>().join(" ")
    }

    pub fn created_at(&self) -> u32 {
        self.created_at
    }

}

fn transform_string_to_scope<'de, D>(deserializer: D) -> Result<HashSet<Scope>, D::Error>
    where D: Deserializer<'de> {

    let s: &str = Deserialize::deserialize(deserializer)?;
    let mut scope_set: HashSet<Scope> = HashSet::new();

    for scope in s.split_whitespace() {
        match Scope::try_from(scope) {
            Ok(scp) => scope_set.insert(scp),
            Err(e) => return Err(D::Error::custom(e)),
        };
    }

    Ok(scope_set)
}
*/
#[cfg(test)]
mod tests {}
