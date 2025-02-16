// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    characteristic::{
        active::ActiveCharacteristic, recording_audio_active::RecordingAudioActiveCharacteristic,
        selected_camera_recording_configuration::SelectedCameraRecordingConfigurationCharacteristic,
        supported_audio_recording_configuration::SupportedAudioRecordingConfigurationCharacteristic,
        supported_camera_recording_configuration::SupportedCameraRecordingConfigurationCharacteristic,
        supported_video_recording_configuration::SupportedVideoRecordingConfigurationCharacteristic, HapCharacteristic,
    },
    service::HapService,
    HapType,
};

/// Camera Recording Management service.
#[derive(Debug, Default)]
pub struct CameraRecordingManagementService {
    /// Instance ID of the Camera Recording Management service.
    id: u64,
    /// [`HapType`](HapType) of the Camera Recording Management service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

    /// Active characteristic (required).
    pub active: ActiveCharacteristic,
    /// Supported Camera Recording Configuration characteristic (required).
    pub supported_camera_recording_configuration: SupportedCameraRecordingConfigurationCharacteristic,
    /// Supported Video Recording Configuration characteristic (required).
    pub supported_video_recording_configuration: SupportedVideoRecordingConfigurationCharacteristic,
    /// Supported Audio Recording Configuration characteristic (required).
    pub supported_audio_recording_configuration: SupportedAudioRecordingConfigurationCharacteristic,
    /// Selected Camera Recording Configuration characteristic (required).
    pub selected_camera_recording_configuration: SelectedCameraRecordingConfigurationCharacteristic,

    /// recording audio active characteristic (optional).
    pub recording_audio_active: Option<RecordingAudioActiveCharacteristic>,
}

impl CameraRecordingManagementService {
    /// Creates a new Camera Recording Management service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::CameraRecordingManagement,
            active: ActiveCharacteristic::new(id + 1 + 0, accessory_id),
            supported_camera_recording_configuration: SupportedCameraRecordingConfigurationCharacteristic::new(
                id + 1 + 1,
                accessory_id,
            ),
            supported_video_recording_configuration: SupportedVideoRecordingConfigurationCharacteristic::new(
                id + 1 + 2,
                accessory_id,
            ),
            supported_audio_recording_configuration: SupportedAudioRecordingConfigurationCharacteristic::new(
                id + 1 + 3,
                accessory_id,
            ),
            selected_camera_recording_configuration: SelectedCameraRecordingConfigurationCharacteristic::new(
                id + 1 + 4,
                accessory_id,
            ),
            recording_audio_active: Some(RecordingAudioActiveCharacteristic::new(id + 1 + 0 + 5, accessory_id)),
            ..Default::default()
        }
    }
}

impl HapService for CameraRecordingManagementService {
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
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
            &self.active,
            &self.supported_camera_recording_configuration,
            &self.supported_video_recording_configuration,
            &self.supported_audio_recording_configuration,
            &self.selected_camera_recording_configuration,
        ];
        if let Some(c) = &self.recording_audio_active {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
            &mut self.active,
            &mut self.supported_camera_recording_configuration,
            &mut self.supported_video_recording_configuration,
            &mut self.supported_audio_recording_configuration,
            &mut self.selected_camera_recording_configuration,
        ];
        if let Some(c) = &mut self.recording_audio_active {
            characteristics.push(c);
        }
        characteristics
    }
}

impl Serialize for CameraRecordingManagementService {
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
