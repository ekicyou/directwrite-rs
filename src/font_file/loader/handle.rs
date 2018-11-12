use error::DWResult;
use factory::Factory;
use font_file::loader::com_loader::ComFontFileLoader;
use font_file::loader::FontFileLoader;
use key::FontKey;

use com_wrapper::ComWrapper;
use std::marker::PhantomData;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontFileLoader;
use wio::com::ComPtr;

#[repr(C)]
pub struct FileLoaderHandle<K: FontKey + ?Sized> {
    pub(crate) ptr: ComPtr<IDWriteFontFileLoader>,
    _marker: PhantomData<K>,
}

impl<K: FontKey + ?Sized> FileLoaderHandle<K> {
    pub fn register<T>(factory: &Factory, loader: T) -> DWResult<Self>
    where
        T: FontFileLoader<Key = K>,
    {
        unsafe {
            let com = ComFontFileLoader::new(loader);
            let hr = (*factory.get_raw()).RegisterFontFileLoader(com.as_raw());
            if SUCCEEDED(hr) {
                Ok(FileLoaderHandle::from_ptr(com))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn unregister(self, factory: &Factory) {
        unsafe {
            (*factory.get_raw()).UnregisterFontFileLoader(self.ptr.as_raw());
        }
    }
}

impl<K: FontKey + ?Sized> Clone for FileLoaderHandle<K> {
    fn clone(&self) -> Self {
        FileLoaderHandle {
            ptr: self.ptr.clone(),
            _marker: PhantomData,
        }
    }
}

unsafe impl<K: FontKey + ?Sized> Send for FileLoaderHandle<K> {}
unsafe impl<K: FontKey + ?Sized> Sync for FileLoaderHandle<K> {}

impl<K: FontKey + ?Sized> ComWrapper for FileLoaderHandle<K> {
    type Interface = IDWriteFontFileLoader;

    unsafe fn get_raw(&self) -> *mut IDWriteFontFileLoader {
        self.ptr.as_raw()
    }

    unsafe fn into_raw(self) -> *mut IDWriteFontFileLoader {
        self.ptr.into_raw()
    }

    unsafe fn from_raw(raw: *mut IDWriteFontFileLoader) -> Self {
        Self::from_ptr(ComPtr::from_raw(raw))
    }

    unsafe fn from_ptr(ptr: ComPtr<IDWriteFontFileLoader>) -> Self {
        FileLoaderHandle {
            ptr,
            _marker: PhantomData,
        }
    }

    unsafe fn into_ptr(self) -> ComPtr<IDWriteFontFileLoader> {
        self.ptr
    }
}
