// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use crate::bindings::{self, vips_connection_filename, vips_connection_nick};
use std::{borrow::Cow, ffi::CStr};

pub(crate) struct VipsConnection;

impl VipsConnection {
    pub(crate) fn filename(ctx: *mut bindings::VipsConnection) -> Option<String> {
        unsafe {
            let result = vips_connection_filename(ctx);
            if result.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(result);
                match cstr.to_string_lossy() {
                    Cow::Borrowed(slice) => Some(slice.to_string()),
                    Cow::Owned(string) => Some(string),
                }
            }
        }
    }

    pub(crate) fn nick(ctx: *mut bindings::VipsConnection) -> Option<String> {
        unsafe {
            let result = vips_connection_nick(ctx);
            if result.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(result);
                match cstr.to_string_lossy() {
                    Cow::Borrowed(slice) => Some(slice.to_string()),
                    Cow::Owned(string) => Some(string),
                }
            }
        }
    }
}
