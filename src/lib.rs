use std::mem;

use bindings::Windows::Win32::{ProcessStatus, SystemServices};

fn fill_vec<T: Copy, F: Fn(&mut [T]) -> Result<usize, String>>(
    f: F,
    default_value: T,
    initial_capacity: usize,
) -> Result<Vec<T>, String> {
    let mut capacity = initial_capacity;
    let mut vec: Vec<T>;
    let mut count;

    loop {
        vec = vec![default_value; capacity];
        count = f(&mut vec)?;

        if count < capacity {
            break;
        }

        capacity *= 2;
    }

    vec.drain(count..);
    vec.shrink_to_fit();
    Ok(vec)
}

pub fn get_process_list() -> Result<Vec<u32>, String> {
    fill_vec(
        |arr: &mut [u32]| {
            let arr_ptr = arr.as_mut_ptr();
            let arr_bytes = mem::size_of_val(arr) as u32;
            let mut bytes_returned = 0u32;
            let success;

            unsafe {
                success = ProcessStatus::K32EnumProcesses(arr_ptr, arr_bytes, &mut bytes_returned)
                    .as_bool();
            }

            if success {
                Ok((bytes_returned as usize) / mem::size_of::<u32>())
            } else {
                Err(String::from("failed fetching process list"))
            }
        },
        0,
        1024,
    )
}

pub fn get_process_name(pid: u32) -> Result<String, String> {
    let res = fill_vec(
        |arr: &mut [u8]| {
            let pac = SystemServices::PROCESS_ACCESS_RIGHTS(0x0400); // PROCESS_QUERY_INFORMATION
            let inherit = SystemServices::BOOL::from(false);

            let arr_buffer = SystemServices::PSTR(arr.as_mut_ptr());
            let arr_bytes = mem::size_of_val(arr) as u32;
            let bytes_returned;

            unsafe {
                let handle = SystemServices::OpenProcess(pac, inherit, pid);
                bytes_returned =
                    ProcessStatus::K32GetModuleFileNameExA(handle, 0, arr_buffer, arr_bytes);
            }

            if bytes_returned > 0 {
                Ok(bytes_returned as usize)
            } else {
                Err(String::from(
                    "failed fetching process information (missing permission?)",
                ))
            }
        },
        0,
        1024,
    )?;

    Ok(String::from_utf8_lossy(&res).into_owned())
}
