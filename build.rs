//! Build Script for nrfxlib-sys
//!
//! Calls out to bindgen to generate a Rust crate from the Nordic header
//! files.

fn main() {
	use std::env;
	use std::path::{Path, PathBuf};
	let root_str = env::var("CARGO_MANIFEST_DIR").unwrap();
	let root = Path::new(&root_str);
	let nrfxlib_path = root.join("./third_party/nordic/nrfxlib");
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	let out_file = out_path.join("bindings.rs");
	// Call out to bindgen command line here
	let status = std::process::Command::new("bindgen")
		// The input header we would like to generate
		// bindings for.
		.arg("wrapper.h")
		.arg("--use-core")
		.arg("--ctypes-prefix")
		.arg("ctypes")
		.arg("--whitelist-function")
		.arg("nrf_.*")
		.arg("--whitelist-function")
		.arg("bsd.*")
		.arg("--whitelist-type")
		.arg("nrf.*")
		.arg("--whitelist-var")
		.arg("NRF.*")
		.arg("-o")
		.arg(out_file.to_str().unwrap())
		.arg("--")
		// Point to Nordic headers
		.arg(format!("-I{}", nrfxlib_path.to_str().unwrap()))
		// Point to our special local headers
		.arg("-I./include")
		// Disable standard includes (they belong to the host)
		.arg("-nostdinc")
		// Set the target
		.arg("-target")
		.arg("arm")
		.arg("-mcpu=cortex-m33")
		// Use softfp
		.arg("-mfloat-abi=soft")
		.status()
		.expect("Unable to generate bindings");

	if !status.success() {
		panic!("Failed to run bindgen: {:?}", status);
	}
	println!(
		"cargo:rustc-link-search={}",
		Path::new(&nrfxlib_path)
			.join("bsdlib/lib/cortex-m33/soft-float")
			.display()
	);
	println!(
		"cargo:rustc-link-search={}",
		Path::new(&nrfxlib_path)
			.join("crypto/nrf_oberon/lib/cortex-m33/soft-float")
			.display()
	);
	println!("cargo:rustc-link-lib=static=bsd_nrf9160_xxaa");
	println!("cargo:rustc-link-lib=static=oberon_3.0.2");
}
