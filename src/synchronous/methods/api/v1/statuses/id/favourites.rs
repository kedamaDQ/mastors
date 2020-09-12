//! This method provides features related to controlling favourite for statuses.

/// This module provides features related to get accounts who favourited status.
pub mod favourited_by {
    use serde::Serialize;
    use crate::{
        Connection,
        Method,
        entities::Accounts,
    };

    /// Get a request to get accounts who favourited status specified by `id`.
    pub fn get(conn: &Connection, id: impl Into<String>) -> GetFavouritedBy{
        GetFavouritedBy{
            conn,
            id: id.into(),
            authorized: true,
        }
    }

    /// GET request for `/api/v1/statuses/:id/favourited_by`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Accounts, "/api/v1/statuses/_PATH_PARAM_/favourited_by")]
    pub struct GetFavouritedBy<'a> {
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

    impl<'a> Method<'a, Accounts> for GetFavouritedBy<'a> {}
}

/// This module provides features related to set favourite of the status.
pub mod favourite {
    use serde::Serialize;
    use crate::{
        Connection,
        Method,
        entities::Status,
    };

    /// Get a request to set favourite to a status specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostFavourite {
        PostFavourite {
            conn,
            id: id.into(),
            authorized: true,
        }
    }

    /// POST request for `/api/v1/statuses/:id/favourite`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/favourite")]
    pub struct PostFavourite<'a> {
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

    impl<'a> Method<'a, Status> for PostFavourite<'a> {}
}

/// This module provides features related to unset favourite of the status.
pub mod unfavourite {
    use serde::Serialize;
    use crate::{
        Connection,
        Method,
        entities::Status,
    };

    /// Get a request to unset favourite of a status specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnfavourite {
        PostUnfavourite {
            conn,
            id: id.into(),
            authorized: true,
        }
    }

    /// POST request for `/api/v1/statuses/:id/unfavourite`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Status, "/api/v1/statuses/_PATH_PARAM_/unfavourite")]
    pub struct PostUnfavourite<'a> {
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

    impl<'a> Method<'a, Status> for PostUnfavourite<'a> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Method;

    #[test]
    fn test_favourite_unfavourite_status() {
        let conn = crate::Connection::new().unwrap();
        let status = crate::api::v1::statuses::post(&conn)
            .status("favourite unfavourite.")
            .send()
            .unwrap();
        let myself = status.account();

        let favourited = favourite::post(&conn, status.id()).send().unwrap();
        assert_eq!(status.id(), favourited.id());
        assert!(favourited.favourited());
        
        let favourited_by = favourited_by::get(&conn, status.id()).send().unwrap();
        assert_eq!(favourited_by.len(), 1);
        assert_eq!(favourited_by.get(0).unwrap().id(), myself.id());

        let unfavourited = unfavourite::post(&conn, status.id()).send().unwrap();
        assert_eq!(status.id(), unfavourited.id());
        assert!(! unfavourited.favourited());

        let favourited_by = favourited_by::get(&conn, status.id()).send().unwrap();
        assert!(favourited_by.is_empty());

        crate::api::v1::statuses::id::delete(&conn, status.id()).send().unwrap();
    }
}
