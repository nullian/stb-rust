use stb_sys as sys;
use std::fmt::Display;
use std::slice;

#[derive(Debug, Clone)]
pub enum ErrCode {
    FileNotExist,
    InvalidUtf8,
    LoadFailed,
    WriteFailed,
    ResizeFailed,
}

#[derive(Debug, Clone)]
pub struct STBErr {
    pub code: ErrCode,
    pub msg: String,
}

impl STBErr {
    pub fn new(code: ErrCode, msg: String) -> Self {
        STBErr { code, msg }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageInfo {
    pub width: usize,
    pub height: usize,
    pub components: usize,
}

impl ImageInfo {
    pub fn new(width: usize, height: usize, components: usize) -> Self {
        ImageInfo {
            width,
            height,
            components,
        }
    }
}

#[derive(Debug)]
pub struct Image<T: Clone + Display> {
    pub info: ImageInfo,
    pub buffer: Vec<T>,
}

impl<T: Clone + Display> Image<T> {
    pub fn new(data: *mut T, w: usize, h: usize, c: usize) -> Self {
        let info = ImageInfo::new(w, h, c);
        let size = w * h * c;
        unsafe {
            let buffer = slice::from_raw_parts_mut(data, size).to_vec();
            sys::stbi_image_free(data as *mut core::ffi::c_void);

            Image { info, buffer }
        }
    }

    pub fn build(data: Vec<T>, w: usize, h: usize, c: usize) -> Self {
        assert!(data.len() == w * h * c);
        let info = ImageInfo::new(w, h, c);

        Image { info, buffer: data }
    }
}
