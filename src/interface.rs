use exif::{Exif, In, Reader, Tag};
use std::os::raw::c_void;

#[repr(C)]
pub struct ExifData {
    val: *mut c_void,
}

impl ExifData {
    pub(crate) fn make_null() -> Self {
        ExifData {
            val: std::ptr::null_mut(),
        }
    }

    pub(crate) fn is_null(&self) -> bool {
        self.val.is_null()
    }

    pub(crate) fn from_exif(exif: Exif) -> Self {
        // Move exif onto the heap and make the box give up ownership.
        let exif_ptr: *mut Exif = Box::into_raw(Box::new(exif));
        Self {
            val: exif_ptr as *mut c_void,
        }
    }

    pub(crate) fn to_exif(self) -> Result<&'static Exif, ErrorCodes> {
        if self.is_null() {
            return Err(ErrorCodes::Nullptr);
        }

        Ok(unsafe { &*(self.val as *mut Exif) })
    }

    pub(crate) fn drop_explicitly(self) -> ErrorCodes {
        if self.is_null() {
            return ErrorCodes::Nullptr;
        }

        // Once the box goes out of scope, the Exif struct on the heap will be dropped and deallocated as well.
        let _box_to_drop = unsafe { Box::from_raw(self.val as *mut Exif) };
        ErrorCodes::Ok
    }
}

#[repr(C)]
pub struct ExifParseResult {
    pub(crate) data: ExifData,
    pub(crate) error_code: ErrorCodes,
}

impl ExifParseResult {
    pub(crate) fn parse_error() -> Self {
        Self {
            data: ExifData::make_null(),
            error_code: ErrorCodes::ParseError,
        }
    }

    pub(crate) fn make_null() -> Self {
        Self {
            data: ExifData::make_null(),
            error_code: ErrorCodes::Nullptr,
        }
    }
}

#[repr(C)]
pub enum ErrorCodes {
    Ok,
    Nullptr,
    ParseError,
}
