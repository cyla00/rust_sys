use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
extern crate reqwest;
extern crate serde_json;
use serde_json::Value;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main(){

    let mut ip: String = String::new();

    match reqwest::get("https://ifconfig.me").await {
        Ok(res) => {
            if res.status() == reqwest::StatusCode::OK {
                match res.text().await {
                    Ok(text) => {
                        ip = text;
                        let mut url: String = "https://nordvpn.com/wp-admin/admin-ajax.php?action=get_user_info_data".to_string();

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
                                            
                                            let data: Geolocation = serde_json::from_str(&text).unwrap();

                                            if &ip == &data.ip {
                                                println!("the ip {} is coherent", &data.ip); //possible verifications HERE
                                            }

                                        }
                                        Err(_) => println!("wasnt able to grab geolocal")
                                    }
                                }
                            }
                            Err(_) => println!("error response text")
                        }
                    }
                    Err(_) => println!("error response text")
                }
            }
            else {
                println!("not 200 OK");
            }
        }
        Err(_) => println!("request didn't work")
    }

    
    

    // let mut sys = System::new_all();

    // sys.refresh_all();

    // println!("=> disks:");
    // for disk in sys.disks() {
    //     println!("{:?}", disk);
    // }

    // println!("=> components:");
    // for component in sys.components() {
    //     println!("{:?}", component);
    // }

    // println!("total memory: {} KB", sys.total_memory());
    // println!("used memory : {} KB", sys.used_memory());
    // println!("total swap  : {} KB", sys.total_swap());
    // println!("used swap   : {} KB", sys.used_swap());

    // println!("System name:             {:?}", sys.name());
    // println!("System kernel version:   {:?}", sys.kernel_version());
    // println!("System OS version:       {:?}", sys.os_version());
    // println!("System host name:        {:?}", sys.host_name());
    // println!("System network0:        {:?}", sys.users());

    // println!("NB processors: {}", sys.processors().len());

    // for (pid, process) in sys.processes() {
    //     println!("[{}] {}", pid, process.name());
    // }
}

