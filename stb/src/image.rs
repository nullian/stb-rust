#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use stb_sys as sys;

use std::ffi::{CStr, CString};
use std::path::Path;

use crate::common::*;

pub fn check_path<T: AsRef<Path>>(path: &T) -> Result<CString, STBErr> {
    match path.as_ref().as_os_str().to_str() {
        Some(s) => match CString::new(s.as_bytes()) {
            Ok(cstring) => Ok(cstring),

            Err(_) => Err(STBErr {
                code: ErrCode::FileNotExist,
                msg: "path contains null character".to_string(),
            }),
        },
        _ => Err(STBErr {
            code: ErrCode::InvalidUtf8,
            msg: "path is not valid utf8".to_string(),
        }),
    }
}

pub fn get_fail_reason() -> Result<String, STBErr> {
    unsafe {
        match CStr::from_ptr(sys::stbi_failure_reason()).to_str() {
            Ok(reason) => Ok(reason.to_string()),
            _ => Err(STBErr {
                code: ErrCode::LoadFailed,
                msg: "invalid message".to_string(),
            }),
        }
    }
}

pub fn load_u8<T: AsRef<Path>>(path: &T, desire_channels: usize) -> Result<Image<u8>, STBErr> {
    let mut w = 0 as core::ffi::c_int;
    let mut h = 0 as core::ffi::c_int;
    let mut component = 0 as core::ffi::c_int;
    let dst_channel = desire_channels as core::ffi::c_int;

    let path_cstring = check_path(path)?;

    unsafe {
        let bytes = path_cstring.as_ptr();
        let buffer = sys::stbi_load(bytes, &mut w, &mut h, &mut component, dst_channel);

        if buffer.is_null() {
            println!("image buffer is null");
            let reason = get_fail_reason()?;
            Err(STBErr {
                code: ErrCode::LoadFailed,
                msg: reason,
            })
        } else {
            Ok(Image::new(
                buffer,
                w as usize,
                h as usize,
                dst_channel as usize,
            ))
        }
    }
}

pub fn load_u16<T: AsRef<Path>>(path: &T, desire_channels: usize) -> Result<Image<u16>, STBErr> {
    let mut w = 0 as core::ffi::c_int;
    let mut h = 0 as core::ffi::c_int;
    let mut component = 0 as core::ffi::c_int;
    let dst_channel = desire_channels as core::ffi::c_int;

    let path_cstring = check_path(path)?;
    unsafe {
        let bytes = path_cstring.as_ptr();
        let buffer = sys::stbi_load_16(bytes, &mut w, &mut h, &mut component, dst_channel);

        if buffer.is_null() {
            let reason = get_fail_reason()?;
            Err(STBErr {
                code: ErrCode::LoadFailed,
                msg: reason,
            })
        } else {
            Ok(Image::new(
                buffer,
                w as usize,
                h as usize,
                dst_channel as usize,
            ))
        }
    }
}

pub fn load_f32<T: AsRef<Path>>(path: &T, desire_channels: usize) -> Result<Image<f32>, STBErr> {
    let mut w = 0 as core::ffi::c_int;
    let mut h = 0 as core::ffi::c_int;
    let mut component = 0 as core::ffi::c_int;
    let dst_channel = desire_channels as core::ffi::c_int;

    let path_cstring = check_path(path)?;
    unsafe {
        let bytes = path_cstring.as_ptr();
        let buffer = sys::stbi_loadf(bytes, &mut w, &mut h, &mut component, dst_channel);
        if buffer.is_null() {
            let reason = get_fail_reason()?;
            Err(STBErr {
                code: ErrCode::LoadFailed,
                msg: reason,
            })
        } else {
            Ok(Image::new(
                buffer,
                w as usize,
                h as usize,
                dst_channel as usize,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn image_path(file: &str) -> PathBuf {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut path = PathBuf::from(root.parent().unwrap());

        path.push("images");
        path.push(file);

        path
    }

    #[test]
    fn load_u8_from_file() {
        for c in 1..=4 {
            if c == 2 {
                continue;
            }

            let image_u8 = load_u8(&image_path("white.png"), c).expect("Failed to load image(u8)");

            assert_eq!(image_u8.info.width, 20);
            assert_eq!(image_u8.info.height, 30);
            assert_eq!(image_u8.info.components, c as usize);
            assert_eq!(image_u8.buffer.len(), (20 * 30 * c) as usize);

            let data = image_u8.buffer;
            for d in data.iter() {
                assert_eq!(*d, u8::MAX);
            }
        }

        assert!(load_u8(&image_path("not_exist.png").clone(), 3).is_err());
    }

    #[test]
    fn load_f16_from_file() {
        for c in 1..=4 {
            if c == 2 {
                continue;
            }

            let image_u16 =
                load_u16(&image_path("white.png"), c).expect("Failed to load image(f16)");

            assert_eq!(image_u16.info.width, 20);
            assert_eq!(image_u16.info.height, 30);
            assert_eq!(image_u16.info.components, c as usize);
            assert_eq!(image_u16.buffer.len(), (20 * 30 * c) as usize);

            let data = image_u16.buffer;
            for d in data.iter() {
                assert_eq!(*d, u16::MAX);
            }
        }
    }

    #[test]
    fn load_f32_from_file() {
        for c in 1..=4 {
            if c == 2 {
                continue;
            }

            let image_f32 =
                load_f32(&image_path("white.png"), c).expect("Failed to load image(f32)");

            assert_eq!(image_f32.info.width, 20);
            assert_eq!(image_f32.info.height, 30);
            assert_eq!(image_f32.info.components, c as usize);
            assert_eq!(image_f32.buffer.len(), (20 * 30 * c) as usize);

            let data = image_f32.buffer;
            for d in data.iter() {
                assert!((*d - 1.0_f32).abs() < 0.0000001_f32, "element {}", *d);
            }
        }
    }
}
