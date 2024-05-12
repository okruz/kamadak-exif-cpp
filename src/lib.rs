/*
 * ########################################################################
 * The contents of this file is free and unencumbered software released into the
 * public domain. For more information, please refer to <http://unlicense.org/>
 * ########################################################################
 */

#![crate_type = "cdylib"]
mod interface;
use crate::interface::*;
use exif::{Exif, In, Reader, Tag};
use std::ffi::c_char;
use std::io::Cursor;

/// Loads the exif data from encoded image.
///
/// Attempts to parse the EXIF data from the provided data. If the parsing was unsuccessful,
/// the ExifParseResult::error_code will indicate what went wrong and the ExifParseResult::data::val
/// pointer will be a nullptr. Else, the the ExifParseResult::error_code will ErrorCodes::Ok and the
/// ExifParseResult::data::val will point to the parsed data.
#[no_mangle]
pub extern "C" fn EXIF_load(data: *const u8, length: usize) -> ExifParseResult {
    if data.is_null() {
        return ExifParseResult::make_null();
    }
    let slice = unsafe { std::slice::from_raw_parts(data, length) };
    let mut buff = Cursor::new(slice);

    let exif = match Reader::new().read_from_container(&mut buff) {
        Err(_) => return ExifParseResult::parse_error(),
        Ok(val) => val,
    };

    ExifParseResult {
        data: ExifData::from_exif(exif),
        error_code: ErrorCodes::Ok,
    }
}

/// Frees the allocated memory.
///
/// Frees all data allocated during the extraction of the EXIF information.
/// If the returned ErrorCodes is not ErrorCodes::Ok, the deallocation could not be performed.
#[no_mangle]
pub extern "C" fn EXIF_free(data: ExifData) -> ErrorCodes {
    data.drop_explicitly()
}

/// Returns the little endian attribute from the EXIF header.
///
/// Indicates whether or not the data in the EXIF fields are encoded in little endian or big endian
/// byte order. If the returned ErrorCodes is not ErrorCodes::Ok, the value could not be extracted.
#[no_mangle]
pub extern "C" fn EXIF_is_little_endian(data: ExifData, little_endian: &mut bool) -> ErrorCodes {
    let exif_scope = match data.to_exif_scope() {
        Err(error_code) => return error_code,
        Ok(val) => val,
    };
    *little_endian = exif_scope.exif.little_endian();
    ErrorCodes::Ok
}

#[no_mangle]
pub extern "C" fn EXIF_get_some_string(data: ExifData, string: *mut *const c_char) -> ErrorCodes {
    let exif_scope = match data.to_exif_scope_mut() {
        Err(error_code) => return error_code,
        Ok(val) => val,
    };

    unsafe {
        *string = match exif_scope.add_string("Hello World") {
            Err(error_code) => return error_code,
            Ok(val) => val as *const c_char,
        };
    }
    ErrorCodes::Ok
}
