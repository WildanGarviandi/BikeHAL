package com.garvield.bike.hardware.callbacks;

oneway interface BatteryLevelCallback {
    void onBatteryUpdate(in long batteryLevel);
    void onVoltageUpdate(in long volt);
}
