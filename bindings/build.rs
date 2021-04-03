fn main() {
    windows::build!(
        Windows::Win32::ProcessStatus::K32EnumProcesses,
        Windows::Win32::ProcessStatus::K32GetModuleFileNameExA,
        Windows::Win32::SystemServices::PROCESS_ACCESS_RIGHTS,
        Windows::Win32::SystemServices::OpenProcess,
    );
}
