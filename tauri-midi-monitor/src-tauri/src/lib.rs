use midir::{MidiInput};

#[tauri::command]
fn fetch_midi_ports() ->  Vec<String> {
  let midi_input = MidiInput::new("My MIDI Input").unwrap();
  let in_ports = midi_input.ports();

  let port_names: Vec<String> = in_ports.iter()
  .map(|p| midi_input.port_name(p).unwrap())
  .collect();

  return port_names;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![fetch_midi_ports])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
