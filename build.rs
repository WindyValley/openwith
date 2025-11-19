use std::env;
use std::fs;
use std::path::Path;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=data");

  if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
      println!("cargo:rerun-if-changed=manifest.xml");
      println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
      println!("cargo:rustc-link-arg-bins=/SUBSYSTEM:WINDOWS");
      println!("cargo:rustc-link-arg-bins=/Entry:mainCRTStartup");

      println!(
          "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
          std::path::Path::new("manifest.xml")
              .canonicalize()
              .unwrap()
              .display()
      );
  }

  // 复制 data 目录到构建结果目录
  let out_dir = env::var("OUT_DIR").unwrap();
  
  // 构建目标目录路径 (target/debug 或 target/release)
  let target_dir = Path::new(&out_dir)
      .ancestors()
      .nth(3)
      .unwrap();
  
  let data_src = Path::new("data");
  let data_dst = target_dir.join("data");
  
  // 如果目标 data 目录已存在，先删除
  if data_dst.exists() {
      fs::remove_dir_all(&data_dst).unwrap();
  }
  
  // 复制整个 data 目录
  copy_dir_all(&data_src, &data_dst).unwrap();
  
  println!("Data directory copied to: {}", data_dst.display());
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}