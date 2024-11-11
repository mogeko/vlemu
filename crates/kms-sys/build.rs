extern crate cc;

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let mut cfg = cc::Build::new();
    let target = env::var("TARGET").unwrap();

    if target.contains("apples") {
        cfg.flag_if_supported("-Wno-deprecated-declarations");
    }

    if target.contains("windows") {
        cfg.define("_WIN32", None);
    }

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    cfg.include("./src")
        .define("CONFIG", "\"config.h\"")
        .define("VERSION", format!("\"{version}\"").as_str())
        .define("IS_LIBRARY", "1")
        .define("SIMPLE_SOCKETS", None)
        .define("NO_TIMEOUT", None)
        .define("NO_SIGHUP", None)
        .define("NO_CL_PIDS", None)
        .define("NO_LOG", None)
        .define("NO_RANDOM_EPID", None)
        .define("NO_INI_FILE", None)
        .define("NO_HELP", None)
        .define("NO_CUSTOM_INTERVALS", None)
        .define("NO_PID_FILE", None)
        .define("NO_USER_SWITCH", None)
        .define("NO_VERBOSE_LOG", None)
        .define("NO_LIMIT", None)
        .define("NO_VERSION_INFORMATION", None)
        .define("NO_PRIVATE_IP_DETECT", None)
        .define("NO_STRICT_MODES", None)
        .define("NO_CLIENT_LIST", None)
        .define("NO_TAP", None)
        .flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-fomit-frame-pointer")
        .flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-Wno-single-bit-bitfield-constant-conversion")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-ignored-qualifiers")
        .flag_if_supported("-UNO_SOCKETS")
        .flag_if_supported("-USIMPLE_RPC")
        .flag_if_supported("-UUSE_MSRPC")
        .file("./src/crypto.c")
        .file("./src/crypto_internal.c")
        .file("./src/endian.c")
        .file("./src/helpers.c")
        .file("./src/kms.c")
        .file("./src/libkms.c")
        .file("./src/network.c")
        .file("./src/output.c")
        .file("./src/rpc.c")
        .file("./src/shared_globals.c")
        .file("./src/vlmcs.c")
        .out_dir(dst.join("lib"))
        .compile("kms");

    let src = env::current_dir().unwrap().join("src");
    let include = dst.join("include");
    fs::create_dir_all(&include).unwrap();
    fs::copy(src.join("libkms.h"), dst.join("include/libkms.h")).unwrap();
    println!("cargo::rustc-link-search={}", dst.join("include").display());
    println!("cargo::rustc-link-lib=static=kms");
}
