use std::{cell::RefCell, ffi::{c_char, CStr, CString}, io::Write, str::FromStr, sync::{Arc, LazyLock}};

use log::{info, Level, LevelFilter, Log, Metadata, Record};
use tokio::{fs::File, io::AsyncWriteExt, runtime::{Builder, Runtime}, sync::{mpsc, RwLock}};
use yunting_lib::{list_all_provinces, get_radio_list, format_live_streams};

#[allow(non_camel_case_types)]
type scs_log_t = extern "C" fn(i32, *const c_char) -> ();

#[allow(non_upper_case_globals)]
const SCS_LOG_TYPE_message: i32 = 0;

#[allow(non_upper_case_globals)]
const SCS_LOG_TYPE_warning: i32 = 1;

#[allow(non_upper_case_globals)]
const SCS_LOG_TYPE_error  : i32 = 2;

#[repr(C)]
#[allow(non_camel_case_types)]
struct scs_sdk_init_params_v100_t {
    game_name: *const c_char,
    game_id: *const c_char,
    game_version: u32,

    #[cfg(target_pointer_width = "64")]
    _padding: u32,

    logger: scs_log_t,
}

struct ScsLogger {
    logger: scs_log_t
}

unsafe impl Send for ScsLogger {}
unsafe impl Sync for ScsLogger {}

impl Log for ScsLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // let level = match level {
            //     Level::Error => SCS_LOG_TYPE_error,
            //     Level::Warn => SCS_LOG_TYPE_warning,
            //     Level::Info => SCS_LOG_TYPE_message,
            //     Level::Trace | Level::Debug => {
            //         return;
            //     }
            // };
            let msg = format!("[{}] [{}] {}", record.target(), record.level(), record.args());
            (self.logger)(SCS_LOG_TYPE_message, CString::from_str(&msg).unwrap().as_ptr());
        }
    }

    fn flush(&self) {}
}

#[derive(Debug)]
#[allow(dead_code)]
struct ScsSdkInitParamsV100 {
    game_name: String,
    game_id: String,
    game_version: u32,
}

impl From<&scs_sdk_init_params_v100_t> for ScsSdkInitParamsV100 {
    fn from(value: &scs_sdk_init_params_v100_t) -> Self {
        unsafe {
            ScsSdkInitParamsV100 {
                game_name: CStr::from_ptr(value.game_name).to_string_lossy().to_string(),
                game_id: CStr::from_ptr(value.game_id).to_string_lossy().to_string(),
                game_version: value.game_version
            }
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct scs_telemetry_init_params_v100_t {
    common: scs_sdk_init_params_v100_t,
}

#[unsafe(no_mangle)]
#[allow(unused_variables)]
unsafe extern "system" fn scs_telemetry_init(version: i32, params: *const scs_telemetry_init_params_v100_t) -> i32 {

    let init_params = unsafe { &(*params).common };

    log::set_logger(Box::leak(Box::new(ScsLogger {
        logger: init_params.logger
    }))).map(|()| log::set_max_level(LevelFilter::Info)).unwrap();

    info!("Plugin Loaded!");

    let params = ScsSdkInitParamsV100::from(init_params);

    let handle = std::thread::spawn(move || {
        // create tokio runtime and notify main thread
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    
        info!("Initialized Tokio Runtime");
        rt.block_on(plugin_main(params));
    });
    0
}

async fn plugin_main(params: ScsSdkInitParamsV100) {
    let live_streams_path = if let Some(home_dir) = std::env::home_dir() {
        let documents = home_dir.join("Documents"); // TODO: use Windows API [Environment]::GetFolderPath('Personal')
        let game_name = if params.game_id == "eut2" { "Euro Truck Simulator 2" } else if params.game_id == "ats" { "American Truck Simulator" } else { &params.game_name };
        let game_dir = documents.join(game_name);
        if !game_dir.is_dir() {
            log::warn!("Game directory does not exist: {:?}", game_dir);
            return;
        }
        game_dir.join("live_streams.sii")
    } else {
        log::warn!("Could not determine home directory");
        return;
    };

    let provinces = list_all_provinces().await;
    if provinces.is_err() {
        log::error!("Failed to list provinces: {:?}", provinces.err());
        return;
    }
    let provinces = provinces.unwrap();
    if provinces.data.is_none() {
        log::error!("No province data received: {:?}", provinces);
        return;
    }

    let mut radio_list = vec![];

    for p in provinces.data.unwrap() {
        let province_radio = get_radio_list(p.province_code).await;
        if province_radio.is_err() {
            log::error!("Failed to get radio list for province {}: {:?}", p.province_code, province_radio.err());
            continue;
        }
        let province_radio = province_radio.unwrap();
        radio_list.push(province_radio.data);
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
        .open(&live_streams_path)
        .await
        .unwrap();
    file.write_all(format_live_streams(&radio_list).as_bytes())
        .await
        .unwrap();
    file.flush().await.unwrap();

    info!("Wrote live streams to {:?} with {} entries", live_streams_path, radio_list.len());

}

#[unsafe(no_mangle)]
unsafe extern "system" fn scs_telemetry_shutdown() {

}