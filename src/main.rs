mod api;
mod types;
mod file_io;

use file_io::*;
use types::*;

fn main() {
    let args = &std::env::args()
	.map(|x| x.to_string())
	.collect::<Vec<_>>();

    let version  = str_ver_to_u64(&args[1]);
    let mod_name = &args[2];
    let path     = &args[3];
    let username = &args[4];
    let token    = &args[5];

    let userinfo = UserInfo::new(username, token, path);

    let releases = api::get_mod_info(mod_name)
	.expect("Mod not found")
	.releases;

    let release = releases.iter()
	.find(|x| x.factorio == version)
        .expect("Unknown factorio version or mod for this version not exists.");


    api::download_release(&release, &userinfo)
        .expect("Provided wrong token.");
}
