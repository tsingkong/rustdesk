#[cfg(windows)]
fn build_windows() {
    let file = "src/platform/windows.cc";
    let file2 = "src/platform/windows_delete_test_cert.cc";
    cc::Build::new().file(file).file(file2).compile("windows");
    println!("cargo:rustc-link-lib=WtsApi32");
    println!("cargo:rerun-if-changed={}", file);
    println!("cargo:rerun-if-changed={}", file2);
}

#[cfg(target_os = "macos")]
fn build_mac() {
    let file = "src/platform/macos.mm";
    let mut b = cc::Build::new();
    if let Ok(os_version::OsVersion::MacOS(v)) = os_version::detect() {
        let v = v.version;
        if v.contains("10.14") {
            b.flag("-DNO_InputMonitoringAuthStatus=1");
        }
    }
    b.file(file).compile("macos");
    println!("cargo:rerun-if-changed={}", file);
}

#[cfg(all(windows, feature = "inline"))]
fn build_manifest() {
    use std::io::Write;
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/icon.ico")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_ENGLISH,
                winapi::um::winnt::SUBLANG_ENGLISH_US,
            ))
            .set_manifest_file("res/manifest.xml");
        match res.compile() {
            Err(e) => {
                write!(std::io::stderr(), "{}", e).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}

fn install_android_deps() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "android" {
        return;
    }
    let mut target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" {
        target_arch = "x64".to_owned();
    } else if target_arch == "x86" {
        target_arch = "x86".to_owned();
    } else if target_arch == "aarch64" {
        target_arch = "arm64".to_owned();
    } else {
        target_arch = "arm".to_owned();
    }
    let target = format!("{}-android", target_arch);
    let vcpkg_root = std::env::var("VCPKG_ROOT").unwrap();
    let mut path: std::path::PathBuf = vcpkg_root.into();
    if let Ok(vcpkg_root) = std::env::var("VCPKG_INSTALLED_ROOT") {
        path = vcpkg_root.into();
    } else {
        path.push("installed");
    }
    path.push(target);
    println!(
        "cargo:rustc-link-search={}",
        path.join("lib").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=ndk_compat");
    println!("cargo:rustc-link-lib=oboe");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=OpenSLES");
}

fn main() {
    hbb_common::gen_version();
    install_android_deps();
    #[cfg(all(windows, feature = "inline"))]
    build_manifest();
    #[cfg(windows)]
    build_windows();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        #[cfg(target_os = "macos")]
        build_mac();
        println!("cargo:rustc-link-lib=framework=ApplicationServices");
    }
    println!("cargo:rustc-link-lib=aom");
    println!("cargo:rustc-link-lib=bluray");
    println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-link-lib=chromaprint");
    println!("cargo:rustc-link-lib=codec2");
    println!("cargo:rustc-link-lib=dav1d");
    println!("cargo:rustc-link-lib=drm");
    println!("cargo:rustc-link-lib=dvdread");
    println!("cargo:rustc-link-lib=dvdnav");
    println!("cargo:rustc-link-lib=gme");
    println!("cargo:rustc-link-lib=gsm");
    println!("cargo:rustc-link-lib=gnutls");
    println!("cargo:rustc-link-lib=jxl");
    println!("cargo:rustc-link-lib=jxl_cms");
    println!("cargo:rustc-link-lib=jxl_extras_codec");
    println!("cargo:rustc-link-lib=jxl_threads");
    println!("cargo:rustc-link-lib=lzma");
    println!("cargo:rustc-link-lib=mp3lame");
    println!("cargo:rustc-link-lib=opencl-clang");
    println!("cargo:rustc-link-lib=OpenCL");
    println!("cargo:rustc-link-lib=openjp2");
    println!("cargo:rustc-link-lib=openmpt");
    println!("cargo:rustc-link-lib=opus");
    println!("cargo:rustc-link-lib=rabbitmq");
    println!("cargo:rustc-link-lib=rav1e");
    println!("cargo:rustc-link-lib=rist");
    println!("cargo:rustc-link-lib=rsvg-2");
    println!("cargo:rustc-link-lib=shine");
    println!("cargo:rustc-link-lib=snappy");
    println!("cargo:rustc-link-lib=speex");
    println!("cargo:rustc-link-lib=srt-gnutls");
    println!("cargo:rustc-link-lib=ssh");
    println!("cargo:rustc-link-lib=SvtAv1Enc");
    println!("cargo:rustc-link-lib=swresample");
    println!("cargo:rustc-link-lib=theora");
    println!("cargo:rustc-link-lib=theoradec");
    println!("cargo:rustc-link-lib=theoraenc");
    println!("cargo:rustc-link-lib=twolame");
    println!("cargo:rustc-link-lib=va");
    println!("cargo:rustc-link-lib=va-drm");
    println!("cargo:rustc-link-lib=va-x11");
    println!("cargo:rustc-link-lib=vdpau");
    println!("cargo:rustc-link-lib=vorbis");
    println!("cargo:rustc-link-lib=vorbisenc");
    println!("cargo:rustc-link-lib=vpx");
    println!("cargo:rustc-link-lib=webp");
    println!("cargo:rustc-link-lib=webpdecoder");
    println!("cargo:rustc-link-lib=webpdemux");
    println!("cargo:rustc-link-lib=webpmux");
    println!("cargo:rustc-link-lib=x264");
    println!("cargo:rustc-link-lib=x265");
    println!("cargo:rustc-link-lib=xml2");
    println!("cargo:rustc-link-lib=xvidcore");
    println!("cargo:rustc-link-lib=zmq");
    println!("cargo:rustc-link-lib=zvbi");
    println!("cargo:rerun-if-changed=build.rs");
}
