//! FontFile and types for creating your own instances.

use crate::descriptions::FontKey;
use crate::enums::FontFaceType;
use crate::enums::FontFileType;
use crate::factory::IFactory;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontFile;
use wio::com::ComPtr;

#[doc(inline)]
pub use crate::font_file::builder::FontFileBuilder;

#[doc(hidden)]
pub mod builder;
pub mod loader;

#[derive(Clone, ComWrapper, PartialEq)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// Represents a font file. Applications such as font managers or font viewers can call `analyze`
/// to find out if a particular file is a font file, and whether it is a font type that is
/// supported by the font system.
pub struct FontFile {
    ptr: ComPtr<IDWriteFontFile>,
}

impl FontFile {
    /// Initializes a builder for creating a FontFile from either custom loaders or a file path.
    pub fn create<K: FontKey + ?Sized>(factory: &dyn IFactory) -> FontFileBuilder<K> {
        unsafe { FontFileBuilder::new(factory.raw_f()) }
    }
}

pub unsafe trait IFontFile {
    /// Analyzes a file and returns whether it represents a font, and whether the font type is
    /// supported by the font system.
    fn analyze(&self) -> Result<Analysis, Error> {
        unsafe {
            let mut sup = 0;
            let mut file = 0;
            let mut face = 0;
            let mut num = 0;

            let hr = self
                .raw_fontfile()
                .Analyze(&mut sup, &mut file, &mut face, &mut num);

            if SUCCEEDED(hr) {
                Ok(Analysis {
                    supported: sup != 0,
                    file_type: file.into(),
                    face_type: face.into(),
                    num_faces: num,
                })
            } else {
                Err(hr.into())
            }
        }
    }

    fn as_font_file(&self) -> FontFile {
        unsafe {
            let ptr = self.raw_fontfile();
            ptr.AddRef();
            FontFile::from_raw(ptr as *const _ as *mut _)
        }
    }

    unsafe fn raw_fontfile(&self) -> &IDWriteFontFile;
}

unsafe impl IFontFile for FontFile {
    unsafe fn raw_fontfile(&self) -> &IDWriteFontFile {
        &self.ptr
    }
}

/// Results of analyzing a font file.
pub struct Analysis {
    /// Whether the font type is supported by the font system.
    pub supported: bool,

    /// Indicates the identified type of font file this is. May be a value other than
    /// `Unknown` even if the font type is not supported by the system.
    pub file_type: UncheckedEnum<FontFileType>,

    /// Indicates the type of the font face. This will be a meaningful value if file_type is
    /// not `Unknown` indicating that this file can be constructed if it is supported.
    pub face_type: UncheckedEnum<FontFaceType>,

    /// The number of font faces contained in the font file.
    pub num_faces: u32,
}
