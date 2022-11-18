use std::ptr;

use windows::Win32::{Media::Audio::*, System::Com::*};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
pub enum ActionType {
    Open { path: String },
    Volume { action: VolumeAction },
    Brightness,
}

impl ActionType {
    pub async fn execute(&self) {
        match &self {
            ActionType::Open { path } => {
                opener::open(std::path::Path::new(path));
            }
            ActionType::Volume { action } => {
                match action {
                    VolumeAction::Mute { data } => {
                        unsafe {
                            CoInitialize(std::ptr::null().);

                            let enumerator: IMMDeviceEnumerator =
                                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).unwrap();
                            let device = enumerator
                                .GetDefaultAudioEndpoint(eRender, eMultimedia)
                                .unwrap();
                            let manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, ptr::null()).unwrap();
                            let sessions = manager.GetSessionEnumerator().unwrap();

                            for n in 0..sessions.GetCount().unwrap() {
                                let session_control = sessions.GetSession(n).unwrap();
                            }

                            CoUninitialize();
                            let session_control_2: windows::Win32::Media::Audio::IAudioSessionControl2 = session_control.cast().unwrap();
                        }
                    }
                    VolumeAction::SetVolume { amount } => {}
                }
            }
            ActionType::Brightness => {}
        }
    }
}


pub enum VolumeAction {
    Mute { data: bool },
    SetVolume { amount: u32 },
}
//
// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
// pub struct OpenData{
//     //pub application: String,
//     pub path: std::path::Path,
// }
