// this file is auto-generated by hap-codegen

use serde::{Deserialize, Serialize};

/// HAP accessory category.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessoryCategory {
    Other = 1,
    Bridge = 2,
    Fan = 3,
    GarageDoorOpener = 4,
    Lightbulb = 5,
    DoorLock = 6,
    Outlet = 7,
    Switch = 8,
    Thermostat = 9,
    Sensor = 10,
    SecuritySystem = 11,
    Door = 12,
    Window = 13,
    WindowCovering = 14,
    ProgrammableSwitch = 15,
    RangeExtender = 16,
    IpCamera = 17,
    VideoDoorbell = 18,
    AirPurifier = 19,
    AirHeater = 20,
    AirConditioner = 21,
    AirHumidifier = 22,
    AirDehumidifier = 23,
    AppleTv = 24,
    Speaker = 26,
    Airport = 27,
    Sprinkler = 28,
    Faucet = 29,
    ShowerHead = 30,
    Television = 31,
    TargetController = 32,
    WiFiRouter = 33,
    AudioReceiver = 34,
    TelevisionSetTopBox = 35,
    TelevisionStreamingStick = 36,
}
