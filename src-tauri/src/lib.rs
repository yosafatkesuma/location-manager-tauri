// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use objc2::rc::Retained;
use objc2_core_location::{CLAuthorizationStatus, CLLocationManager};
use serde::{Deserialize, Serialize};
use std::ptr;
use std::sync::Once;

static mut LOCATION_MANAGER: *mut Retained<CLLocationManager> = ptr::null_mut();
static INIT: Once = Once::new();

#[derive(Serialize, Deserialize)]
struct Coordinate {
    latitude: f64,
    longitude: f64,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn request_location_permission() {
    unsafe {
        println!("Requesting location permission");
        INIT.call_once(|| {
            LOCATION_MANAGER = Box::into_raw(Box::new(CLLocationManager::new()));
        });

        if !LOCATION_MANAGER.is_null() {
            let manager = &*LOCATION_MANAGER;
            let authorization_status = manager.authorizationStatus();
            log::debug!("authorization status: {:?}", authorization_status);
            println!("Authorization Status: {:?}", authorization_status);

            if authorization_status == CLAuthorizationStatus::NotDetermined {
                manager.requestWhenInUseAuthorization();
            }
        }
    }
}

// Cleanup function to prevent memory leaks
#[tauri::command]
fn cleanup_location_manager() {
    unsafe {
        if !LOCATION_MANAGER.is_null() {
            drop(Box::from_raw(LOCATION_MANAGER));
            LOCATION_MANAGER = ptr::null_mut();
        }
    }
}

// Cleanup function to prevent memory leaks
#[tauri::command]
fn check_location_permission() -> Result<Option<bool>, String> {
    unsafe {
        Ok(Some(!LOCATION_MANAGER.is_null()))
    }
}

// Cleanup function to prevent memory leaks
#[tauri::command]
fn location_coor() -> Result<Option<Coordinate>, String> {
    unsafe {
        if LOCATION_MANAGER.is_null() {
            Ok(None)
        } else {
            let manager = &*LOCATION_MANAGER;
            let coordination = manager.location();
            let unwrap_coordination = (unsafe { coordination.unwrap() }).coordinate();
            Ok(Some(Coordinate {
                latitude: unwrap_coordination.latitude,
                longitude: unwrap_coordination.longitude,
            }))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            request_location_permission,
            cleanup_location_manager,
            location_coor,
            check_location_permission,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
