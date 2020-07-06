//! This module provides features related to blocking accounts.

pub mod block {
    /// This module provides features related to block accounts.
	use serde::Serialize;
    use crate::{
    	Connection,
    	Method,
    	entities::Relationship,
    };
    
    /// Get a request to block an account specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostBlock {
    	PostBlock {
    		conn,
    		id: id.into(),
    		authorized: true
    	}
    }
    
    /// POST request for `/api/v1/accounts/:id/block`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/block")]
    pub struct PostBlock<'a> {
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
    
    impl<'a> Method<'a, Relationship> for PostBlock<'a> {}
}

pub mod unblock {
    /// This module provides features related to unblock accounts.
    use serde::Serialize;
    use crate::{
    	Connection,
    	Method,
    	entities::Relationship,
    };
    
    /// Get a request to unblock an account specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnblock {
    	PostUnblock {
    		conn,
    		id: id.into(),
    		authorized: true
    	}
    }
    
    /// POST request for `/api/v1/accounts/:id/unblock`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/unblock")]
    pub struct PostUnblock<'a> {
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
    
    impl<'a> Method<'a, Relationship> for PostUnblock<'a> {}
}

#[cfg(test)]
mod tests {
	// fmm...
}
