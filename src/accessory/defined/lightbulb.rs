// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    accessory::{AccessoryInformation, HapAccessory},
    service::{accessory_information::AccessoryInformationService, lightbulb::LightbulbService, HapService},
    HapType, Result,
};

/// Lightbulb Accessory.
#[derive(Debug, Default)]
pub struct LightbulbAccessory {
    /// ID of the Lightbulb Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// Lightbulb Service.
    pub lightbulb: LightbulbService,
}

impl LightbulbAccessory {
    /// Creates a new Lightbulb Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let lightbulb_id = accessory_information.get_characteristics().len() as u64;
        let mut lightbulb = LightbulbService::new(1 + lightbulb_id + 1, id);
        lightbulb.set_primary(true);

        // TODO - this has to do with adaptive lighting and the controller refuses to pair until we figured it out
        lightbulb.characteristic_value_active_transition_count = None;
        lightbulb.characteristic_value_transition_control = None;
        lightbulb.supported_characteristic_value_transition_configuration = None;

        Ok(Self {
            id,
            accessory_information,
            lightbulb,
        })
    }
}

impl HapAccessory for LightbulbAccessory {
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
        vec![&self.accessory_information, &self.lightbulb]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![&mut self.accessory_information, &mut self.lightbulb]
    }
}

impl Serialize for LightbulbAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
