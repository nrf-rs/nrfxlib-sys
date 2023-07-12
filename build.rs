//! Build Script for nrfxlib-sys
//!
//! Calls out to bindgen to generate a Rust crate from the Nordic header
//! files.

fn main() {
	use std::env;
	use std::path::{Path, PathBuf};
	let nrfxlib_path = "./third_party/nordic/nrfxlib";
	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
		// The input header we would like to generate
		// bindings for.
		.header("wrapper.h")
		// Point to Nordic headers
		.clang_arg(format!("-I{}", nrfxlib_path))
		// Point to our special local headers
		.clang_arg("-I./include")
		// Add extra paths that the C files assume are searched
		.clang_arg("-I./third_party/nordic/nrfxlib/crypto/nrf_cc310_platform/include")
		.clang_arg("-I./third_party/nordic/nrfxlib/crypto/nrf_oberon")
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
		.allowlist_function("nrf_.*")
		.allowlist_function("ocrypto_.*")
		.allowlist_function("bsd_.*")
		.allowlist_type("nrf_.*")
		.allowlist_type("ocrypto_.*")
		.allowlist_var("NRF_.*")
		.allowlist_var("BSD_.*")
		.allowlist_var("OCRYPTO_.*")
		// Format the output
		.rustfmt_bindings(true)
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let mut rust_source = bindings.to_string();

	// Munge Doxygen comments into something Rustdoc can handle
	rust_source = rust_source.replace("#[doc = \"@{*/\"]", "");
	let re = regex::Regex::new("\"   \\s+- ").unwrap();
	rust_source = re.replace_all(&rust_source, "\" * ").into();
	let re = regex::Regex::new(r"\s*@param\s+(?P<var>[A-Za-z0-9_]+)\s+").unwrap();
	rust_source = re.replace_all(&rust_source, " * `$var` - ").into();
	let re =
		regex::Regex::new(r"\s*@param\[(out|in|inout|in,out)\](\\t|\s+)(?P<var>[A-Za-z0-9_]+)\s+")
			.unwrap();
	rust_source = re.replace_all(&rust_source, " * `$var` - ").into();
	let re = regex::Regex::new(r"@[cp]\s+(?P<var>[A-Za-z0-9_\(\)]+)").unwrap();
	rust_source = re.replace_all(&rust_source, " * `$var` - ").into();
	let re = regex::Regex::new(r"\\\\[cp]\s+(?P<var>[A-Za-z0-9_\(\)]+)").unwrap();
	rust_source = re.replace_all(&rust_source, "`$var`").into();
	let re = regex::Regex::new(r"\\\\ref\s+(?P<var>[A-Za-z0-9_\(\)]+)").unwrap();
	rust_source = re.replace_all(&rust_source, "`$var`").into();
	rust_source = rust_source.replace("\" @remark", "\" NB: ");
	rust_source = rust_source.replace("\"@brief", "\"");
	rust_source = rust_source.replace("\" @brief", "\" ");
	rust_source = rust_source.replace("\"@detail", "\"");
	rust_source = rust_source.replace("\" @detail", "\" ");
	rust_source = rust_source.replace("@name ", "# ");
	rust_source = rust_source.replace("@return ", "Returns ");
	rust_source = rust_source.replace("@retval ", "Returns ");

	let bindings_out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
	std::fs::write(bindings_out_path, rust_source).expect("Couldn't write updated bindgen output");

	let libmodem_original_path =
		Path::new(&nrfxlib_path).join("nrf_modem/lib/cortex-m33/hard-float/libmodem.a");
	let libmodem_changed_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libmodem.a");

	// The modem library now has compressed headers, but Rust cannot deal with that.
	// If the appropriate features is active, we're gonna strip it or decompress it.
	#[cfg(feature = "arm-none-eabi-objcopy")]
	{
		// We assume the arm-none-eabi-objcopy comes from the official arm website.
		let child = std::process::Command::new("arm-none-eabi-objcopy")
			.arg("--decompress-debug-sections")
			.arg(&libmodem_original_path)
			.arg(&libmodem_changed_path)
			.spawn()
			.expect("Could not start `arm-none-eabi-objcopy`. Is it installed and available in your path?");

		let child_result = child.wait_with_output().unwrap();
		if !child_result.status.success() {
			panic!("Something went wrong with `arm-none-eabi-objcopy`.");
		}
	}
	#[cfg(feature = "llvm-objcopy")]
	{
		// We assume the llvm-objcopy comes from the rustup llvm-tools.
		// This cannot do decompression, so we'll just strip the debug sections

		let tool_error = "Could not find `llvm-objcopy`. Is it installed? Use `rustup component add llvm-tools` to install it or select the `arm-none-eabi-objcopy` feature if you have that tool installed.";
		// It's not in our path, so we have to search for it
		let path = llvm_tools::LlvmTools::new()
			.expect(tool_error)
			.tool(&llvm_tools::exe("llvm-objcopy"))
			.expect(tool_error);

		let child = std::process::Command::new(path)
			.arg("--strip-debug")
			.arg(&libmodem_original_path)
			.arg(&libmodem_changed_path)
			.spawn()
			.expect(tool_error);

		let child_result = child.wait_with_output().unwrap();
		if !child_result.status.success() {
			panic!("Something went wrong with `llvm-objcopy`.");
		}
	}

	// Make sure we link against the libraries
	println!(
		"cargo:rustc-link-search={}",
		libmodem_changed_path.parent().unwrap().display()
	);
	println!(
		"cargo:rustc-link-search={}",
		Path::new(&nrfxlib_path)
			.join("crypto/nrf_oberon/lib/cortex-m33/hard-float")
			.display()
	);
	println!("cargo:rustc-link-lib=static=modem");
	println!("cargo:rustc-link-lib=static=oberon_3.0.13");
}
