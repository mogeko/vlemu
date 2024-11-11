use kms::get_libkms_version;

fn main() {
    let lib_version = env!("CARGO_PKG_VERSION");
    let libkms_version = get_libkms_version();

    println!("libself {}", lib_version);
    println!("libkms  {}", libkms_version);
}
