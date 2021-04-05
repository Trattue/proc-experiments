extern crate dont_disappear;

use dont_disappear::any_key_to_continue;
use proc_experiments::processes;

#[cfg(target_os = "windows")]
fn main() {
    let mut process_list = match processes::get_process_list() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("failed with {:?}", e);
            return;
        }
    };

    // Prints a list of all currently known processes
    process_list.sort_unstable();
    process_list
        .iter()
        .map(|&pid| {
            (
                pid,
                processes::get_process_name(pid)
                    .unwrap_or_else(|e| format!("failed with {:?} (permission denied?)", e)),
            )
        })
        .for_each(|(pid, name)| println!("PID {}: {}", pid, name));

    any_key_to_continue::custom_msg("Press any key to continue...");
}
