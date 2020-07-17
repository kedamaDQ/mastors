//! This module provides features related to following and followers.

/// This module provides features related to followers of an account specified by ID.
pub mod followers {
    use serde::Serialize;
    use crate::{
        Connection,
        MethodWithRespHeader as Method,
        entities::Accounts,
    };
    
    /// Get a request to get followers of an account specified by `id`.
    pub fn get(conn: &Connection, id: impl Into<String>) -> GetAccountFollowers {
        GetAccountFollowers {
            conn,
            id: id.into(),
            authorized: true,
            max_id: None,
            since_id: None,
            limit: None,
        }
    }
    
    /// GET request for `/api/v1/accounts/:id/followers`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Accounts, "/api/v1/accounts/_PATH_PARAM_/followers", "Link")]
    pub struct GetAccountFollowers<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,
    
        #[serde(skip_serializing)]
        #[mastors(path_param)]
        id: String,
    
        #[serde(skip_serializing)]
        #[mastors(authorization)]
        authorized: bool,
    
        max_id: Option<String>,
        since_id: Option<String>,
        limit: Option<usize>,
    }
    
    impl<'a> GetAccountFollowers<'a> {
        /// Set the max ID of the follower accounts to get.
        pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
            self.max_id = Some(max_id.into());
            self
        }
    
        /// Set the since ID of the follower accounts to get.
        pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
            self.since_id = Some(since_id.into());
            self
        }
    
        /// Set a number of the follower accounts to get.
        /// 
        /// If not set, 40 by default.
        pub fn limit(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
    }
    
    impl<'a> Method<'a, Accounts> for GetAccountFollowers<'a> {}
}

/// This module provides features related to accounts that the account specified by ID is following.
pub mod following {
    use serde::Serialize;
    use crate::{
        Connection,
        MethodWithRespHeader as Method,
        entities::Accounts,
    };
    
    /// Get a request to get following of an account specified by `id`.
    pub fn get(conn: &Connection, id: impl Into<String>) -> GetAccountFollowing {
        GetAccountFollowing {
            conn,
            id: id.into(),
            authorized: true,
            max_id: None,
            since_id: None,
            limit: None,
        }
    }
    
    /// GET request for `/api/v1/accounts/:id/following`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, Accounts, "/api/v1/accounts/_PATH_PARAM_/following", "Link")]
    pub struct GetAccountFollowing<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,
    
        #[serde(skip_serializing)]
        #[mastors(path_param)]
        id: String,
    
        #[serde(skip_serializing)]
        #[mastors(authorization)]
        authorized: bool,
    
        max_id: Option<String>,
        since_id: Option<String>,
        limit: Option<usize>,
    }
    
    impl<'a> GetAccountFollowing<'a> {
        /// Set the max ID of the following accounts to get.
        pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
            self.max_id = Some(max_id.into());
            self
        }
    
        /// Set the since ID of the following accounts to get.
        pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
            self.since_id = Some(since_id.into());
            self
        }
    
        /// Set a number of the following accounts to get.
        pub fn limit(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
    }
    
    impl<'a> Method<'a, Accounts> for GetAccountFollowing<'a> {}
}

/// This module provides features related to follow the account.
pub mod follow {
    use serde::Serialize;
    use crate::{
        Connection,
        Method,
        entities::Relationship,
    };
    
    /// Get a request to follow an account specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostFollow {
        PostFollow {
            conn,
            id: id.into(),
            authorized: true,
            reblogs: None,
        }
    }
    
    /// POST request for `/api/v1/accounts/:id/follow`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/follow")]
    pub struct PostFollow<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,
    
        #[serde(skip_serializing)]
        #[mastors(path_param)]
        id: String,
    
        #[serde(skip_serializing)]
        #[mastors(authorization)]
        authorized: bool,
    
        // Optional params
        reblogs: Option<bool>,
    }
    
    impl<'a> PostFollow<'a> {
        /// Set to show reblogs from account specified by ID.
        pub fn reblogs(mut self) -> Self {
            self.reblogs = Some(true);
            self
        }
    }
    
    impl<'a> Method<'a, Relationship> for PostFollow<'a> {}
}

/// This module provides features related to unfollow account.
pub mod unfollow {
    use serde::Serialize;
    use crate::{
    	Connection,
    	Method,
    	entities::Relationship,
    };
    
    /// Get a request to unfollow account specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnfollow {
    	PostUnfollow {
    		conn,
    		id: id.into(),
    		authorized: true
    	}
    }
    
    /// POST request for `/api/v1/accounts/:id/unfollow`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/unfollow")]
    pub struct PostUnfollow<'a> {
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
    
    impl<'a> Method<'a, Relationship> for PostUnfollow<'a> {}
}

#[cfg(test)]
mod tests {
    use super::*;
	use crate::{
		Connection,
		MethodWithRespHeader,
	};

    #[test]
    fn test_get_accounts_followers() {
        let conn = Connection::new().unwrap();
        let _got = followers::get(&conn, "1")
            .limit(1)
            .send()
            .unwrap();
	}
	
    #[test]
    fn test_get_accounts_following() {
        let conn = Connection::new().unwrap();
        let _got = following::get(&conn, "1")
            .limit(1)
            .send()
            .unwrap();
    }

}
