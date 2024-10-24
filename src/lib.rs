// Copyright (c) Victor Derks.
// SPDX-License-Identifier: MIT

mod guids;
mod property_store;

use windows::{
    core::*, Win32::System::Com::*,
    Win32::Graphics::Imaging::*,
};
use windows::Win32::Foundation::{HINSTANCE, S_FALSE, S_OK};
use windows::Win32::System::Ole::*;
use windows::Win32::UI::Shell::*;
use windows_registry::*;
use log::debug;
use guids::*;

const MIME_TYPES: &str = "image/x-portable-graymap,image/x-portable-pixmap";


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => process_attach()
    }

    true
}

#[no_mangle]
extern "system" fn DllRegisterServer() -> HRESULT {
    debug!("netpbm-wic-codec-rs::DllRegisterServer");
    match register_server() {
        Ok(_) => S_OK,
        Err(_) => SELFREG_E_CLASS
    }
}

#[no_mangle]
extern "system" fn DllUnregisterServer() -> HRESULT {
    debug!("netpbm-wic-codec-rs::DllUnregisterServer");

    // Note: keep the file registrations intact.
    unregister_server().into()
}


#[no_mangle]
extern "system" fn DllGetClassObject(class_id: *const GUID, _interface_id: *const GUID,
                                            _factory: OutRef<IClassFactory>) -> HRESULT {

    unsafe {
        if *class_id == guids::NETPBM_DECODER_ID {}

        if *class_id == guids::PROPERTY_STORE_CLASS_ID {}
    }

    S_OK
}

static mut REF_COUNT: u32 = 0;

#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    unsafe {
        if REF_COUNT == 0 {
            S_OK
        } else {
            S_FALSE
        }
    }
}

fn process_attach() {
    log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    debug!("netpbm-wic-codec-rs::DllMain DLL_PROCESS_ATTACH");
}

fn register_server() -> Result<()> {
    register_decoder()?;
    register_property_store()?;

    unsafe { SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None) };
    Ok(())
}

fn register_decoder() -> Result<()> {
    register_general_decoder_settings(&NETPBM_DECODER_ID)
}

fn register_general_decoder_settings(class_id: &GUID) -> Result<()> {
    let key = LOCAL_MACHINE.create("SOFTWARE\\Classes\\CLSID\\".to_string() +
        &guid_to_string(class_id))?;

    key.set_u32("ArbitrationPriority", 10)?;
    key.set_string("Author", "Team CharLS")?;
    key.set_string("ColorManagementVersion", "1.0.0.0")?;
    key.set_string("ContainerFormat", &*guid_to_string(&guids::CONTAINER_FORMAT_NETPBM_ID))?;
    key.set_string("Description", "Netpbm Codec")?;
    key.set_string("FileExtensions", ".pgm,.ppm")?;
    key.set_string("FriendlyName", "Victor's Netpbm Decoder (Rust)")?;
    key.set_string("MimeTypes", MIME_TYPES)?;
    key.set_string("SpecVersion", "1.0.0.0")?;
    key.set_u32("SupportAnimation", 0)?;
    key.set_u32("SupportChromaKey", 0)?;
    key.set_u32("SupportLossless", 1)?;
    key.set_u32("SupportMultiframe", 0)?;
    key.set_string("Vendor", &*guid_to_string(&guids::VENDOR_VICTOR_ID))?;
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    key.set_string("Version", VERSION)?;

    // WIC category registration.
    let category_id_key = LOCAL_MACHINE.create("(SOFTWARE\\Classes\\CLSID\\)".to_string() +
        &guid_to_string(&CATID_WICBitmapDecoders) + "(\\Instance\\)" +
        &guid_to_string(class_id))?;
    category_id_key.set_string("FriendlyName", "Victor's Netpbm Decoder")?;
    category_id_key.set_string("CLSID", &guid_to_string(class_id))
}

fn register_property_store() -> Result<()> {
    // TODO implement
    Ok(())
}

fn unregister_server() -> Result<()> {
    let result1 = LOCAL_MACHINE.remove_tree("SOFTWARE\\Classes\\CLSID\\".to_string() +
        &guid_to_string(&NETPBM_DECODER_ID));
    let result2 = LOCAL_MACHINE.remove_tree("SOFTWARE\\Classes\\CLSID\\".to_string() +
        &guid_to_string(&CATID_WICBitmapDecoders) + "(\\Instance\\)" +
        &guid_to_string(&NETPBM_DECODER_ID));

    if result1.is_err() { result1 } else { result2 }
}


fn guid_to_string(guid: &GUID) -> String {
    // Create a buffer to hold the wide string (39 characters max for GUID + null terminator)
    let mut buffer = [0u16; 39];

    // Call StringFromGUID2
    let len = unsafe {
        StringFromGUID2(guid, &mut buffer)
    };

    // Convert wide string to a Rust String
    String::from_utf16_lossy(&buffer[..(len as usize - 1)]) // Exclude the null terminator
}

#[no_mangle]
pub extern fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
