use serde::Serialize;
use crate::{
    Connection,
    entities::Emojis,
    methods::Method,
};

/// Get all custom emojis registered on the server.
pub fn get(conn: &Connection) -> GetCustomEmojis {
    GetCustomEmojis {
        conn
    }
}

#[derive(Debug, Clone, Serialize, mastors_derive::Method)]
#[method_params(GET, Emojis, "/api/v1/custom_emojis")]
pub struct GetCustomEmojis<'a> {
    #[serde(skip_serializing)]
    #[mastors(connection)]
    conn: &'a Connection,
}

impl<'a> Method<'a, Emojis> for GetCustomEmojis<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_emojis() {
        let conn = Connection::new_with_path(".env.test").unwrap();
        get(&conn).send().unwrap();
    }
}
