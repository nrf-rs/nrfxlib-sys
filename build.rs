#[cfg(workaround_build)]
fn main() {
	use std::env;
	use std::path::{Path, PathBuf};
	let root_str = env::var("CARGO_MANIFEST_DIR").unwrap();
	let root = Path::new(&root_str);
	let nrfxlib_path = root.join("./third_party/nordic/nrfxlib");
	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
		// The input header we would like to generate
		// bindings for.
		.header("wrapper.h")
		// Point to Nordic headers
		.clang_arg(format!("-I{}", nrfxlib_path.to_str().unwrap()))
		// Point to our special local headers
		.clang_arg("-I./include")
		// Disable standard includes (they belong to the host)
		.clang_arg("-nostdinc")
		// Set the target
		.clang_arg("-target")
		.clang_arg("arm")
		.clang_arg("-mcpu=cortex-m33")
		// Use softfp
		.clang_arg("-mfloat-abi=soft")
		// We're no_std
		.use_core()
		// Use our own ctypes to save using libc
		.ctypes_prefix("ctypes")
		// Include only the useful stuff
		.whitelist_function("nrf_.*")
		.whitelist_function("bsd_.*")
		.whitelist_type("nrf_.*")
		.whitelist_var("NRF_.*")
		// Format the output
		.rustfmt_bindings(true)
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");

	// Make sure we link against the libraries
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
	println!("cargo:rustc-link-lib=static=oberon_3.0.0");
}
