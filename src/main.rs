use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
extern crate reqwest;
extern crate select;
extern crate scraper;

use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

#[tokio::main]
async fn main(){

    let mut ip: String = String::new();
    // let mut country: String = String::new();
    // let mut region: String = String::new();
    // let mut city: String = String::new();

    match reqwest::get("https://ifconfig.me").await {
        Ok(res) => {
            if res.status() == reqwest::StatusCode::OK {
                match res.text().await {
                    Ok(text) => {
                        ip = text;
                        let mut url: String = "https://www.iplocation.net/ip-lookup?query=".to_string();
                        let complete_url = format!("{}{}", url, ip);

                        let client = reqwest::Client::new();

                        match client.post(complete_url).send().await {
                            Ok(res) => {
                                if res.status() == reqwest::StatusCode::OK {
                                    match res.text().await {
                                        Ok(text) => {
                                            let document = Html::parse_document(&text);
                                            Document::from_read(&document)
                                                .unwrap()
                                                .find(Name("a"))
                                                .filter_map(|n| n.attr("href"))
                                                .for_each(|x| println!("{}", x));

                                            //push geoloc data into vars

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

