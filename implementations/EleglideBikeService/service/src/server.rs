use eleglide_bike_service::EleglideBikeService;
use EleglideBikeInterface::aidl::com::garvield::bike::hardware::IBatteryControl::BnBatteryControl;
use EleglideBikeInterface::binder;

const SERVICE_IDENTIFIER: &str = "eleglidebikeservice";

fn main() {
    let eleglide_bike_service = EleglideBikeService::new();
    let eleglide_bike_service_binder = BnBatteryControl::new_binder(
        eleglide_bike_service,
        binder::BinderFeatures::default(),
    );

    binder::add_service(SERVICE_IDENTIFIER, eleglide_bike_service_binder.as_binder())
        .expect("Failed to register service");
    binder::ProcessState::join_thread_pool();
}
