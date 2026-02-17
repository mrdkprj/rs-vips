// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use crate::bindings::{self, g_type_from_name, VipsArrayDouble, VipsArrayImage, VipsArrayInt};
use crate::error::Error;
use crate::VipsImage;
use crate::{
    connection::{VipsSource, VipsTarget},
    Result,
};
use std::{
    ffi::{c_void, CString},
    path::Path,
};

pub(crate) struct VipsArrayIntWrapper {
    pub ctx: *mut VipsArrayInt,
}
pub(crate) struct VipsArrayDoubleWrapper {
    pub ctx: *mut VipsArrayDouble,
}
pub(crate) struct VipsArrayImageWrapper {
    pub ctx: *mut VipsArrayImage,
}

impl Drop for VipsArrayIntWrapper {
    fn drop(&mut self) {
        unsafe {
            bindings::vips_area_unref(self.ctx as *mut bindings::VipsArea);
        }
    }
}

impl Drop for VipsArrayDoubleWrapper {
    fn drop(&mut self) {
        unsafe {
            bindings::vips_area_unref(self.ctx as *mut bindings::VipsArea);
        }
    }
}

impl Drop for VipsArrayImageWrapper {
    fn drop(&mut self) {
        unsafe {
            bindings::vips_area_unref(self.ctx as *mut bindings::VipsArea);
        }
    }
}

impl From<&[i32]> for VipsArrayIntWrapper {
    #[inline]
    fn from(array: &[i32]) -> Self {
        VipsArrayIntWrapper {
            ctx: unsafe {
                bindings::vips_array_int_new(
                    array.as_ptr(),
                    array.len() as i32,
                )
            },
        }
    }
}

impl From<&[f64]> for VipsArrayDoubleWrapper {
    #[inline]
    fn from(array: &[f64]) -> Self {
        VipsArrayDoubleWrapper {
            ctx: unsafe {
                bindings::vips_array_double_new(
                    array.as_ptr(),
                    array.len() as i32,
                )
            },
        }
    }
}

impl From<&[VipsImage]> for VipsArrayImageWrapper {
    #[inline]
    fn from(array: &[VipsImage]) -> Self {
        let len = array.len() as i32;
        let mut images = array
            .iter()
            .map(|v| v.ctx)
            .collect::<Vec<_>>();
        VipsArrayImageWrapper {
            ctx: unsafe {
                bindings::vips_array_image_new(
                    images.as_mut_ptr(),
                    len,
                )
            },
        }
    }
}

pub(crate) fn vips_image_result(res: *mut bindings::VipsImage, err: Error) -> Result<VipsImage> {
    if res.is_null() {
        Err(err.extend())
    } else {
        Ok(
            VipsImage {
                ctx: res,
            },
        )
    }
}

pub(crate) fn vips_image_result_ext(res: VipsImage, err: Error) -> Result<VipsImage> {
    if res
        .ctx
        .is_null()
    {
        Err(err.extend())
    } else {
        Ok(res)
    }
}

pub(crate) fn vips_source_result(res: *mut bindings::VipsSource, err: Error) -> Result<VipsSource> {
    if res.is_null() {
        Err(err.extend())
    } else {
        Ok(
            VipsSource {
                ctx: res,
            },
        )
    }
}

pub(crate) fn vips_target_result(res: *mut bindings::VipsTarget, err: Error) -> Result<VipsTarget> {
    if res.is_null() {
        Err(err.extend())
    } else {
        Ok(
            VipsTarget {
                ctx: res,
            },
        )
    }
}

#[inline]
pub fn result<T>(res: i32, output: T, error: Error) -> Result<T> {
    if res == 0 {
        Ok(output)
    } else {
        Err(error.extend())
    }
}

#[inline]
pub(crate) fn safe_result<F, O, R>(res: i32, output: O, func: F, error: Error) -> Result<R>
where
    F: Fn(O) -> R,
{
    if res == 0 {
        Ok(func(
            output,
        ))
    } else {
        Err(error.extend())
    }
}

#[inline]
pub(crate) fn new_c_string(string: impl Into<Vec<u8>>) -> Result<CString> {
    CString::new(string)
        .map_err(|_| Error::InitializationError("Error initializing C string.".to_string()))
}

#[inline]
pub(crate) fn path_to_cstring<P: AsRef<Path>>(path: P) -> Result<CString> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        CString::new(
            path.as_ref()
                .as_os_str()
                .as_bytes(),
        )
        .map_err(|_| Error::InitializationError("Error initializing C string.".to_string()))
    }

    #[cfg(windows)]
    {
        CString::new(
            path.as_ref()
                .to_string_lossy()
                .as_bytes(),
        )
        .map_err(|_| Error::InitializationError("Error initializing C string.".to_string()))
    }
}

#[inline]
pub(crate) fn ensure_null_terminated(input: impl AsRef<[u8]>) -> crate::Result<CString> {
    let bytes = input.as_ref();

    // Check if already null-terminated
    if bytes.last() == Some(&0) {
        CString::new(&bytes[..bytes.len() - 1])
            .map_err(|_| Error::InitializationError("Error initializing C string.".to_string()))
    } else {
        // Not null-terminated, append 0 and create CString
        CString::new(bytes)
            .map_err(|_| Error::InitializationError("Error initializing C string.".to_string()))
    }
}

#[inline]
pub(crate) unsafe fn new_byte_array(buf: *mut c_void, size: u64) -> Vec<u8> {
    Vec::from_raw_parts(
        buf as *mut u8,
        size as usize,
        size as usize,
    )
}

#[inline]
pub(crate) unsafe fn new_int_array(array: *mut i32, size: u64) -> Vec<i32> {
    Vec::from(
        std::slice::from_raw_parts(
            array,
            size as usize,
        ),
    )
}

#[inline]
pub(crate) unsafe fn new_double_array(array: *mut f64, size: u64) -> Vec<f64> {
    Vec::from(
        std::slice::from_raw_parts(
            array,
            size as usize,
        ),
    )
}

pub(crate) const G_TYPE_BOOLEAN: &str = "gboolean";
pub(crate) const G_TYPE_INT: &str = "gint";
pub(crate) const G_TYPE_UINT64: &str = "guint64";
pub(crate) const G_TYPE_DOUBLE: &str = "gdouble";
pub(crate) const G_TYPE_STRING: &str = "gchararray";

pub(crate) fn get_g_type(name: &str) -> Result<u64> {
    let type_name = new_c_string(name)?;
    Ok(unsafe { g_type_from_name(type_name.as_ptr()) })
}
