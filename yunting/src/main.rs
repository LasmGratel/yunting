use std::fmt::Result;

use tokio::{fs::File, io::AsyncWriteExt};
use yunting_lib::{list_all_provinces, get_radio_list, format_live_streams};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let provinces = list_all_provinces().await;
    if provinces.is_err() {
        panic!("Failed to fetch provinces");
    }
    let provinces = provinces.unwrap();
    if provinces.data.is_none() {
        panic!("No provinces data found, cause: {:?}", provinces);
    }

    let mut radio_list = vec![];

    for p in provinces.data.unwrap() {
        let radios = get_radio_list(p.province_code).await;
        if radios.is_err() {
            log::warn!(
                "Failed to fetch radio list for province {}: {:?}",
                p.province_code,
                radios.err()
            );
            continue;
        }
        let radios = radios.unwrap();
        if radios.data.is_none() {
            log::warn!(
                "No radio list data found for province {}, cause: {:?}",
                p.province_code,
                radios
            );
            continue;
        }
        radio_list.push(radios.data);
    }


    let radio_list = radio_list
        .into_iter()
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open("live_streams.sii")
        .await
        .unwrap();
    file.write_all(format_live_streams(&radio_list).as_bytes())
        .await
        .unwrap();
    file.flush().await.unwrap();
}
