#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]
// bindgen-generated bitfield accessors transmute small ints to/from themselves.
#![allow(clippy::useless_transmute)]
// bindgen converts Doxygen list items in a way that overindents under clippy's rule.
#![allow(clippy::doc_overindented_list_items)]
// bindgen emits wide-arity bitfield constructors for the new DECT MAC structs.
#![allow(clippy::too_many_arguments)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Compat shim: libmodem 3.3 renamed `NRF_MODEM_DECT_PHY_SHMEM_CTRL_SIZE` to
// `NRF_MODEM_DECT_SHMEM_CTRL_SIZE`. Keep the old name available so downstream
// crates (notably `nrf-modem`) continue to compile against this version.
pub const NRF_MODEM_DECT_PHY_SHMEM_CTRL_SIZE: u32 = NRF_MODEM_DECT_SHMEM_CTRL_SIZE;
