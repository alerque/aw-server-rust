#![feature(plugin)]
#![feature(proc_macro_hygiene)]
#![feature(custom_attribute)]
#![feature(decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate rocket_cors;
extern crate multipart;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate rusqlite;

extern crate crossbeam_requests;

extern crate chrono;

extern crate plex;

extern crate appdirs;

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate log;
extern crate fern;

extern crate toml;

pub mod models;
pub mod transform;
pub mod datastore;
pub mod query;
pub mod endpoints;
pub mod dirs;
pub mod logging;
pub mod config;

fn main() {
    use std::sync::Mutex;

    logging::setup_logger().expect("Failed to setup logging");

    let config = config::get_config();

    let db_path = dirs::db_path().to_str().unwrap().to_string();
    info!("Using DB at path {:?}", db_path);

    let asset_path = get_asset_path();
    info!("Using aw-webui assets at path {:?}", asset_path);

    let server_state = endpoints::ServerState {
        datastore: Mutex::new(datastore::Datastore::new(db_path)),
        asset_path: asset_path,
    };

    endpoints::build_rocket(server_state, &config).launch();
}

use std::path::PathBuf;

fn get_asset_path () -> PathBuf {
    use std::env::current_exe;

    // TODO: Add cmdline arg which can override asset path?

    // Search order for asset path is:
    // 1. ./aw-webui/dist
    // 2. $current_exe_dir/aw_server_rust/static
    // 3. ./aw_server_rust/static
    // 4. (fallback) ./aw-webui/dist

    // cargo_dev_path
    // (for running with cargo run)
    let cargo_dev_path = PathBuf::from("./aw-webui/dist/");
    if cargo_dev_path.as_path().exists() {
        return cargo_dev_path;
    }

    // current_exe_path
    // (for self-contained deployed binaries)
    match current_exe() {
        Ok(mut current_exe_path) => {
            current_exe_path.pop(); // remove name of executable
            current_exe_path.push("aw_server_rust/static/");
            if current_exe_path.as_path().exists() {
                return current_exe_path;
            }
        },
        Err(_) => (),
    };

    // cwd_path
    // (for linux opt installs)
    let cwd_path = PathBuf::from("./aw_server_rust/static/");
    if cwd_path.as_path().exists() {
        return cwd_path;
    }

    // usr_path
    // (for linux usr installs)
    let mut usr_path = appdirs::site_data_dir(Some("activitywatch"), None).unwrap();
    usr_path.push("aw_server_rust/static");
    if usr_path.as_path().exists() {
        return cwd_path;
    }

    warn!("Unable to find an aw-webui asset path which exists, falling back to ./aw-webui/dist");
    return cargo_dev_path;
}
