use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Deserialize)]
pub enum Scope {
    Read,
    ReadAccounts,
    ReadBlocks,
    ReadBookmarks,
    ReadFavourites,
    ReadFilters,
    ReadFollows,
    ReadLists,
    ReadMutes,
    ReadNotifications,
    ReadSearch,
    ReadStatuses,
    Write,
    WriteAccounts,
    WriteBlocks,
    WriteBookmarks,
    WriteConversations,
    WriteFavourites,
    WriteFilters,
    WriteFollows,
    WriteLists,
    WriteMedia,
    WriteMutes,
    WriteNotifications,
    WriteReports,
    WriteStatuses,
    Follow,
    Push,
    AdminRead,
    AdminReadAccounts,
    AdminReadReports,
    AdminWrite,
    AdminWriteAccounts,
    AdminWriteReports,
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Read => write!(f, "read"),
            Scope::ReadAccounts => write!(f, "read:accounts"),
            Scope::ReadBlocks => write!(f, "read:blocks"),
            Scope::ReadBookmarks => write!(f, "read:bookmarks"),
            Scope::ReadFavourites => write!(f, "read:favourites"),
            Scope::ReadFilters => write!(f, "read:filters"),
            Scope::ReadFollows => write!(f, "read:follows"),
            Scope::ReadLists => write!(f, "read:lists"),
            Scope::ReadMutes => write!(f, "read:mutes"),
            Scope::ReadNotifications => write!(f, "read:notifications"),
            Scope::ReadSearch => write!(f, "read:search"),
            Scope::ReadStatuses => write!(f, "read:statuses"),
            Scope::Write => write!(f, "write"),
            Scope::WriteAccounts => write!(f, "write:accounts"),
            Scope::WriteBlocks => write!(f, "write:blocks"),
            Scope::WriteBookmarks => write!(f, "write:bookmarks"),
            Scope::WriteConversations => write!(f, "write:conversations"),
            Scope::WriteFavourites => write!(f, "write:favourites"),
            Scope::WriteFilters => write!(f, "write:filters"),
            Scope::WriteFollows => write!(f, "write:follows"),
            Scope::WriteLists => write!(f, "write:lists"),
            Scope::WriteMedia => write!(f, "write:media"),
            Scope::WriteMutes => write!(f, "write:mutes"),
            Scope::WriteNotifications => write!(f, "write:notifications"),
            Scope::WriteReports => write!(f, "write:reports"),
            Scope::WriteStatuses => write!(f, "write:statuses"),
            Scope::Follow => write!(f, "follow"),
            Scope::Push => write!(f, "push"),
            Scope::AdminRead => write!(f, "admin:read"),
            Scope::AdminReadAccounts => write!(f, "admin:read:accounts"),
            Scope::AdminReadReports => write!(f, "admin:read:reports"),
            Scope::AdminWrite => write!(f, "admin:write"),
            Scope::AdminWriteAccounts => write!(f, "admin:write:accounts"),
            Scope::AdminWriteReports => write!(f, "admin:wirte:reports"),
        }
    }
}

impl TryFrom<&str> for Scope {
    type Error = crate::Error;

    fn try_from(value: &str) -> crate::Result<Self> {
        lazy_static!{
            static ref REVERSE_TABLE: HashMap<&'static str, Scope> = {
                let mut rt = HashMap::new();
                rt.insert("read", Scope::Read);
                rt.insert("read:accounts", Scope::ReadAccounts);
                rt.insert("read:blocks", Scope::ReadBlocks);
                rt.insert("read:bookmarks", Scope::ReadBookmarks);
                rt.insert("read:favourites", Scope::ReadFavourites);
                rt.insert("read:filters", Scope::ReadFilters);
                rt.insert("read:follows", Scope::ReadFollows);
                rt.insert("read:lists", Scope::ReadLists);
                rt.insert("read:mutes", Scope::ReadMutes);
                rt.insert("read:notifications", Scope::ReadNotifications);
                rt.insert("read:search", Scope::ReadSearch);
                rt.insert("read:statuses", Scope::ReadStatuses);
                rt.insert("write", Scope::Write);
                rt.insert("write:accounts", Scope::WriteAccounts);
                rt.insert("write:blocks", Scope::WriteBlocks);
                rt.insert("write:bookmarks", Scope::WriteBookmarks);
                rt.insert("write:conversations", Scope::WriteConversations);
                rt.insert("write:favourites", Scope::WriteFavourites);
                rt.insert("write:filters", Scope::WriteFilters);
                rt.insert("write:follows", Scope::WriteFollows);
                rt.insert("write:lists", Scope::WriteLists);
                rt.insert("write:media", Scope::WriteMedia);
                rt.insert("write:mutes", Scope::WriteMutes);
                rt.insert("write:notifications", Scope::WriteNotifications);
                rt.insert("write:reports", Scope::WriteReports);
                rt.insert("write:statuses", Scope::WriteStatuses);
                rt.insert("write:follow", Scope::Follow);
                rt.insert("push", Scope::Push);
                rt.insert("admin:read", Scope::AdminRead);
                rt.insert("admin:read:accounts", Scope::AdminReadAccounts);
                rt.insert("admin:read:reports", Scope::AdminReadReports);
                rt.insert("admin:write", Scope::AdminWrite);
                rt.insert("admin:write:accounts", Scope::AdminWriteAccounts);
                rt.insert("admin:write:reports", Scope::AdminWriteReports);
                rt
            };
        };

        match REVERSE_TABLE.get(value) {
            Some(scope) => Ok(scope.clone()),
            None => Err(
                crate::Error::ParseScopeError(value.to_owned())
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deconvert_scope() {
        assert_eq!("read", Scope::Read.to_string());
    }
    
    #[test]
    fn test_convert_scope() {
        assert_eq!(Scope::Write, Scope::try_from("write").unwrap());
    }

    #[test]
    fn test_convert_and_deconvert_read_scopes() {
        assert_eq!(Scope::Read, Scope::try_from(Scope::Read).unwrap());
        assert_eq!(Scope::ReadAccounts, Scope::try_from(Scope::ReadAccounts).unwrap());
        assert_eq!(Scope::ReadBlocks, Scope::try_from(Scope::ReadBlocks).unwrap());
        assert_eq!(Scope::ReadBookmarks, Scope::try_from(Scope::ReadBookmarks).unwrap());
        assert_eq!(Scope::ReadFavourites, Scope::try_from(Scope::ReadFavourites).unwrap());
        assert_eq!(Scope::ReadFilters, Scope::try_from(Scope::ReadFilters).unwrap());
        assert_eq!(Scope::ReadFollows, Scope::try_from(Scope::ReadFollows).unwrap());
        assert_eq!(Scope::ReadLists, Scope::try_from(Scope::ReadLists).unwrap());
        assert_eq!(Scope::ReadMutes, Scope::try_from(Scope::ReadMutes).unwrap());
        assert_eq!(Scope::ReadNotifications, Scope::try_from(Scope::ReadNotifications).unwrap());
        assert_eq!(Scope::ReadSearch, Scope::try_from(Scope::ReadSearch).unwrap());
        assert_eq!(Scope::ReadStatuses, Scope::try_from(Scope::ReadStatuses).unwrap());
    }

    #[test]
    fn test_convert_and_deconvert_write_scopes() {
        assert_eq!(Scope::Write, Scope::try_from(Scope::Write).unwrap());
        assert_eq!(Scope::WriteAccounts, Scope::try_from(Scope::WriteAccounts).unwrap());
        assert_eq!(Scope::WriteBlocks, Scope::try_from(Scope::WriteBlocks).unwrap());
        assert_eq!(Scope::WriteBookmarks, Scope::try_from(Scope::WriteBookmarks).unwrap());
        assert_eq!(Scope::WriteConversations, Scope::try_from(Scope::WriteConversations).unwrap());
        assert_eq!(Scope::WriteFavourites, Scope::try_from(Scope::WriteFavourites).unwrap());
        assert_eq!(Scope::WriteFilters, Scope::try_from(Scope::WriteFilters).unwrap());
        assert_eq!(Scope::WriteFollows, Scope::try_from(Scope::WriteFollows).unwrap());
        assert_eq!(Scope::WriteLists, Scope::try_from(Scope::WriteLists).unwrap());
        assert_eq!(Scope::WriteMedia, Scope::try_from(Scope::WriteMedia).unwrap());
        assert_eq!(Scope::WriteMutes, Scope::try_from(Scope::WriteMutes).unwrap());
        assert_eq!(Scope::WriteNotifications, Scope::try_from(Scope::WriteNotifications).unwrap());
        assert_eq!(Scope::WriteReports, Scope::try_from(Scope::WriteReports).unwrap());
        assert_eq!(Scope::WriteStatuses, Scope::try_from(Scope::WriteStatuses).unwrap());
    }

    #[test]
    fn test_convert_and_deconver_other_scopes() {
        assert_eq!(Scope::Follow, Scope::try_from(Scope::Follow).unwrap());
        assert_eq!(Scope::Push, Scope::try_from(Scope::Push).unwrap());
        assert_eq!(Scope::AdminRead, Scope::try_from(Scope::AdminRead).unwrap());
        assert_eq!(Scope::AdminReadAccounts, Scope::try_from(Scope::AdminReadAccounts).unwrap());
        assert_eq!(Scope::AdminReadReports, Scope::try_from(Scope::AdminReadReports).unwrap());
        assert_eq!(Scope::AdminWrite, Scope::try_from(Scope::AdminWrite).unwrap());
        assert_eq!(Scope::AdminWriteAccounts, Scope::try_from(Scope::AdminWriteAccounts).unwrap());
        assert_eq!(Scope::AdminWriteReports, Scope::try_from(Scope::AdminWriteReports).unwrap());
    }
}
