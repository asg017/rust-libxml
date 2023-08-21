use std::{env, fs::File, process::Command};

use cmake::Config;
use flate2::read::GzDecoder;
use std::path::Path;
use tar::Archive;

fn main() {
  if std::env::var("CARGO_FEATURE_BUILD_SOURCE").is_ok() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(out_dir.as_str());

    let download_path = out_dir.join("libxml2-v2.11.5.tar.gz");
    let libxml_target_dir = out_dir.join("libxml2-v2.11.5");

    // Run the curl command to download the file
    let status = Command::new("curl")
      .args([
        "-L",
        "-o",
        download_path.to_str().unwrap(),
        "https://gitlab.gnome.org/GNOME/libxml2/-/archive/v2.11.5/libxml2-v2.11.5.tar.gz",
      ])
      .status()
      .expect("Failed to execute curl command");
    if !status.success() {
      panic!("")
    }
    let tar_gz = File::open(download_path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(out_dir).unwrap();

    let dst = Config::new(libxml_target_dir.to_str().unwrap())
      .define("LIBXML2_WITH_ZLIB", "OFF")
      .define("LIBXML2_WITH_LZMA", "OFF")
      .define("LIBXML2_WITH_HTTP", "OFF")
      .define("LIBXML2_WITH_PYTHON", "OFF")
      .define("LIBXML2_WITH_ICONV", "OFF")
      .define("BUILD_SHARED_LIBS", "OFF")
      .build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=xml2");
  } else if let Ok(ref s) = std::env::var("LIBXML2") {
    // println!("{:?}", std::env::vars());
    // panic!("set libxml2.");
    let p = std::path::Path::new(s);
    let fname = std::path::Path::new(p.file_name().expect("no file name in LIBXML2 env"));
    assert!(p.is_file());
    println!(
      "cargo:rustc-link-lib={}",
      fname
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .strip_prefix("lib")
        .unwrap()
    );
    println!(
      "cargo:rustc-link-search={}",
      p.parent()
        .expect("no library path in LIBXML2 env")
        .to_string_lossy()
    );
  } else {
    #[cfg(any(target_family = "unix", target_os = "macos"))]
    {
      if pkg_config_dep::find() {
        return;
      }
    }

    #[cfg(windows)]
    {
      if vcpkg_dep::find() {
        return;
      }
    }

    panic!("Could not find libxml2.")
  }
}

#[cfg(any(target_family = "unix", target_os = "macos"))]
mod pkg_config_dep {
  pub fn find() -> bool {
    if pkg_config::find_library("libxml-2.0").is_ok() {
      return true;
    }
    false
  }
}

#[cfg(target_family = "windows")]
mod vcpkg_dep {
  pub fn find() -> bool {
    if vcpkg::find_package("libxml2").is_ok() {
      return true;
    }
    false
  }
}
