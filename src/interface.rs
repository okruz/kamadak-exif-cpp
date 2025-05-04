/*
 * ########################################################################
 * The contents of this file is free and unencumbered software released into the
 * public domain. For more information, please refer to <http://unlicense.org/>
 * ########################################################################
 */

use exif::Exif;
use std::ffi::{c_char, CString};
use std::os::raw::c_void;

// ######################################################################################
// ######################################################################################
// ##################################### ExifScope ######################################

/// Internal memory management struct. Couples the lifetimes of the returned CStrings to that of the parsed Exif data
/// to make sure the string data are freed together with the Exif data.
pub(crate) struct ExifScope {
    /// The parsed exif data (struct from the kadamak crate).
    pub(crate) exif: Exif,
    /// The CStrings returned as "*const char" over the ffi interface. Once this struct is dropped,
    /// the CStrings will be dropped as well.
    cstrings: Vec<CString>,
    key_value_pairs: Vec<KeyValuePair>,
}

impl ExifScope {
    /// Converts a &str to a CString and adds it to the ExifScope, such that it will be dropped,
    /// once the ExifScope is dropped. Returns the contents of the CString as a null-terminated
    /// "*const char"-pointer.
    pub(crate) fn add_string(&mut self, data: &str) -> Result<*const c_char, ErrorCodes> {
        self.cstrings
            .push(CString::new(data).map_err(|_| ErrorCodes::UnknownError)?);
        Ok(self
            .cstrings
            .last()
            .ok_or(ErrorCodes::UnknownError)?
            .as_bytes_with_nul()
            .as_ptr() as *const c_char)
    }

    pub(crate) fn add_key_value_pair(&mut self, key: &str, value: &str) -> Result<(), ErrorCodes> {
        let key_value_pair = KeyValuePair {
            key: self.add_string(key)?,
            value: self.add_string(value)?,
        };
        self.key_value_pairs.push(key_value_pair);
        Ok(())
    }

    pub(crate) fn retrieve_data(
        &mut self,
        key_value_pairs: *mut *const KeyValuePair,
        num_elements: &mut usize,
    ) -> Result<(), ErrorCodes> {
        // Remove previously held entries.
        self.key_value_pairs.clear();
        let mut tmp_key_value_pairs: Vec<(String, String)> = vec![];

        for f in self.exif.fields() {
            let key = format!("{}.{}", f.ifd_num.index(), f.tag);
            let value = format!("{}", f.display_value().with_unit(&self.exif));
            tmp_key_value_pairs.push((key, value));
        }

        for (key, value) in tmp_key_value_pairs.iter() {
            self.add_key_value_pair(&key, &value)?;
        }

        *num_elements = self.key_value_pairs.len();
        unsafe { *key_value_pairs = self.key_value_pairs.as_ptr() };
        Ok(())
    }
}

// ##################################### ExifScope ######################################
// ######################################################################################
// ###################################### ExifData ######################################

/// The opaque struct returned over the ffi interface. The C/C++ side shall not be concerned with the internal representation anyway.
///
/// Note: The struct is NOT thread-safe and may not be used concurrently without proper synchronisation.
#[repr(C)]
pub struct ExifData {
    val: *mut c_void,
}

impl ExifData {
    /// Constructs an empty struct.
    pub(crate) fn make_null() -> Self {
        ExifData {
            val: std::ptr::null_mut(),
        }
    }

    // Checks whether self holds any data.
    pub(crate) fn is_null(&self) -> bool {
        self.val.is_null()
    }

    /// Constructs an ExifData object from successfully parsed exif data.
    pub(crate) fn from_exif(exif: Exif) -> Self {
        // Move exif onto the heap and make the box give up ownership to extend the lifetime until drop_explicitly() is called.
        let exif_scope_ptr: *mut ExifScope = Box::into_raw(Box::new(ExifScope {
            exif: exif,
            cstrings: vec![],
            key_value_pairs: vec![],
        }));
        Self {
            val: exif_scope_ptr as *mut c_void,
        }
    }

    /// Casts the val pointer to a shared reference to an ExifScope.
    pub(crate) fn to_exif_scope(self) -> Result<&'static ExifScope, ErrorCodes> {
        if self.is_null() {
            return Err(ErrorCodes::Nullptr);
        }

        // Is safe as long as there is no concurrent use of a mutable reference to the ExifScope,
        // self was created via from_exif and has not been dropped explicitly.
        // It is upon the user of the ffi interface to make sure these assumptions hold true.
        Ok(unsafe { &*(self.val as *const ExifScope) })
    }

    /// Casts the val pointer to a mutable reference to an ExifScope.
    pub(crate) fn to_exif_scope_mut(self) -> Result<&'static mut ExifScope, ErrorCodes> {
        if self.is_null() {
            return Err(ErrorCodes::Nullptr);
        }

        // Is safe as long as self is not used concurrently, it was created via from_exif and has not been dropped explicitly.
        // It is upon the user of the ffi interface to make sure these assumptions hold true.
        Ok(unsafe { &mut *(self.val as *mut ExifScope) })
    }

    /// Drops and deallocates the data pointed to to by self.val.
    pub(crate) fn drop_explicitly(self) -> ErrorCodes {
        if self.is_null() {
            return ErrorCodes::Nullptr;
        }

        // Once the box goes out of scope, the ExifScope struct on the heap will be dropped and deallocated as well.
        let _box_to_drop = unsafe { Box::from_raw(self.val as *mut ExifScope) };
        ErrorCodes::Ok
    }
}

// ###################################### ExifData ######################################
// ######################################################################################
// ################################## ExifParseResult ###################################

#[repr(C)]
pub struct ExifParseResult {
    /// Opaque pointer to the parsed data to be returned via the ffi interface.
    pub(crate) data: ExifData,
    /// Indicates whether the parsing was successful.
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

// ###################################### ExifData ######################################
// ######################################################################################
// ##################################### ErrorCodes #####################################

/// The error codes returned via the ffi interface.
///
/// Used to communicate errors to the C/C++ side.
#[repr(C)]
pub enum ErrorCodes {
    Ok,
    Nullptr,
    ParseError,
    UnknownError,
}

// ##################################### ErrorCodes #####################################
// ######################################################################################
// ###################################### ResultMap #####################################

#[repr(C)]
pub struct KeyValuePair {
    pub key: *const c_char,
    pub value: *const c_char,
}
