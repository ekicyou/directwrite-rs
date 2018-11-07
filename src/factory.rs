use error::DWriteError;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{DWriteCreateFactory, IDWriteFactory, DWRITE_FACTORY_TYPE_SHARED};
use winapi::um::unknwnbase::IUnknown;
use winapi::Interface;
use wio::com::ComPtr;

#[derive(Clone, ComWrapper)]
#[com(send, sync)]
#[repr(transparent)]
pub struct Factory {
    ptr: ComPtr<IDWriteFactory>,
}

impl Factory {
    pub fn new() -> Result<Factory, DWriteError> {
        unsafe {
            let mut ptr: *mut IDWriteFactory = ptr::null_mut();
            let hr = DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &IDWriteFactory::uuidof(),
                &mut ptr as *mut _ as *mut *mut IUnknown,
            );

            if SUCCEEDED(hr) {
                Ok(Factory {
                    ptr: ComPtr::from_raw(ptr),
                })
            } else {
                Err(hr.into())
            }
        }
    }
}

