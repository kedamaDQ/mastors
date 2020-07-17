
/// Represent informations for pagination.
/// 
/// Some API methods of mastodon server such as `/api/v1/accounts/:id/followers` returns `Link` HTTP response header to using pagination controll.
/// This entity provides a raw `Link` Response header string and some convenient parsed parameters for pagination controll.

const REGEX_CAPTURE_ID: &str = r#"\?(?:min|max|since)_id=(\w+)>\s*;\s+rel="(next|prev)""#;

pub struct PageNavigation {
	raw: Option<String>,
	newest: Option<String>,
	oldest: Option<String>,
}

impl PageNavigation {
	pub fn new(link_header: Option<String>) -> Self {
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

	pub fn raw(&self) -> Option<&String> {
		self.raw.as_ref()
	}

	pub fn newest(&self) -> Option<&String> {
		self.newest.as_ref()
	}

	pub fn oldest(&self) -> Option<&String> {
		self.oldest.as_ref()
	}

	pub fn since_id(&self) -> Option<&String> {
		self.newest.as_ref()
	}

	pub fn min_id(&self) -> Option<&String> {
		self.newest.as_ref()
	}

	pub fn max_id(&self) -> Option<&String> {
		self.oldest.as_ref()
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
