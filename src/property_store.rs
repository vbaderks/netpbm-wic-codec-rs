// Copyright (c) Victor Derks.
// SPDX-License-Identifier: MIT

#![allow(non_snake_case)]

use std::cell::RefCell;
use log::debug;
use windows::core::*;
use windows::Win32::Foundation::{BOOL, E_INVALIDARG, STG_E_ACCESSDENIED};
use windows::Win32::System::Com::{IClassFactory_Impl, IStream, IClassFactory};
use windows::Win32::UI::Shell::PropertiesSystem::*;
use windows::Win32::Storage::EnhancedStorage::*;

#[implement(IInitializeWithStream, IPropertyStore)]
struct PropertyStore {
    properties: RefCell<[(*const PROPERTYKEY, i32); 5]>,
    initialized: RefCell<bool>,
}

impl Default for PropertyStore {
    fn default() -> Self {
        PropertyStore {
            properties: RefCell::new([(std::ptr::null(), 0); 5]),
            initialized: RefCell::new(false),
        }
    }
}

impl IInitializeWithStream_Impl for PropertyStore_Impl {
    fn Initialize(&self, _stream: Option<&IStream>, _access_mode: u32) -> Result<()> {
        self.properties.borrow_mut()[0] = (&PKEY_Image_HorizontalSize, 1);
        self.properties.borrow_mut()[1] = (&PKEY_Image_VerticalSize, 2);

        self.initialized.replace(true);

        Ok(())
    }
}

impl IPropertyStore_Impl for PropertyStore_Impl {
    fn GetCount(&self) -> Result<u32> {
        debug!("property_store-r::GetCount");
        if *self.initialized.borrow() { Ok(self.properties.borrow().len() as u32) } else { Ok(0) }
    }

    fn GetAt(&self, index: u32, key: *mut PROPERTYKEY) -> Result<()> {
        debug!("property_store-r::GetAt");

        let properties = self.properties.borrow();

        if index > properties.len() as u32 || !(*self.initialized.borrow()) {
            return E_INVALIDARG.ok();
        }

        unsafe { *key = *properties[index as usize].0; }

        Ok(())
    }

    fn GetValue(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        for property in *self.properties.borrow() {
            if property.0 == key {
                return Ok(PROPVARIANT::new());
            }
        }

        Ok(PROPVARIANT::default())
    }

    fn SetValue(&self, _key: *const PROPERTYKEY, _value: *const PROPVARIANT) -> Result<()> {
        debug!("property_store-r::SetValue");
        STG_E_ACCESSDENIED.ok()
    }

    fn Commit(&self) -> Result<()> {
        debug!("property_store-r::Commit");
        STG_E_ACCESSDENIED.ok()
    }
}


#[implement(IClassFactory)]
struct Factory();

impl IClassFactory_Impl for Factory_Impl {
    fn CreateInstance(
        &self,
        outer: Option<&IUnknown>,
        iid: *const GUID,
        object: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        assert!(outer.is_none());
        let unknown: IUnknown = PropertyStore::default().into();
        unsafe { unknown.query(iid, object).ok() }
    }

    fn LockServer(&self, lock: BOOL) -> Result<()> {
        assert!(lock.as_bool());
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use windows::Win32::System::Com::IClassFactory;
    use super::*;

    #[test]
    fn get_count() -> Result<()> {
        let initialize_with_stream: IInitializeWithStream = PropertyStore::default().into();
        unsafe { initialize_with_stream.Initialize(None, 0)? };

        let property_store: IPropertyStore = initialize_with_stream.cast()?;

        let count = unsafe { property_store.GetCount()? };
        assert_eq!(5u32, count);
        Ok(())
    }

    #[test]
    fn create_using_class_factory() -> Result<()> {
        let factory: IClassFactory = Factory().into();

        let initialize_with_stream: IInitializeWithStream = unsafe { factory.CreateInstance(None) }?;
        unsafe { initialize_with_stream.Initialize(None, 0)? };

        let property_store: IPropertyStore = initialize_with_stream.cast()?;
        let count = unsafe { property_store.GetCount()? };
        assert_eq!(5u32, count);
        Ok(())
    }
}
