use serde::Serialize;
use crate::{
    Connection,
    Error,
    Result,
    entities::Poll,
    methods::{
        Method,
        MethodInternal,
    },
};

pub fn get(conn: &Connection, id: impl Into<String>) -> GetPolls {
    GetPolls {
        conn,
        id: id.into(),
        authorized: false,
    }
}

use std::collections::HashSet;

pub fn post(
    conn: &Connection,
    id: impl Into<String>,
    choices: impl Into<HashSet<usize>>,
) -> PostPolls {

    PostPolls {
        conn,
        id: id.into(),
        choices: choices.into(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetPolls<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    id: String,
    authorized: bool,
}

impl<'a> GetPolls<'a> {
    pub fn authorized(&mut self) -> &Self {
        self.authorized = true;
        self
    }

    pub fn unauthorized(&mut self) -> &Self {
        self.authorized = false;
        self
    }
}

impl<'a> Method<'a, Poll> for GetPolls<'a> {
    fn send(&self) -> Result<Poll> {
        Ok(self.get()?)
    }
}

impl<'a> MethodInternal<'a, Poll> for GetPolls<'a> {
    const ENDPOINT: &'a str = "/api/v1/polls";

    fn connection(&self) -> &Connection {
        &self.conn
    }

    fn path(&self) -> String {
        format!("{}{}", Self::ENDPOINT, &self.id)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PostPolls<'a> {
    #[serde(skip_serializing)]
    conn: &'a Connection,
    id: String,
    choices: HashSet<usize>,
}

impl<'a> Method<'a, Poll> for PostPolls<'a> {
    fn send(&self) -> Result<Poll> {
        if self.choices.is_empty() {
            return Err(Error::TooLittlePollOptionsError);
        }
    
        if self.choices.len() > self.conn.poll_max_options() {
            return Err(Error::TooManyPollOptionsError(self.choices.len(), self.conn.poll_max_options()));
        }
        Ok(self.post()?)
    }
}

impl<'a> MethodInternal<'a, Poll> for PostPolls<'a> {
    const ENDPOINT: &'a str = "api/v1/polls";

    fn connection(&self) -> &Connection {
        &self.conn
    }

    fn path(&self) -> String {
        format!("{}{}", Self::ENDPOINT, self.id)
    }

    fn authorization(&self) -> Option<&'a str> {
        Some(&self.conn.access_token())
    }
}
