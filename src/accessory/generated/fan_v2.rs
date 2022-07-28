// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    accessory::{AccessoryInformation, HapAccessory},
    service::{accessory_information::AccessoryInformationService, fan_v2::FanV2Service, HapService},
    HapType, Result,
};

/// Fan v2 accessory.
#[derive(Debug, Default)]
pub struct FanV2Accessory {
    /// ID of the Fan v2 accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,
    /// Fan v2 service.
    pub fan_v2: FanV2Service,
}

impl FanV2Accessory {
    /// Creates a new Fan v2 accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let fan_v2_id = accessory_information.get_characteristics().len() as u64;
        let mut fan_v2 = FanV2Service::new(1 + fan_v2_id + 1, id);
        fan_v2.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            fan_v2,
        })
    }
}

impl HapAccessory for FanV2Accessory {
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
        vec![&self.accessory_information, &self.fan_v2]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![&mut self.accessory_information, &mut self.fan_v2]
    }
}

impl Serialize for FanV2Accessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
