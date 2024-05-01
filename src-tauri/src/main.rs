// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine};
use opencv::{
    core::Vector,
    imgcodecs::imencode,
    prelude::*,
    videoio::{self, VideoCapture},
    Result,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, Window};

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}

struct CameraOption {
    camera_on: bool,
}

impl Default for CameraOption {
    fn default() -> Self {
        Self { camera_on: false }
    }
}

// Camera
fn camera_feed(cam: &mut VideoCapture) -> Result<String> {
    let mut frame = Mat::default();
    cam.read(&mut frame)?;

    let params: Vector<i32> = Vector::new();
    let mut buf: Vector<u8> = Vector::new();
    imencode(".jpg", &frame, &mut buf, &params)?;

    let b64 = general_purpose::STANDARD.encode(&buf);

    Ok(format!("data:image/jpg;base64,{}", b64))
}

fn open_cam() -> Option<VideoCapture> {
    let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY);
    match cam {
        Ok(cam) => {
            let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
            if !opened {
                panic!("Camera is unavailable")
            }

            return Some(cam);
        }

        Err(_) => panic!("There is a problem opening OPENCV video capture"),
    }
}

//commands

#[tauri::command]
async fn stop(app: AppHandle) -> Result<bool, String> {
    let app_state = app.state::<Arc<Mutex<CameraOption>>>();
    let arc_state = Arc::clone(&app_state);
    let mut state = arc_state.lock().unwrap();
    println!("Stopping camera");
    state.camera_on = false;
    Ok(state.camera_on.clone())
}

#[tauri::command]
async fn start(app: AppHandle) -> Result<bool, String> {
    let app_state = app.state::<Arc<Mutex<CameraOption>>>();
    let arc_state = Arc::clone(&app_state);
    let mut state = arc_state.lock().unwrap();
    println!("Starting camera");
    state.camera_on = true;
    Ok(state.camera_on.clone())
}

#[tauri::command]
async fn image_sending(window: Window, app: AppHandle) -> Result<(), String> {
    let mut cam = open_cam().unwrap();
    let app_state = app.state::<Arc<Mutex<CameraOption>>>();
    let arc_state = Arc::clone(&app_state);
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(1));
        let state = arc_state.lock().unwrap();
        if state.camera_on == true {
            let feed = camera_feed(&mut cam);
            match feed {
                Ok(feed) => window
                    .emit("image-byte", Payload { message: feed })
                    .unwrap(),

                Err(_) => panic!("There is a problem in the camera feed"),
            }
        } else {
            cam.release().unwrap();
            break;
        }
    });

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(CameraOption::default())))
        .invoke_handler(tauri::generate_handler![image_sending, stop, start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
