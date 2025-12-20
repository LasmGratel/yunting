use tokio::{fs::File, io::AsyncWriteExt};
use yunting_lib::{config::load_config, format_live_streams, get_radio_list, list_all_provinces};

#[tokio::main]
async fn main() {
    env_logger::init();

    let provinces = if let Ok(config) = load_config(&std::env::current_dir().unwrap()) {
        config.provinces
    } else {
        let provinces = list_all_provinces().await;
        if provinces.is_err() {
            panic!("Failed to fetch provinces");
        }
        let provinces = provinces.unwrap();
        if provinces.data.is_none() {
            panic!("No province data received: {:?}", provinces);
        }
        let province_codes = provinces
            .data
            .unwrap()
            .into_iter()
            .map(|p| p.province_code)
            .collect::<Vec<_>>();
        if let Err(e) = yunting_lib::config::save_config(
            &std::env::current_dir().unwrap(),
            province_codes.clone(),
        ) {
            panic!("Failed to save config: {:?}", e);
        }
        province_codes
    };

    let mut radio_list = vec![];

    for p in provinces {
        let radios = get_radio_list(p).await;
        if radios.is_err() {
            log::warn!(
                "Failed to fetch radio list for province {}, cause: {:?}",
                p,
                radios.err()
            );
            continue;
        }
        let radios = radios.unwrap();
        if radios.data.is_none() {
            log::warn!(
                "No radio list data found for province {}, cause: {:?}",
                p,
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
