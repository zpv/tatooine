use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::winnt::HANDLE;

use std::ffi::CString;

pub fn get_process(window_name: String) -> HANDLE {
    let window_name = CString::new(window_name).unwrap();

    unsafe {
        let window_handle = FindWindowA(std::ptr::null_mut(), window_name.as_ptr());

        let mut process_id: DWORD = 0;
        GetWindowThreadProcessId(window_handle, &mut process_id);

        let process_handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id);
        process_handle
    }
}