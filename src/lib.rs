use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Gps;
#[cfg(mobile)]
use mobile::Gps;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the gps APIs.
pub trait GpsExt<R: Runtime> {
  fn gps(&self) -> &Gps<R>;
}

impl<R: Runtime, T: Manager<R>> crate::GpsExt<R> for T {
  fn gps(&self) -> &Gps<R> {
    self.state::<Gps<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("gps")
    .invoke_handler(tauri::generate_handler![commands::execute])
    .setup(|app, api| {
      #[cfg(mobile)]
      let gps = mobile::init(app, api)?;
      #[cfg(desktop)]
      let gps = desktop::init(app, api)?;
      app.manage(gps);

      // manage state so it is accessible by the commands
      app.manage(MyState::default());
      Ok(())
    })
    .build()
}
