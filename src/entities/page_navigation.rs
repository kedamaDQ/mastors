// Regex for `Link` HTTP response header with captureing `id` and `rel`.
const REGEX_CAPTURE_ID: &str = r#"\?(?:min|max|since)_id=(\w+)>\s*;\s+rel="(next|prev)""#;

/// Represent informations for pagination.
/// 
/// Some API methods of mastodon server such as `/api/v1/accounts/:id/followers` returns `Link` HTTP response header to using pagination controll.
/// This entity provides a raw `Link` Response header string and some convenient parsed parameters for pagination controll.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct PageNavigation {
	raw: Option<String>,
	newest: Option<String>,
	oldest: Option<String>,
}

impl PageNavigation {
	// Create new parsed Link header
	pub(crate) fn new(link_header: Option<String>) -> Self {
		let mut pn = PageNavigation {
			raw: link_header.clone(),
			newest: None,
			oldest: None,
		};

		if let Some(link_header) = link_header {
    		match regex::Regex::new(REGEX_CAPTURE_ID) {
    			Ok(regex) => regex,
    			Err(e) => panic!("Probably this is a mastors bug!!: {}", e),
    		}
    		.captures_iter(&link_header)
    		.for_each(|caps| {
    			match &caps[2] {
    				"next" => pn.oldest = Some(caps[1].to_string()),
    				"prev" => pn.newest = Some(caps[1].to_string()),
    				_ => panic!("Probably this is a mastors bug!!: Unknown 'rel' in Link header"),
    			}
    		});
    	}

		pn
	}

	/// Get the raw body body of `Link` HTTP response header.
	pub fn raw(&self) -> Option<&str> {
		self.raw.as_deref()
	}

	/// Get the latest ID in the acquired list.
	pub fn newest(&self) -> Option<&str> {
		self.newest.as_deref()
	}

	/// Get the oldes ID in the acquired list.
	pub fn oldest(&self) -> Option<&str> {
		self.oldest.as_deref()
	}

	/// Get an ID to set to  `since_id` for get the previous page.
	pub fn since_id(&self) -> Option<&str> {
		self.newest.as_deref()
	}

	/// Get an ID to set to  `min_id` for get the previous page.
	pub fn min_id(&self) -> Option<&str> {
		self.newest.as_deref()
	}

	/// Get an ID to set to  `max_id` for get the next page.
	pub fn max_id(&self) -> Option<&str> {
		self.oldest.as_deref()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_next_prev() {
		let s = "<https://foresdon.jp/api/v1/accounts/1/followers?max_id=2777>; rel=\"next\", <https://foresdon.jp/api/v1/accounts/1/followers?since_id=3142>; rel=\"prev\"";

		let pn = PageNavigation::new(Some(s.to_owned()));
		assert_eq!(pn.raw().unwrap(), s);
		assert_eq!(pn.newest().unwrap(), "3142");
		assert_eq!(pn.oldest().unwrap(), "2777");
	}

	#[test]
	fn test_next_only() {
		let s = "<https://foresdon.jp/api/v1/accounts/1/followers?max_id=2777>; rel=\"next\",";

		let pn = PageNavigation::new(Some(s.to_owned()));
		assert_eq!(pn.raw().unwrap(), s);
		assert_eq!(pn.newest(), None);
		assert_eq!(pn.oldest().unwrap(), "2777");
	}

	#[test]
	fn test_prev_only() {
		let s = ", <https://foresdon.jp/api/v1/accounts/1/followers?since_id=3142>; rel=\"prev\"";

		let pn = PageNavigation::new(Some(s.to_owned()));
		assert_eq!(pn.raw().unwrap(), s);
		assert_eq!(pn.newest().unwrap(), "3142");
		assert_eq!(pn.oldest(), None);
	}

	#[test]
	fn test_none() {
		let pn = PageNavigation::new(None);
		assert_eq!(pn.raw(), None);
		assert_eq!(pn.newest(), None);
		assert_eq!(pn.oldest(), None);
	}
}
