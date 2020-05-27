//! This module provides features related to poll attached to status.
use std::collections::HashSet;
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

/// Get a request to vote to the poll.
pub fn post(
    conn: &Connection,
    id: impl Into<String>,
    choices: impl Into<HashSet<usize>>,
) -> PostPolls {

    PostPolls {
        conn,
        id: id.into(),
        auth: true,
        choices: choices.into(),
    }
}

/// Get a request to get a poll specified by ID.
/// 
/// This method is an alias of `mastors::api::v1::polls::id::get()`.
pub use id::get as get_by_id;

/// POST request for `/api/v1/polls/:id`.
/// 
/// This request votes to poll specified by `id`.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(POST, Poll, "/api/v1/polls/_PATH_PARAM_")]
pub struct PostPolls<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(path_param)]
    id: String,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

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
        Ok(self.send_internal()?)
    }
}

/// This module provides features related to status of a poll.
pub mod id {
    use serde::Serialize;
    use crate:: {
        Connection,
        entities::Poll,
        methods::Method,
    };

    /// Get a request to get a poll specified by ID.
    pub fn get(conn: &Connection, id: impl Into<String>) -> GetPolls {
        GetPolls {
            conn,
            id: id.into(),
            authorized: false,
        }
    }

    /// GET request for `/api/v1/polls/:id`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Poll, "/api/v1/polls/_PATH_PARAM_")]
    pub struct GetPolls<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,

        #[serde(skip_serializing)]
        #[mastors(path_param)]
        id: String,

        #[serde(skip_serializing)]
        #[mastors(authorization)]
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

    impl<'a> Method<'a, Poll> for GetPolls<'a> {}
}

#[cfg(test)]
mod tests {

}
