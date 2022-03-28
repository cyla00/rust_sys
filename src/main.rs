extern crate mac_address;
extern crate postgres;
extern crate reqwest;
extern crate serde_json;

use mac_address::get_mac_address;
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let mut ip: String = String::new();

    match reqwest::get("https://ifconfig.me").await {
        Ok(res) => {
            if res.status() == reqwest::StatusCode::OK {
                match res.text().await {
                    Ok(text) => {
                        ip = text;
                        let url: String =
                            "https://nordvpn.com/wp-admin/admin-ajax.php?action=get_user_info_data"
                                .to_string();

                        let client = reqwest::Client::new();

                        match client.get(url).send().await {
                            Ok(res) => {
                                if res.status() == reqwest::StatusCode::OK {
                                    match res.text().await {
                                        Ok(text) => {
                                            #[derive(Serialize, Deserialize, Debug)]
                                            pub struct Geolocation {
                                                coordinates: Coordinates,
                                                ip: String,
                                                isp: String,
                                                host: Host,
                                                status: bool,
                                                country: String,
                                                region: String,
                                                city: String,
                                                location: String,
                                                area_code: String,
                                                country_code: String,
                                            }

                                            #[derive(Serialize, Deserialize, Debug)]
                                            pub struct Coordinates {
                                                latitude: f64,
                                                longitude: f64,
                                            }

                                            #[derive(Serialize, Deserialize, Debug)]
                                            pub struct Host {
                                                domain: String,
                                                ip_address: String,
                                            }

                                            let geo: Geolocation =
                                                serde_json::from_str(&text).unwrap();

                                            if &ip == &geo.ip {
                                                let mut sys = System::new_all();

                                                sys.refresh_all();

                                                pub struct SystemInfo {
                                                    os: String,
                                                    os_version: String,
                                                    cores: usize,
                                                    tot_memory: u64,
                                                    host_name: String,
                                                }

                                                let system = SystemInfo {
                                                    os: sys.name().unwrap(),
                                                    os_version: sys.os_version().unwrap(),
                                                    cores: sys.processors().len(),
                                                    tot_memory: sys.total_memory(),
                                                    host_name: sys.host_name().unwrap(),
                                                };

                                                match get_mac_address() {
                                                    Ok(Some(address)) => {
                                                        let mac_address = &address;
                                                        //CONNECT TO DATABASE
                                                        // SEND DATA INTO DATABASE
                                                    }
                                                    Ok(None) => {
                                                        println!("No MAC address found.");
                                                    }
                                                    Err(e) => println!("{:?}", e),
                                                };
                                            }
                                        }
                                        Err(_) => println!("wasnt able to grab geolocal"),
                                    }
                                }
                            }
                            Err(_) => println!("error response text"),
                        }
                    }
                    Err(_) => println!("error response text"),
                }
            } else {
                println!("not 200 OK");
            }
        }
        Err(_) => println!("request didn't work"),
    }
}
