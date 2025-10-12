package com.garvield.bike.hardware;

oneway interface BatteryLevelCallback {
    void onBatteryUpdate(in long batteryLevel);
    void onVoltageUpdate(in long volt);
}
