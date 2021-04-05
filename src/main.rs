use std::process::exit;

use proc_experiments::{get_process_list, get_process_name};

#[cfg(target_os = "windows")]
fn main() {
    let mut process_list = get_process_list().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    // Prints a list of all currently known processes
    process_list.sort_unstable();
    process_list
        .iter()
        .map(|pid| {
            (
                *pid,
                get_process_name(*pid).unwrap_or_else(|err| format!("ERROR: {}", err)),
            )
        })
        .for_each(|(pid, name)| println!("PID {}: {}", pid, name));

    dont_disappear::any_key_to_continue::custom_msg("Press any key to continue...");
}
