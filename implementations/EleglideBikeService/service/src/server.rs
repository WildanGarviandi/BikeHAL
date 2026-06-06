use log::{debug, LevelFilter};

use eleglide_bike_service::EleglideBikeService;
use EleglideBikeInterface::aidl::com::garvield::bike::hardware::IBatteryControl::BnBatteryControl;
use EleglideBikeInterface::binder;

const SERVICE_IDENTIFIER: &str = "com.garvield.bike.hardware.IBatteryControl/default";

fn main() {
    logger::init(
        logger::Config::default()
            .with_tag_on_device("eleglidebikeservice")
            .with_max_level(LevelFilter::Trace),
    );
    debug!("This is a debug message.");

    let eleglide_bike_service = EleglideBikeService::new();
    let eleglide_bike_service_binder = BnBatteryControl::new_binder(
        eleglide_bike_service,
        binder::BinderFeatures::default(),
    );

    debug!("Preparing register service");
    match binder::add_service(SERVICE_IDENTIFIER, eleglide_bike_service_binder.as_binder()) {
        Ok(_) => log::info!("Service registered successfully"),
        Err(e) => {
            log::error!("Failed to register service: {:?}", e);
            std::process::exit(1);
        }
    }

    binder::ProcessState::start_thread_pool();
    binder::ProcessState::join_thread_pool();
}
