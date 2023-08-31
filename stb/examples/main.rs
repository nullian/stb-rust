use stb_rust::{common::Image, image, image_resize, image_write};
use std::path::PathBuf;
fn main() {
    load_and_write_image();

    resize_images();
}

fn get_image(components: usize) -> (PathBuf, Image<u8>) {
    let module_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut image_path = PathBuf::from(module_root.parent().unwrap());
    image_path.push("images");
    image_path.push("lena.jpg");

    let image_u8 = image::load_u8(&image_path, components).unwrap();
    println!(
        "image w:{}, h:{}, channel:{}",
        image_u8.info.width, image_u8.info.height, image_u8.info.components
    );

    (image_path, image_u8)
}

fn load_and_write_image() {
    let (image_path, image_u8) = get_image(3);

    let out_path_dir = PathBuf::from(image_path.parent().unwrap());
    let out_path_jpg = out_path_dir.join("load_and_write.jpg");

    image_write::write_jpg(&out_path_jpg, &image_u8, 100_i32).expect("write jpg failed");

    let out_path_png = out_path_dir.join("load_and_write.png");
    image_write::write_png(&out_path_png, &image_u8).expect("write png failed");

    let out_path_bmp = out_path_dir.join("load_and_write.bmp");
    image_write::write_bmp(&out_path_bmp, &image_u8).expect("write bmp failed");

    let out_path_tga = out_path_dir.join("load_and_write.tga");
    image_write::write_tga(&out_path_tga, &image_u8).expect("tga jpg failed");
}

fn resize_images() {
    let downsample_w = 240;
    let downsample_h = 240;
    let upsample_w = 1000;
    let upsample_h = 1000;

    for c in 1..=4 {
        if c == 2 {
            continue;
        }

        let (image_path, image_u8) = get_image(c);
        let out_path_dir = PathBuf::from(image_path.parent().unwrap());

        assert_eq!(image_u8.info.width, 512);
        assert_eq!(image_u8.info.height, 512);
        assert_eq!(image_u8.info.components, c);
        assert_eq!(image_u8.buffer.len(), 512_usize * 512_usize * c);

        let down_sample_resize = image_resize::resize_u8(&image_u8, 240, 240);
        assert!(down_sample_resize.is_ok());

        let resize = down_sample_resize.unwrap();
        assert_eq!(resize.info.width, downsample_w);
        assert_eq!(resize.info.height, downsample_h);
        assert_eq!(resize.info.components, c);
        assert_eq!(resize.buffer.len(), downsample_w * downsample_h * c);

        let out_path_dir = PathBuf::from(&out_path_dir);
        let out_filename = format!("resize_downsample_{}.jpg\0", c.to_string());

        let out_path_jpg = PathBuf::from(&out_path_dir).join(out_filename.as_str());
        assert!(image_write::write_jpg(&out_path_jpg, &resize, 100).is_ok());

        let up_sample_resize = image_resize::resize_u8(&image_u8, upsample_w, upsample_h);
        assert!(up_sample_resize.is_ok());

        let resize = up_sample_resize.unwrap();
        assert_eq!(resize.info.width, upsample_w);
        assert_eq!(resize.info.height, upsample_h);
        assert_eq!(resize.info.components, c);
        assert_eq!(resize.buffer.len(), upsample_w * upsample_h * c);

        let out_filename = format!("resize_upsample_{}.jpg\0", c.to_string());
        let out_path_jpg = PathBuf::from(&out_path_dir).join(out_filename.as_str());
        assert!(image_write::write_jpg(&out_path_jpg, &resize, 100).is_ok());
    }
}
