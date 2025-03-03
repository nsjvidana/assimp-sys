use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
    // Try building w/ current extracted library
    let result = std::panic::catch_unwind(build_lib);
    match result {
        Err(..) => {// If current extracted lib doesn't work / doesn't exist, extract from the tar and try again.
            match reqwest::blocking::get("https://github.com/assimp/assimp/archive/v3.1.1.tar.gz") {
                Ok(mut resp) => {
                    let path = Path::new("./libs/assimp-3.1.1.tar.gz");
                    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                    let mut tar_out = File::create(path)
                        .expect("Failed to create assimp tar file");
                    resp.copy_to(&mut tar_out).expect("Failed to copy data to assimp tar file");
                },
                Err(e) => panic!("Failed downloading tar file. Error:\n    {:?}", e),
            }

            // Unpack the tar file & try building assimp again
            let tar_gz = File::open("./libs/assimp-3.1.1.tar.gz")
                .expect("Couldn't open assimp-3.1.1.tar.gz");
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            archive.unpack("./libs/").expect("Failed to unpack assimp-3.1.1.tar.gz");
            build_lib();
        },
        _ => {}
    }
}

fn build_lib() {
    let dst = cmake::Config::new("./libs/assimp-3.1.1")
        .configure_arg("-DASSIMP_BUILD_ASSIMP_TOOLS=OFF")
        .configure_arg("-DASSIMP_BUILD_TESTS=OFF")
        .build()
        .join("lib");
    println!("cargo:rustc-link-search=static={}", dst.display());
    println!("cargo:rustc-link-search=native={}", dst.display());   
}
