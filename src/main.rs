use proc_experiments::{get_process_list, get_process_name};

#[cfg(target_os = "windows")]
fn main() {
    match get_process_list() {
        Ok(mut vec) => {
            vec.sort_unstable();
            vec.iter()
                .map(|pid| {
                    (
                        *pid,
                        get_process_name(*pid).unwrap_or_else(|err| format!("ERROR: {}", err)),
                    )
                })
                .for_each(|(pid, name)| println!("PID {}: {}", pid, name));
        }
        Err(e) => println!("{}", e),
    }

    dont_disappear::any_key_to_continue::default();
}
