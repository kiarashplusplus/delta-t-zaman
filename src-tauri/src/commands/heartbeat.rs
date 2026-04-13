#[tauri::command]
pub fn heartbeat_ack(_sequence: u64) {
    // Acknowledge heartbeat
}
