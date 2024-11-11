extern crate cc;

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let mut cfg = cc::Build::new();
    let target = env::var("TARGET").unwrap();

    if target.contains("windows") {
        cfg.define("_WIN32", None).flag_if_supported("-UUSE_MSRPC");
    }

    if target.contains("apple") {
        cfg.flag_if_supported("-Wno-deprecated-declarations");
    }

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    cfg.include("./src")
        .define("CONFIG", "\"../config.h\"")
        .define("VERSION", format!("\"{version}\"").as_str())
        .define("IS_LIBRARY", "1")
        .flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-fomit-frame-pointer")
        .flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-Wno-single-bit-bitfield-constant-conversion")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-ignored-qualifiers")
        .flag_if_supported("-UNO_SOCKETS")
        .flag_if_supported("-USIMPLE_RPC")
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
