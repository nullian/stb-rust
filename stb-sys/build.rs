use std::env;
use std::path::PathBuf;

static FILES: &[&str] = &[
    "src/stb_image.c",
    "src/stb_image_write.c",
    "src/stb_image_resize.c",
];

static HEADER: &[&str] = &[
    "stb/stb_image.h",
    "stb/stb_image_write.h",
    "stb/stb_image_resize.h",
];

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_dir.join("bindings.rs");

    if FILES.is_empty() {
        std::fs::write(bindings_path, "").unwrap();
        return;
    }

    let mut builder = bindgen::builder();
    for f in HEADER {
        builder = builder.header(*f)
    }

    builder
        .allowlist_function("stb.*")
        .allowlist_type("stb.*")
        .allowlist_var("stb.*")
        .use_core()
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(bindings_path)
        .expect("Failed to write bindings file");

    let mut builder = cc::Build::new();

    builder.files(FILES).warnings(false).compile("stb");
}
