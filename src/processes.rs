use bindings::Windows::Win32::{
    ProcessStatus,
    SystemServices::{self, BOOL, PROCESS_ACCESS_RIGHTS, PSTR},
};
use std::mem;

#[derive(Debug)]
pub enum ProcessError {
    ProcessListError,
    ProcessInformationError,
}

/// Returns a list of process IDs currently known.
pub fn get_process_list() -> Result<Vec<u32>, ProcessError> {
    fill_vec(
        |arr: &mut [u32]| {
            let arr_ptr = arr.as_mut_ptr();
            let arr_size = mem::size_of_val(arr) as u32;
            let mut bytes_returned = 0u32;
            let success = unsafe {
                ProcessStatus::K32EnumProcesses(arr_ptr, arr_size, &mut bytes_returned).as_bool()
            };

            if success {
                Ok((bytes_returned as usize) / mem::size_of::<u32>())
            } else {
                Err(ProcessError::ProcessListError)
            }
        },
        0,
        // I saw 1024 getting used in some example from Microsoft, so I guess it's an acceptable
        // starting point
        1024,
    )
}

/// Resolves the process name by ID.
pub fn get_process_name(pid: u32) -> Result<String, ProcessError> {
    let res = fill_vec(
        |arr: &mut [u8]| {
            // PROCESS_QUERY_INFORMATION is needed for K32GetModuleFileNameExA
            let pac = PROCESS_ACCESS_RIGHTS(0x0400);
            let inherit = BOOL::from(false);
            let handle = unsafe { SystemServices::OpenProcess(pac, inherit, pid) };

            let arr_buffer = PSTR(arr.as_mut_ptr());
            let arr_size = mem::size_of_val(arr) as u32;
            let bytes_returned =
                unsafe { ProcessStatus::K32GetModuleFileNameExA(handle, 0, arr_buffer, arr_size) };

            if bytes_returned > 0 {
                // ending null byte isn't counted by WinAPI so add 1
                Ok((bytes_returned + 1) as usize)
            } else {
                Err(ProcessError::ProcessInformationError)
            }
        },
        0,
        // Max path length used to be 260, but apparently that's not always the case nowadays (?),
        // and 512 is a much nicer value anyway...
        512,
    )?;

    Ok(String::from_utf8_lossy(&res).to_string())
}

/// Calls a function filling a buffer array and increases the buffer size until all results fit
/// into it.
fn fill_vec<T: Copy, F: Fn(&mut [T]) -> Result<usize, ProcessError>>(
    f: F,
    default_value: T,
    initial_capacity: usize,
) -> Result<Vec<T>, ProcessError> {
    let mut capacity = initial_capacity;
    let mut vec;
    let mut size;

    loop {
        vec = vec![default_value; capacity];
        size = f(&mut vec)?;

        if size < capacity {
            break;
        }

        // Number of values returned is equal to capacity -> there might be more, increase capacity
        // and repeat
        capacity *= 2;
    }

    vec.drain(size..);
    // Shrinking probably isn't necessary, but I don't really feel comfortable passing around
    // nearly double as much memory as needed in the worst case
    vec.shrink_to_fit();

    Ok(vec)
}
