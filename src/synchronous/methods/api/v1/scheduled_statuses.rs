//! This module provides features related to scheduled statuses that is posted by authenticated user.
use serde::Serialize;
use crate::{
    Connection,
    Method,
    entities::ScheduledStatuses,
};

/// Get a request to get scheduled statuses that are created by the authenticated user.
pub fn get(conn: &Connection) -> GetScheduledStatuses {
    GetScheduledStatuses {
        conn,
        auth: true,
        limit: None,
        max_id: None,
        since_id: None,
        min_id: None,
    }
}

/// GET request for scheduled statuses that are created by authenticated user.
#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, ScheduledStatuses, "/api/v1/scheduled_statuses")]
pub struct GetScheduledStatuses<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,

    #[serde(skip_serializing)]
    #[mastors(authorization)]
    auth: bool,

    limit: Option<usize>,
    max_id: Option<String>,
    since_id: Option<String>,
    min_id: Option<String>,
}

impl<'a> GetScheduledStatuses<'a> {
    /// Set the number of scheduled statuses to get. Defaults to 20.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set to return scheduled statuses older than ID.
    pub fn max_id(mut self, max_id: impl Into<String>) -> Self {
        self.max_id = Some(max_id.into());
        self
    }

    /// Set to return scheduled statuses newer than ID.
    pub fn since_id(mut self, since_id: impl Into<String>) -> Self {
        self.since_id = Some(since_id.into());
        self
    }

    /// Set to return scheduled statuses immediately newer than ID.
    pub fn min_id(mut self, min_id: impl Into<String>) -> Self {
        self.min_id = Some(min_id.into());
        self
    }
}

impl<'a> Method<'a, ScheduledStatuses> for GetScheduledStatuses<'a> {}

/// This module provides features related to manipulate a scheduled status that is specified by ID posted by authenticated user.
pub mod id {
    use serde::Serialize;
    use crate::{
        Connection,
        DateTime,
        Method,
        Utc,
        entities::{
            DeletedScheduledStatus,
            ScheduledStatus,
        },
    };

    /// Get a scheduled status specified by `id`.
    pub fn get(conn: &Connection, id: impl Into<String>) -> GetScheduledStatuses {
        GetScheduledStatuses {
            conn,
            auth: true,
            id: id.into(),
        }
    }

    /// Update a scheduled date and time of scheduled status specified by `id`.
    pub fn put(conn: &Connection, id: impl Into<String>) -> PutScheduledStatuses {
        PutScheduledStatuses {
            conn,
            auth: true,
            id: id.into(),
            scheduled_at: None,
        }
    }

    /// Delete a scheduled status specified by `id`.
    pub fn delete(conn: &Connection, id: impl Into<String>) -> DeleteScheduledStatuses {
        DeleteScheduledStatuses {
            conn,
            auth: true,
            id: id.into(),
        }
    }

    /// GET request for `/api/v1/scheduled_statuses/:id`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(GET, ScheduledStatus, "/api/v1/scheduled_statuses/_PATH_PARAM_")]
    pub struct GetScheduledStatuses<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,

        #[serde(skip_serializing)]
        #[mastors(authorization)]
        auth: bool,

        #[serde(skip_serializing)]
        #[mastors(path_param)]
        id: String,
    }

    impl<'a> Method<'a, ScheduledStatus> for GetScheduledStatuses<'a> {}

    /// PUT request for `/api/v1/scheduled_statuses/:id`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(PUT, ScheduledStatus, "/api/v1/scheduled_statuses/_PATH_PARAM_")]
    pub struct PutScheduledStatuses<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,

        #[serde(skip_serializing)]
        #[mastors(authorization)]
        auth: bool,

        #[serde(skip_serializing)]
        #[mastors(path_param)]
        id: String,

        scheduled_at: Option<DateTime<Utc>>,
    }

    impl<'a> PutScheduledStatuses<'a> {
        /// Update `scheduled_at` of this scheduled status.
        /// 
        /// `DateTime<Utc>`, the type of `scheduled_at`, is re-export from [`chrono`](https://docs.rs/chrono/). For example to create a `DateTime<Utc>` of **NOW**:
        /// 
        /// ```rust
        /// use mastors::{ DateTime, Utc };
        /// 
        /// let now: DateTime<Utc> = Utc::now();
        /// ```
        /// Refer to the [original document](https://docs.rs/chrono/) for details.
        pub fn scheduled_at(mut self, scheduled_at: DateTime<Utc>) -> Self {
            self.scheduled_at = Some(scheduled_at);
            self
        }
    }

    impl<'a> Method<'a, ScheduledStatus> for PutScheduledStatuses<'a> {}

    /// DELETE request for `/api/v1/scheduled_statuses/:id`.
    #[derive(Debug, Clone, Serialize, mastors_derive::Method)]
    #[method_params(DELETE, DeletedScheduledStatus, "/api/v1/scheduled_statuses/_PATH_PARAM_")]
    pub struct DeleteScheduledStatuses<'a> {
        #[serde(skip_serializing)]
        #[mastors(connection)]
        conn: &'a Connection,

        #[serde(skip_serializing)]
        #[mastors(authorization)]
        auth: bool,

        #[serde(skip_serializing)]
        #[mastors(path_param)]
        id: String,
    }

    impl<'a> Method<'a, DeletedScheduledStatus> for DeleteScheduledStatuses<'a> {}

    #[cfg(test)]
    mod tests {
        //! Tests are executed collectively with `crate::api::v1::statuses`.
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Utc;
    use crate::api::v1::statuses;

    #[test]
    fn test_get_scheduled_status() {
        let conn = Connection::new().unwrap();
        let scheduled_at = Utc::now() + chrono::Duration::seconds(310);

        // Clear all existing scheduled statuses.
        let got = get(&conn)
            .send()
            .unwrap();

        for ss in got.iter() {
            id::delete(&conn, ss.id()).send().unwrap();
        }

        let posted1 = statuses::post(&conn)
            .status("first")
            .scheduled_at(scheduled_at)
            .send()
            .unwrap();
        let posted2 = statuses::post(&conn)
            .status("second")
            .scheduled_at(scheduled_at)
            .send()
            .unwrap();
        let posted3 = statuses::post(&conn)
            .status("third")
            .scheduled_at(scheduled_at)
            .send()
            .unwrap();
        let posted4 = statuses::post(&conn)
            .status("fourth")
            .scheduled_at(scheduled_at)
            .send()
            .unwrap();
        let posted5 = statuses::post(&conn)
            .status("fifth")
            .scheduled_at(scheduled_at)
            .send()
            .unwrap();

        let mut posted_ids = vec![
            posted1.id().to_string(),
            posted2.id().to_string(),
            posted3.id().to_string(),
            posted4.id().to_string(),
            posted5.id().to_string()
        ];
        posted_ids.sort();

        let got = get(&conn)
            .send()
            .unwrap();

        let mut got_ids = got
            .iter()
            .map(|ss| ss.id().to_string())
            .collect::<Vec<String>>();
        got_ids.sort();

        assert_eq!(posted_ids, got_ids);

        for ss in got_ids {
            id::delete(&conn, ss)
                .send()
                .unwrap();
        }
    }
}
