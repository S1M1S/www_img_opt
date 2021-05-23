use std::fs;
use std::path::Path;
use std::error;
use raster::transform;

fn resize_image(path: &Path, size: i32) {
    let src = &path.to_str().unwrap();
    let mut image = raster::open(src).unwrap();
    transform::resize_exact_width(&mut image, size).expect("failed to transform image"); 
    raster::save(&image, "sc_resized.png").unwrap();
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // list all of the image files in the directory
    let dir_entries = fs::read_dir(".")?
        .filter_map(|result| {
            let path = result.unwrap().path();
            match path.extension() {
                Some(ext) => {
                    match ext.to_str().unwrap() {
                        "png" => Some(path),
                        _ => None,
                    }
                },
                None => None,
           }
        });
    println!("{:?}", &dir_entries.collect::<Vec<_>>());

    // make image variants and save them to a subdirectory
    // for dir_entry in &dir_entries {
    //   resize_image(&dir_entry.path(), 100)
    // }
    //
    // create a manifest of the current file listing and variants
    Ok(())
}
