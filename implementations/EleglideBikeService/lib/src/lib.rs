use EleglideBikeInterface::aidl::com::garvield::bike::hardware::{
    IBatteryControl::{BnBatteryControl, IBatteryControl},
    BatteryLevelCallback::IBatteryLevelCallback,
};
use binder::{self, BinderFeatures, Interface, Strong};
use std::sync::{Arc, Mutex};

/// Battery service implementation
pub struct EleglideBikeService {
    battery_level: Arc<Mutex<i64>>,
    volt_level: Arc<Mutex<i64>>,
    callbacks: Arc<Mutex<Vec<Strong<dyn IBatteryLevelCallback>>>>,
}

impl EleglideBikeService {
    /// Create a new battery control service
    pub fn new() -> Self {
        Self {
            battery_level: Arc::new(Mutex::new(100)),
            volt_level: Arc::new(Mutex::new(20)),
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create a new binder instance
    pub fn new_binder() -> Strong<dyn IBatteryControl> {
        BnBatteryControl::new_binder(
            Self::new(),
            BinderFeatures::default(),
        )
    }

    /// Notify all registered callbacks of battery level change
    fn notify_callbacks(&self, level: i64, volt: i64) {
        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            if let Err(e) = callback.onBatteryUpdate(level) {
                eprintln!("Failed to notify callback: {:?}", e);
            }
            if let Err(e) = callback.onVoltageUpdate(volt) {
                eprintln!("Failed to notify callback: {:?}", e);
            }
        }
    }

    /// Simulate battery discharge (for testing)
    pub fn simulate_discharge(&self) {
        let battery_level = self.battery_level.clone();
        let callbacks = self.callbacks.clone();
        
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(5));
                
                let mut level = battery_level.lock().unwrap();
                if *level > 0 {
                    *level -= 1;
                    let current_level = *level;
                    drop(level);
                    
                    let cbs = callbacks.lock().unwrap();
                    for callback in cbs.iter() {
                        let _ = callback.onBatteryUpdate(current_level);
                    }
                }
            }
        });
    }
}

impl Default for EleglideBikeService {
    fn default() -> Self {
        Self::new()
    }
}

impl Interface for EleglideBikeService {}

impl IBatteryControl for EleglideBikeService {
    fn getCurrentBatteryLevel(
        &self,
        callback: &Strong<dyn IBatteryLevelCallback>,
    ) -> binder::Result<()> {
        let level = *self.battery_level.lock().unwrap();
        let volt = *self.volt_level.lock().unwrap();
        println!("Returning current battery level: {}%", level);
        callback.onBatteryUpdate(level)?;
        callback.onVoltageUpdate(volt)?;
        Ok(())
    }

    fn listenToBatteryLevel(
        &self,
        callback: &Strong<dyn IBatteryLevelCallback>,
    ) -> binder::Result<()> {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(callback.clone());
        
        println!("Callback registered. Total listeners: {}", callbacks.len());
        
        let level = *self.battery_level.lock().unwrap();
        let volt = *self.volt_level.lock().unwrap();
        drop(callbacks);

        self.notify_callbacks(level, volt);
        Ok(())
    }
}
