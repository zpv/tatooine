use std::mem::size_of;
use winapi::um::winnt::HANDLE;
use winapi::um::memoryapi::WriteProcessMemory;
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::basetsd::DWORD64;
use winapi::shared::minwindef::{LPVOID};

pub fn write_mem<T>(process_handle: HANDLE, dw_addr: DWORD64, value: &mut T) -> Result<i32, i32> {
    let bytes_to_write = size_of::<T>();
    let result = unsafe {
        let result = WriteProcessMemory(
            process_handle,
            dw_addr as usize as *mut _,
            value as *mut _ as LPVOID,
            bytes_to_write,
            std::ptr::null_mut(),
        );
        result
    };
    match result {
        0 => unsafe { Err(GetLastError() as i32) },
        _ => Ok(bytes_to_write as i32)
    }
}
