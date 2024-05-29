use std::{env, path::PathBuf, process::Command};

use bindgen::Builder;

const FDKAAC_HEADERS: [&str; 2] = [
    "libAACenc/include/aacenc_lib.h",
    "libAACdec/include/aacdecoder_lib.h",
];

const FDKAAC_INCLUDES: [&str; 13] = [
    "libSACenc/include",
    "libSACdec/include",
    "libSYS/include",
    "libPCMutils/include",
    "libAACdec/include",
    "libDRCdec/include",
    "libFDK/include",
    "libMpegTPDec/include",
    "libSBRdec/include",
    "libAACenc/include",
    "libArithCoding/include",
    "libSBRenc/include",
    "libMpegTPEnc/include",
];

fn git_fetch(repo: &str, reference: &str, output: &str) {
    #[rustfmt::skip]
    Command::new("git")
        .args([
            "clone",
            "--depth=1",
            "--recurse-submodules",
            repo,
            output,
        ])
        .status()
        .expect("Unable to clone repository.");
    Command::new("git")
        .args(["checkout", reference])
        .status()
        .expect("Unable to checkout repository.");
}

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn main() {
    let mut fdk_path = output();
    fdk_path.push("fdk-aac");

    git_fetch(
        "git@github.com:mstorsjo/fdk-aac.git",
        "v2.0.3",
        &fdk_path.to_string_lossy(),
    );
    Command::new("cmake")
        .current_dir(&fdk_path)
        .args([".", "-DCMAKE_BUILD_TYPE=Release", "-DBUILD_SHARED_LIBS=OFF"])
        .status()
        .expect("Unable to run cmake.");

    Command::new("make")
        .current_dir(&fdk_path)
        .args(["-j"])
        .status()
        .expect("Unable to run make.");

    let includes = ["-I".to_string()]
        .into_iter()
        .cycle()
        .zip(FDKAAC_INCLUDES.into_iter().map(|item| {
            let mut tmp = fdk_path.clone();
            tmp.push(item);
            tmp.to_string_lossy().to_string()
        }))
        .flat_map(|tup| vec![tup.0, tup.1])
        .collect::<Vec<String>>();

    let mut bindings = Builder::default().clang_args(includes);
    for header in FDKAAC_HEADERS {
        let mut tmp = fdk_path.clone();
        tmp.push(header);
        bindings = bindings.header(tmp.to_string_lossy());
    }

    bindings
        .generate()
        .expect("Unable to generate bindings.")
        .write_to_file(format!("{}/fdkaac.rs", output().to_string_lossy()))
        .expect("Unable to write generated bindings.");

    println!(
        "cargo:rustc-link-search=native={}",
        dbg!(fdk_path).to_string_lossy()
    );
    println!("cargo:rustc-link-lib=fdk-aac");
}
