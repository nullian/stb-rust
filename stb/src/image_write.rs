use core::ffi::CStr;
use stb_sys as sys;
use std::path::Path;

use crate::common::*;

pub fn write_png<T: AsRef<Path>>(path: &T, image: &Image<u8>) -> Result<(), STBErr> {
    let path_str: &str;
    match path.as_ref().to_str() {
        None => {
            return Err(STBErr {
                code: ErrCode::InvalidUtf8,
                msg: "invalid utf8".to_string(),
            });
        }

        Some(s) => path_str = s,
    };

    let path_bytes = path_str.as_bytes();

    match unsafe {
        let path_cstr = CStr::from_bytes_with_nul_unchecked(path_bytes);
        sys::stbi_write_png(
            path_cstr.as_ptr(),
            image.info.width as i32,
            image.info.height as i32,
            image.info.components as i32,
            image.buffer.as_ptr() as *const core::ffi::c_void,
            (image.info.width * image.info.components) as i32,
        )
    } {
        1 => Ok(()),

        _ => Err(STBErr {
            code: ErrCode::WriteFailed,
            msg: "write png failed".to_string(),
        }),
    }
}

pub fn write_jpg<T: AsRef<Path>>(path: &T, image: &Image<u8>, quality: i32) -> Result<(), STBErr> {
    let path_str: &str;
    match path.as_ref().to_str() {
        None => {
            return Err(STBErr {
                code: ErrCode::InvalidUtf8,
                msg: "invalid utf8".to_string(),
            });
        }

        Some(s) => path_str = s,
    };

    let path_bytes = path_str.as_bytes();

    match unsafe {
        let path_cstr = CStr::from_bytes_with_nul_unchecked(path_bytes);
        sys::stbi_write_jpg(
            path_cstr.as_ptr(),
            image.info.width as i32,
            image.info.height as i32,
            image.info.components as i32,
            image.buffer.as_ptr() as *const core::ffi::c_void,
            quality,
        )
    } {
        1 => Ok(()),

        _ => Err(STBErr {
            code: ErrCode::WriteFailed,
            msg: "write jpg failed".to_string(),
        }),
    }
}

pub fn write_bmp<T: AsRef<Path>>(path: T, image: &Image<u8>) -> Result<(), STBErr> {
    let path_str: &str;
    match path.as_ref().to_str() {
        None => {
            return Err(STBErr {
                code: ErrCode::InvalidUtf8,
                msg: "invalid utf8".to_string(),
            });
        }

        Some(s) => path_str = s,
    };

    let path_bytes = path_str.as_bytes();

    match unsafe {
        let path_cstr = CStr::from_bytes_with_nul_unchecked(path_bytes);
        sys::stbi_write_bmp(
            path_cstr.as_ptr(),
            image.info.width as i32,
            image.info.height as i32,
            image.info.components as i32,
            image.buffer.as_ptr() as *const core::ffi::c_void,
        )
    } {
        1 => Ok(()),

        _ => Err(STBErr {
            code: ErrCode::WriteFailed,
            msg: "write bmp failed".to_string(),
        }),
    }
}

pub fn write_tga<T: AsRef<Path>>(path: T, image: &Image<u8>) -> Result<(), STBErr> {
    let path_str: &str;
    match path.as_ref().to_str() {
        None => {
            return Err(STBErr {
                code: ErrCode::InvalidUtf8,
                msg: "invalid utf8".to_string(),
            });
        }

        Some(s) => path_str = s,
    };

    let path_bytes = path_str.as_bytes();

    match unsafe {
        let path_cstr = CStr::from_bytes_with_nul_unchecked(path_bytes);
        sys::stbi_write_tga(
            path_cstr.as_ptr(),
            image.info.width as i32,
            image.info.height as i32,
            image.info.components as i32,
            image.buffer.as_ptr() as *const core::ffi::c_void,
        )
    } {
        1 => Ok(()),

        _ => Err(STBErr {
            code: ErrCode::WriteFailed,
            msg: "write tga failed".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn path_to_cstr() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut path = PathBuf::from(root.parent().unwrap());

        path.push("images");
        path.push("white.png");

        let path_bytes = path.as_path().to_str().unwrap().as_bytes();
        println!("path: {}", path.as_path().to_str().unwrap());

        let _path_cstr = unsafe { CStr::from_bytes_with_nul_unchecked(&path_bytes[..]) };
    }

    #[test]
    fn test_write() {
        let dir = std::env::temp_dir();

        let image = Image::build(vec![1_u8, 1_u8, 1_u8], 1, 1, 3);

        // png
        let png_path = dir.join("test_png.png");
        assert_eq!(write_png(&png_path, &image).unwrap(), ());
        assert!(png_path.exists());
        fs::remove_file(png_path).expect("failed to remove png file");

        // jpg
        let jpg_path = dir.join("test_jpg.jpg");
        assert_eq!(write_jpg(&jpg_path, &image, 100).unwrap(), ());
        assert!(jpg_path.exists());
        fs::remove_file(jpg_path).expect("failed to remove png file");

        // bmp
        let bmp_path = dir.join("test_bmp.bmp");
        assert_eq!(write_bmp(&bmp_path, &image).unwrap(), ());
        assert!(bmp_path.exists());
        fs::remove_file(bmp_path).expect("failed to remove png file");

        // tga
        let tga_path = dir.join("test_tga.tga");
        assert_eq!(write_tga(&tga_path, &image).unwrap(), ());
        assert!(tga_path.exists());
        fs::remove_file(tga_path).expect("failed to remove png file");
    }
}
