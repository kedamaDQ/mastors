//! This module provides features related to parent and child statuses of the status reply tree.

/// This module provides features related to get context of the status.
pub mod context {
    use serde::Serialize;
    use crate::{
        Connection,
        Method,
        entities::Context,
    };

    /// Get a request to get statuses that are ancestors or descendants of status specified by id. 
    pub fn get(conn: &Connection, id: impl Into<String>) -> GetContext {
        GetContext {
            conn,
            id: id.into(),
            authorized: false,
        }
    }

    /// GET request for `/api/v1/statuses/:id/context`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Context, "/api/v1/statuses/_PATH_PARAM_/context")]
    pub struct GetContext<'a> {
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

    impl<'a> GetContext<'a> {
        /// Set the Authorization HTTP request header.
        /// You can get private status if set.
        /// `false` is defaults.
        pub fn authorized(mut self) -> Self {
            self.authorized = true;
            self
        }
    }

    impl<'a> Method<'a, Context> for GetContext<'a> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Method;

    #[test]
    fn test_get_context() {
        use crate::api::v1::statuses;

        let conn = crate::Connection::new().unwrap();

        let first = statuses::post(&conn, "first").send().unwrap().status().unwrap();
        let second = statuses::post(&conn, "second").in_reply_to_id(first.id()).send().unwrap().status().unwrap();
        let third = statuses::post(&conn, "third").in_reply_to_id(second.id()).send().unwrap().status().unwrap();
        let fourth = statuses::post(&conn, "fourth").in_reply_to_id(third.id()).send().unwrap().status().unwrap();

        let contexts = context::get(&conn, third.id()).send().unwrap();
        let ancestors = contexts.ancestors().iter().map(|status| status.id().to_owned()).collect::<Vec<String>>();
        let descendants = contexts.descendants().iter().map(|status| status.id().to_owned()).collect::<Vec<String>>();

        assert_eq!(ancestors.len(), 2);
        assert_eq!(vec![first.id(), second.id()], ancestors);

        assert_eq!(descendants.len(), 1);
        assert_eq!(vec![fourth.id()], descendants);

        statuses::id::delete(&conn, first.id()).send().unwrap();
        statuses::id::delete(&conn, second.id()).send().unwrap();
        statuses::id::delete(&conn, third.id()).send().unwrap();
        statuses::id::delete(&conn, fourth.id()).send().unwrap();
    }
}
