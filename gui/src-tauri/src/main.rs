//! QOPS Desktop Application - Main Entry Point
//!
//! A Tauri-based desktop application for quantum algorithm research.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    qops_gui_lib::run()
}
