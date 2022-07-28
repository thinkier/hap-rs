// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    characteristic::{name::NameCharacteristic, power_state::PowerStateCharacteristic, HapCharacteristic},
    service::HapService,
    HapType,
};

/// Switch service.
#[derive(Debug, Default)]
pub struct SwitchService {
    /// Instance ID of the Switch service.
    id: u64,
    /// [`HapType`](HapType) of the Switch service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

    /// Power State characteristic (required).
    pub power_state: PowerStateCharacteristic,

    /// Name characteristic (optional).
    pub name: Option<NameCharacteristic>,
}

impl SwitchService {
    /// Creates a new Switch service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::Switch,
            power_state: PowerStateCharacteristic::new(id + 1 + 0, accessory_id),
            name: Some(NameCharacteristic::new(id + 1 + 0 + 1, accessory_id)),
            ..Default::default()
        }
    }
}

impl HapService for SwitchService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn set_type(&mut self, hap_type: HapType) {
        self.hap_type = hap_type;
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.power_state];
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![&mut self.power_state];
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

impl Serialize for SwitchService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
