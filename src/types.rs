use std::{cell::RefCell, fmt::Debug, sync::Arc};

use serde_json::Value as JValue;

pub struct UserInfo {
    pub username: Arc<RefCell<String>>,
    pub token:    Arc<RefCell<String>>,
    pub factorio: Arc<RefCell<String>>, 
}

impl UserInfo {
    pub fn new(
	username: &str,
	token:    &str,
	factorio: &str,
    ) -> Self {
	UserInfo {
	    username: Arc::new(RefCell::new(username.into())),
	    token:    Arc::new(RefCell::new(token.into())),
	    factorio: Arc::new(RefCell::new(factorio.into())),
	}
    }
}

#[derive(Debug)]
pub struct ReleaseInfo {
    pub url:      String,
    pub file:     String,
    pub factorio: u64,
    pub version:  u64,
    #[cfg(feature = "check_sha1")]
    pub sha1:     String,
}

pub struct ModInfo {
    pub releases: Vec<ReleaseInfo>,
}

pub fn str_ver_to_u64(value: &str) -> u64 {
    let mut result: u64 = 0;
    let mut split = value.split(".").into_iter();
    for _ in 0..8 {
	result <<= 8;
	if let Some(value) = split.next() {
	    result += value.parse::<u64>().unwrap()
	};
    }
    result
}

macro_rules! to_string {
    ($v:expr) => { $v.as_str().unwrap().to_string() };
}

impl <'a> From<&'a JValue> for ReleaseInfo {
    fn from(jvalue: &'a JValue) -> Self {
	let jobject = jvalue.as_object().unwrap();

	Self {
	    url:      to_string!(jobject["download_url"]),
	    file:     to_string!(jobject["file_name"]),
	    factorio: {
		let info_json = jobject["info_json"].as_object().unwrap();
		let version = &info_json["factorio_version"];
		str_ver_to_u64(version.as_str().unwrap())
	    },
	    version:  {
		let version = &jobject["version"];
		str_ver_to_u64(version.as_str().unwrap())
	    },
	    #[cfg(feature = "check_sha1")]
	    sha1:     to_string!(jobject["sha1"]),
	}
    }
}

impl <'a> From<&'a JValue> for ModInfo {
    fn from(jvalue: &'a JValue) -> Self {
	let jobject = jvalue.as_object().unwrap();

	Self {
	    #[cfg(feature = "mod_info_name")]
	    name:     to_string!(["name"]),
	    releases: jobject["releases"]
		.as_array().unwrap()
		.into_iter()
		.map(|x| x.into())
		.rev()
		.collect(),
	}
    }
}

impl Debug for ModInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	self.releases.fmt(f)
    }
}

pub type Paths = Arc<Vec<String>>;

pub fn into_paths(
    paths: &Vec<Box<std::path::Path>>
)
    -> Paths
{
    Arc::new(
	paths.iter()
	    .map(|path| path.to_str().unwrap().to_owned())
	    .collect()
    )
}
