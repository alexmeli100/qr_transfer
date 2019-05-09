use rand;
use rand::Rng;
use crate::opts::Opt;
use std::fs::metadata;
use ipconfig::Adapter;


pub fn get_address() -> Result<String, &'static str> {
    let adapters = ipconfig::get_adapters().unwrap();
    let candidate = adapters
        .iter()
        .find(|adapter| adapter.friendly_name() == "Wi-Fi");

    if let Some(adapter) = candidate {
        return Ok(find_ip(&adapter));
    }

    Err("Can't find wi-fi interface")
}

fn find_ip(adapter: &Adapter) -> String {
    let addr = adapter.ip_addresses()[1];

    addr.to_string()
}

pub fn random_url_path() -> String {
    rand::thread_rng().gen_range(1000, 9999).to_string()
}

pub fn should_be_zipped(opt: &Opt) -> Result<bool, std::io::Error> {
    if opt.zip || opt.files.len() > 1 {
        return Ok(true);
    }

    let md = metadata(&opt.files[0])?;

    Ok(md.is_dir())

}