//! This module provides features related to muting accounts.

/// This module provides features related to mute accounts.
pub mod mute {
    use serde::Serialize;
    use crate::{
    	Connection,
    	Method,
    	entities::Relationship,
    };
    
    /// Get a request to mute an account specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostMute {
    	PostMute {
    		conn,
    		id: id.into(),
    		authorized: true,
    		notifications: None,
    	}
    }
    
    /// POST request for `/api/v1/accounts/:id/mute`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/mute")]
    pub struct PostMute<'a> {
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
    	notifications: Option<bool>,
    }
    
    impl<'a> PostMute<'a> {
    	/// Set to also mute account related notifications. Defaults to true.
    	pub fn without_notifications(mut self) -> Self {
    		self.notifications = Some(false);
    		self
    	}
    }
    
    impl<'a> Method<'a, Relationship> for PostMute<'a> {}
}

/// This module provides features related to unmute accounts.
pub mod unmute {
    use serde::Serialize;
    use crate::{
    	Connection,
    	Method,
    	entities::Relationship,
    };
    
    /// Get a request to unmute an account specified by `id`.
    pub fn post(conn: &Connection, id: impl Into<String>) -> PostUnmute {
    	PostUnmute {
    		conn,
    		id: id.into(),
    		authorized: true
    	}
    }
    
    /// POST request for `/api/v1/accounts/:id/unmute`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(POST, Relationship, "/api/v1/accounts/_PATH_PARAM_/unmute")]
    pub struct PostUnmute<'a> {
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
    
    impl<'a> Method<'a, Relationship> for PostUnmute<'a> {}
}

#[cfg(test)]
mod tests {
	// fmm...
}
