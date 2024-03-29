//! This module provides features related to poll attached to status.

/// This module provides features related to poll specified by ID.
pub mod id {
    use serde::Serialize;
    use crate:: {
        Connection,
        Method,
        entities::Poll,
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
        /// Set the authorization header to this request.
        /// 
        /// Authorization required if visibility of the parent status of the poll is set to private.
        pub fn authorized(&mut self) -> &Self {
            self.authorized = true;
            self
        }

        /// Unset the authorization header of this request.
        pub fn unauthorized(&mut self) -> &Self {
            self.authorized = false;
            self
        }
    }

    impl<'a> Method<'a, Poll> for GetPolls<'a> {}

    /// This module provides features related to vote to poll.
    pub mod votes {
        use std::collections::HashSet;
        use serde::Serialize;
        use crate::{
            Connection,
            Error,
            Method,
            Result,
            entities::Poll,
            private::MethodInternalWithoutRespHeader,
        };
        
        /// Get a request to vote to the poll.
        pub fn post(
            conn: &Connection,
            id: impl Into<String>,
            choices: impl AsRef<[usize]>,
        ) -> PostPolls {
        
            PostPolls {
                conn,
                id: id.into(),
                auth: true,
                choices: choices.as_ref()
                    .iter()
                    .map(|c| c.to_owned())
                    .collect::<Vec<usize>>(),
            }
        }
        
        /// POST request for `/api/v1/polls/:id`.
        /// 
        /// This request votes to poll specified by `id`.
        #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
        #[method_params(POST, Poll, "/api/v1/polls/_PATH_PARAM_/votes")]
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
        
            choices: Vec<usize>,
        }
        
        impl<'a> Method<'a, Poll> for PostPolls<'a> {
            fn send(&self) -> Result<Poll> {
                if self.choices.is_empty() {
                    return Err(Error::TooLittlePollOptionsError);
                }
        
                if self.choices.len() > self.conn.poll_max_options() {
                    return Err(Error::TooManyPollOptionsError(self.choices.len(), self.conn.poll_max_options()));
                }
        
                if self.choices.clone().into_iter().collect::<HashSet<usize>>().len() != self.choices.len() {
                    return Err(Error::DuplicateVoteOptionError)
                }
        
                self.send_internal()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Connection,
        Method,
    };

    #[test]
    fn test_vote_to_poll() {
        let conn = Connection::new().unwrap();
        let posted = crate::api::v1::statuses::post(&conn)
            .status("test_vote_to_poll")
            .poll(["a", "b", "c"], 3600)
            .poll_multiple()
            .send()
            .unwrap();
/* mastodon became to be error a self vote
        let voted = super::id::votes::post(&conn, posted.poll().unwrap().id(), [0, 1])
            .send()
            .unwrap();
*/
        let got = super::id::get(&conn, posted.poll().unwrap().id())
            .authorized()
            .send()
            .unwrap();

        let got_option_0 = got.options().iter().next().unwrap();

        assert_eq!(got_option_0.title(), "a");
        assert_eq!(got_option_0.votes_count().unwrap(), 0);

        let got = super::id::get(&conn, posted.poll().unwrap().id())
            .send()
            .unwrap();

        assert!(got.voted().is_none());

        let _deleted = crate::api::v1::statuses::id::delete(&conn, posted.id())
            .send()
            .unwrap();
    }
}
