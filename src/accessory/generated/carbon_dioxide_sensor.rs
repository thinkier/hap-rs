// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    accessory::{AccessoryInformation, HapAccessory},
    service::{
        accessory_information::AccessoryInformationService, carbon_dioxide_sensor::CarbonDioxideSensorService,
        HapService,
    },
    HapType, Result,
};

/// Carbon dioxide Sensor accessory.
#[derive(Debug, Default)]
pub struct CarbonDioxideSensorAccessory {
    /// ID of the Carbon dioxide Sensor accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,
    /// Carbon dioxide Sensor service.
    pub carbon_dioxide_sensor: CarbonDioxideSensorService,
}

impl CarbonDioxideSensorAccessory {
    /// Creates a new Carbon dioxide Sensor accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let carbon_dioxide_sensor_id = accessory_information.get_characteristics().len() as u64;
        let mut carbon_dioxide_sensor = CarbonDioxideSensorService::new(1 + carbon_dioxide_sensor_id + 1, id);
        carbon_dioxide_sensor.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            carbon_dioxide_sensor,
        })
    }
}

impl HapAccessory for CarbonDioxideSensorAccessory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![&self.accessory_information, &self.carbon_dioxide_sensor]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![&mut self.accessory_information, &mut self.carbon_dioxide_sensor]
    }
}

impl Serialize for CarbonDioxideSensorAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
