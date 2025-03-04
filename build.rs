use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
    // Try building w/ current extracted library
    let result = std::panic::catch_unwind(build_lib);
    match result {
        Err(..) => {// If current extracted lib doesn't work / doesn't exist, extract from the tar and try again.
            // Remove curr lib directory since it might be corrupted
            let lib_dir = Path::new("./libs/assimp-3.1.1");
            let _ = std::fs::remove_dir_all(lib_dir);

            // Unpack tar and build again.
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
    cmake::Config::new("./libs/assimp-3.1.1")
        .configure_arg("-DASSIMP_BUILD_ASSIMP_TOOLS=OFF")
        .configure_arg("-DASSIMP_BUILD_TESTS=OFF")
        .build();
    let dst = cmake::Config::new("./libs/assimp-3.1.1")
        .configure_arg("-DASSIMP_BUILD_ASSIMP_TOOLS=OFF")
        .configure_arg("-DASSIMP_BUILD_TESTS=OFF")
        .configure_arg("-DASSIMP_BUILD_STATIC_LIB=ON")
        .build()
        .join("lib");
    println!("cargo:rustc-link-search=static={}", dst.display());
    println!("cargo:rustc-link-search=native={}", dst.display());
}
