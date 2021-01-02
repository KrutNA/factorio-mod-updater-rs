
use std::{ fs::{ File, self },
	   io::prelude::Write,
	   path::Path};

use crate::types::{Paths, UserInfo, into_paths};

pub fn save_to_file(
    path: &str,
    bytes: &[u8],
) {
    let mut file = File::create(path).unwrap();
    let _ = file.write_all(bytes);
}

pub fn find_mods(
    userinfo: UserInfo,
)
    -> Result<Paths, String>
{
    let path = &format!("{}/mods", userinfo.factorio.borrow());

    let dir = {
	let path = Path::new(path.into());

	if !path.exists() {
	    return Err("Couldn't find `mods` directory.".to_string());
	}

	fs::read_dir(path).map_err(|x| x.to_string())?
    };

    let mut paths = Vec::new();

    for entry in dir {
	let entry = entry.map_err(|x| x.to_string())?;
	if entry.file_type().unwrap().is_file() {
	    if entry.file_name().to_str().unwrap().ends_with(".zip") {
		paths.push(entry.path().into());
	    }
	} else {
	    paths.push(entry.path().into());
	};
    }

    Ok(into_paths(&paths))
}
