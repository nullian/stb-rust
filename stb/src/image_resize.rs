use crate::common::*;
use stb_sys as sys;

pub fn resize_u8(src: &Image<u8>, dst_w: usize, dst_h: usize) -> Result<Image<u8>, STBErr> {
    let size = dst_w * dst_h * src.info.components;

    let mut buffer = vec![0_u8; size];

    match unsafe {
        sys::stbir_resize_uint8(
            src.buffer.as_ptr(),
            src.info.width as i32,
            src.info.height as i32,
            (src.info.width * src.info.components) as i32,
            buffer.as_mut_ptr(),
            dst_w as i32,
            dst_h as i32,
            (dst_w * src.info.components) as i32,
            src.info.components as i32,
        )
    } {
        1 => Ok(Image {
            info: ImageInfo {
                width: dst_w,
                height: dst_h,
                components: src.info.components,
            },
            buffer,
        }),

        _ => Err(STBErr {
            code: ErrCode::ResizeFailed,
            msg: "resize failed".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::load_u8;
    use std::path::PathBuf;

    fn image_path(file: &str) -> PathBuf {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut path = PathBuf::from(root.parent().unwrap());

        path.push("images");
        path.push(file);

        path
    }

    #[test]
    fn test_resize_u8() {
        for c in 1..=4 {
            if c == 2 {
                continue;
            }
            let image_path = image_path("lena.jpg");
            let image = load_u8(&image_path, c).unwrap();
            assert_eq!(image.info.width, 512);
            assert_eq!(image.info.height, 512);
            assert_eq!(image.info.components, c);
            assert_eq!(image.buffer.len(), 512_usize * 512_usize * c);

            let resize = resize_u8(&image, 240, 240);
            assert!(resize.is_ok());

            let resize = resize.unwrap();
            assert_eq!(resize.info.width, 240);
            assert_eq!(resize.info.height, 240);
            assert_eq!(resize.info.components, c);
            assert_eq!(resize.buffer.len(), 240_usize * 240_usize * c);
        }
    }
}
