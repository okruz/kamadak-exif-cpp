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
use std::io::Cursor;

#[no_mangle]
pub extern "C" fn EXIF_load_exif(data: *const u8, length: usize) -> ExifParseResult {
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

#[no_mangle]
pub extern "C" fn EXIF_free_exif(data: ExifData) -> ErrorCodes {
    data.drop_explicitly()
}

#[no_mangle]
pub extern "C" fn EXIF_is_little_endian(data: ExifData, little_endian: &mut bool) -> ErrorCodes {
    let exif = match data.to_exif() {
        Err(error_code) => return error_code,
        Ok(val) => val,
    };
    *little_endian = exif.little_endian();
    ErrorCodes::Ok
}
