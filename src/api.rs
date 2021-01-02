const API_DOWNLOAD: &'static str = "direct.mods-data.factorio.com";
const API_HOST: &'static str = "mods.factorio.com";
const API_PORT: u16 = 443;
const DIFF: u16 = b'0' as u16 * 100 + b'0' as u16 * 10 + b'0' as u16;

use crate::{ ModInfo,
	     ReleaseInfo,
	     UserInfo, };
use crate::save_to_file;

use std::{io::{ Read, Write },
	  net::TcpStream};

use serde_json::Value as JValue;
use native_tls::{ TlsStream as Stream,
		  TlsConnector as Connector };

fn get_stream(
    host: &str,
) -> Stream<TcpStream> {
    let connector = Connector::new().unwrap();
    let stream = TcpStream::connect((host, API_PORT)).unwrap();
    connector.connect(host, stream).unwrap()
}

pub fn get_mod_info(
    name: &str,
)
    -> Result<ModInfo, String>
{
    let endpoint = format!("/api/mods/{}", name);

    let result = &call_mod_api(&endpoint)?;
    let result = &result[skip_header(&result)?..];
    let result = std::str::from_utf8(result).unwrap();
    let result = &serde_json::from_str::<JValue>(result).unwrap();

    Ok(result.into())
}

pub fn download_release(
    release: &ReleaseInfo,
    userinfo: &UserInfo,
)
    -> Result<(), String>
{
    let link = &{
	let endpoint = &format!(
	    "{download}?username={username}&token={token}",
	    download = release.url,
	    username = userinfo.username.borrow(),
	    token    = userinfo.token.borrow(),
	);

	let response = {
	    let response = call_mod_api(endpoint)?;
	    std::str::from_utf8(&response[..]).unwrap().to_owned()
	};
	
	response.split("\r\n")
	    .find(|x| x.starts_with("Location: "))
	    .unwrap()
	    .split("factorio.com")
	    .nth(1)
	    .unwrap()
	    .to_owned()
    };

    let archive = download_by_link(link);

    #[cfg(feature = "chech_sha1")]
    {
	if sha1::Sha1::from(archive).hexdigest() != release.sha1 {
	    return Err("Hashes not equals.".into());
	}
    }

    {
	let path = format!("{factorio}/mods/{mod_name}",
			   factorio = userinfo.factorio.borrow(),
			   mod_name = release.file, );

	save_to_file(&path, &archive[..]);
    }

    Ok(())
}

fn call_mod_api(
    endpoint: &str,
)
    -> Result<Vec<u8>, String>
{
    let mut stream = get_stream(API_HOST);
    {

	let request = format!(
	    "GET {} HTTP/1.0\r
Host: mods.factorio.com\r
\r
",
	    endpoint,
	);

	print!("{}", request);

	stream.write_all(request.as_bytes()).unwrap();
    }
    
    {
	let mut buf = vec![];
	stream.read_to_end(&mut buf).unwrap();
	
	// print!("{}", std::str::from_utf8(buf.as_slice()).unwrap());

	if buf.len() == 0 {
	    return Err("Response is empty.".into());
	}

	return Ok(buf);
    }
}

fn download_by_link(
    link: &str
)
    -> Vec<u8>
{
    let mut stream = get_stream(API_DOWNLOAD);

    let request = format!(
	"GET {} HTTP/1.0\r
Host: direct.mods-data.factorio.com\r
\r
",
	link,
    );

    print!("{}", request);
    stream.write_all(request.as_bytes()).unwrap();

    let mut buf = vec![];
    stream.read_to_end(&mut buf).unwrap();

    return buf[skip_header(&buf).unwrap()..].to_vec();
}


fn skip_header(
    buf: &Vec<u8>
)
    -> Result<usize, String>
{
    let return_code
	= buf[9]  as u16 * 100
	+ buf[10] as u16 * 10
	+ buf[11] as u16
	- DIFF;

    check_return_code(return_code)?;
    
    for i in 12..buf.len() {
	if buf[i] == b'\r' && buf[i+2] == b'\r' {
	    return Ok(i + 4);
	}
    }
    unreachable!();
}

// TODO: create normal erorr catching
fn check_return_code(code: u16) -> Result<(), String> {
    println!("{}", code);
    
    if code != 200 && code != 302 {
	return Err("This is not fine.".into());
    }

    Ok(())
}
