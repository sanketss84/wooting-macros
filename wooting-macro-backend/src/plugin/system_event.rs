#[cfg(any(target_os = "windows", target_os = "linux"))]
use brightness::{windows::BrightnessExt, Brightness, BrightnessDevice};

use copypasta::{ClipboardContext, ClipboardProvider};
#[cfg(any(target_os = "windows", target_os = "linux"))]
use futures::{executor::block_on, TryStreamExt};
use std::time;
use wifi_rs;

use crate::hid_table::SCANCODE_TO_RDEV;
use rdev::{Event, EventType};
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
pub enum SystemAction {
    Open { path: String },
    Volume { action: VolumeAction },
    Brightness { action: MonitorBrightnessAction },
    Clipboard { action: ClipboardAction },
    // Wifi needs more communication between frontend and backend,
    // though same could be said for the brightness devices
    Wifi { action: WifiAction },
}

impl SystemAction {
    pub async fn execute(&self, send_channel: Sender<EventType>) {
        match &self {
            SystemAction::Open { path } => {
                opener::open(std::path::Path::new(path));
            }
            SystemAction::Volume { action } => match action {
                VolumeAction::ToggleMute => {
                    send_channel.send(
                            rdev::EventType::KeyPress(rdev::Key::Unknown(173)))
                        .await
                        .unwrap();

                    send_channel.send(
                        rdev::EventType::KeyRelease(rdev::Key::Unknown(173)))
                        .await
                        .unwrap();
                }
                VolumeAction::LowerVolume => {
                    send_channel.send(
                        rdev::EventType::KeyPress(rdev::Key::Unknown(174)))
                        .await
                        .unwrap();

                    send_channel.send(
                        rdev::EventType::KeyRelease(rdev::Key::Unknown(174)))
                        .await
                        .unwrap();
                }
                VolumeAction::IncreaseVolume => {
                    send_channel.send(
                        rdev::EventType::KeyPress(rdev::Key::Unknown(175)))
                        .await
                        .unwrap();

                    send_channel.send(
                        rdev::EventType::KeyRelease(rdev::Key::Unknown(175)))
                        .await
                        .unwrap();
                }
            },
            SystemAction::Brightness { action } => match action {
                MonitorBrightnessAction::Get => {
                    // println!("{:#?}", show_brightness().await.unwrap());
                    #[cfg(any(target_os = "windows", target_os = "linux"))]
                    brightness_get_info().await;
                    #[cfg(target_os = "macos")]
                    println!("Not supported on macOS");
                }
                MonitorBrightnessAction::Set { level } => {
                    #[cfg(any(target_os = "windows", target_os = "linux"))]
                    brightness_set_info(*level).await;
                    #[cfg(target_os = "macos")]
                    println!("Not supported on macOS");
                }
            },
            SystemAction::Clipboard { action } => match action {
                ClipboardAction::SetClipboard { data } => {
                    let mut ctx = ClipboardContext::new().unwrap();

                    ctx.set_contents(data.to_owned()).unwrap();
                }
                ClipboardAction::Copy => {
                    send_channel
                        .send(rdev::EventType::KeyPress(rdev::Key::ControlLeft)
                        )
                        .await
                        .unwrap();
                    send_channel
                        .send(rdev::EventType::KeyPress(rdev::Key::KeyC)
                        )
                        .await
                        .unwrap();
                    send_channel
                        .send(rdev::EventType::KeyRelease(rdev::Key::KeyC)
                        )
                        .await
                        .unwrap();
                    send_channel
                        .send(rdev::EventType::KeyRelease(rdev::Key::ControlLeft)
                        )
                        .await
                        .unwrap();
                }
                ClipboardAction::GetClipboard => {
                    let mut ctx = ClipboardContext::new().unwrap();

                    let content = ctx.get_contents().unwrap();

                    println!("{}", content);
                }
                ClipboardAction::Paste => {
                    send_channel
                        .send(rdev::EventType::KeyPress(rdev::Key::ControlLeft)
                        )
                        .await
                        .unwrap();
                    send_channel
                        .send(rdev::EventType::KeyPress(rdev::Key::KeyV)
                        )
                        .await
                        .unwrap();
                    send_channel
                        .send(rdev::EventType::KeyRelease(rdev::Key::KeyV)
                        )
                        .await
                        .unwrap();
                    send_channel
                        .send(rdev::EventType::KeyRelease(rdev::Key::ControlLeft)
                        )
                        .await
                        .unwrap();
                }
            },
            SystemAction::Wifi { .. } => {}
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
async fn brightness_get_info() {
    let count = brightness::brightness_devices()
        .try_fold(0, |count, dev| async move {
            show_brightness(&dev).await?;
            Ok(count + 1)
        })
        .await
        .unwrap();
    println!("Found {} displays", count);
}

//TODO: accept device from frontend
#[cfg(any(target_os = "windows", target_os = "linux"))]
async fn brightness_set_info(percentage_level: u32) {
    let count = brightness::brightness_devices()
        .try_fold(0, |count, mut dev| async move {
            set_brightness(&mut dev, percentage_level).await?;
            Ok(count + 1)
        })
        .await
        .unwrap();
    println!("Found {} displays", count);
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
async fn show_brightness(dev: &BrightnessDevice) -> Result<(), brightness::Error> {
    println!("Display {}", dev.device_name().await?);
    println!("\tBrightness = {}%", dev.get().await?);
    show_platform_specific_info(dev).await?;
    Ok(())
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
async fn set_brightness(
    dev: &mut BrightnessDevice,
    percentage_level: u32,
) -> Result<(), brightness::Error> {
    println!("Display {}", dev.device_name().await?);
    dev.set(percentage_level).await?;
    show_platform_specific_info(dev).await?;
    Ok(())
}

#[cfg(windows)]
async fn show_platform_specific_info(dev: &BrightnessDevice) -> Result<(), brightness::Error> {
    println!("\tDevice description = {}", dev.device_description()?);
    println!("\tDevice registry key = {}", dev.device_registry_key()?);
    Ok(())
}

#[cfg(target_os = "linux")]
async fn show_platform_specific_info(_: &BrightnessDevice) -> Result<(), brightness::Error> {
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
pub enum ClipboardAction {
    SetClipboard { data: String },
    Copy,
    GetClipboard,
    Paste,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
pub enum MonitorBrightnessAction {
    Get,
    Set { level: u32 },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
pub enum VolumeAction {
    LowerVolume,
    IncreaseVolume,
    ToggleMute,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Hash, Eq)]
pub enum WifiAction {
    Connect,
    Disconnect,
    Hotspot,
}
