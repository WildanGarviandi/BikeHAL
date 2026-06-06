package com.garvield.bike.hardware;

import com.garvield.bike.hardware.callbacks.BatteryLevelCallback;

interface IBatteryControl {
    void getCurrentBatteryLevel(in BatteryLevelCallback callback);
    void listenToBatteryLevel(in BatteryLevelCallback callback);
}
