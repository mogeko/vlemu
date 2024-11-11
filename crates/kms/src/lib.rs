use kms_sys::GetEmulatorVersion;
use std::ffi::CStr;

pub fn get_libkms_version() -> &'static str {
    let c_buf = unsafe { GetEmulatorVersion() };
    let c_str = unsafe { CStr::from_ptr(c_buf) };

    c_str.to_str().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_get_libkms_version() {
        get_libkms_version();
    }
}
