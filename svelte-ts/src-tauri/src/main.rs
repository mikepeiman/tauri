#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
extern crate single_instance;

use single_instance::SingleInstance;

fn main() {
  {
    let instance_a = SingleInstance::new("whatever").unwrap();
    assert!(instance_a.is_single());
    let instance_b = SingleInstance::new("whatever").unwrap();
    assert!(!instance_b.is_single());
}
let instance_c = SingleInstance::new("whatever").unwrap();
assert!(instance_c.is_single());
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
