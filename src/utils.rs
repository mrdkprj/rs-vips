// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use crate::bindings::{
    self, g_log, g_type_from_name, vips_error_buffer, GLogLevelFlags_G_LOG_LEVEL_WARNING,
};
use crate::{error::Error, Image, Result, VipsImage};
use std::{ffi::CString, path::Path, sync::Arc};

pub(crate) fn vips_image_result(out: *mut bindings::VipsImage, err: Error) -> Result<VipsImage> {
    if out.is_null() {
        Err(err.extend())
    } else {
        Ok(
            new_vipsimage(
                out, None, None,
            ),
        )
    }
}

pub(crate) fn vips_image_result_ext(out: VipsImage, err: Error) -> Result<VipsImage> {
    if out
        .image
        .ctx
        .is_null()
    {
        Err(err.extend())
    } else {
        Ok(out)
    }
}

#[inline]
pub(crate) fn result<T>(res: i32, output: T, error: Error) -> Result<T> {
    if res == 0 {
        Ok(output)
    } else {
        Err(error.extend())
    }
}

#[inline]
pub(crate) fn result_cond<T>(cond: bool, output: T, error: Error) -> Result<T> {
    if cond {
        Ok(output)
    } else {
        Err(error.extend())
    }
}

#[inline]
pub(crate) fn safe_result<F, O>(res: i32, func: F, error: Error) -> Result<O>
where
    F: FnOnce() -> O,
{
    if res == 0 {
        Ok(func())
    } else {
        Err(error.extend())
    }
}

#[inline]
pub(crate) fn safe_result_cond<F, O>(cond: bool, func: F, error: Error) -> Result<O>
where
    F: FnOnce() -> O,
{
    if cond {
        Ok(func())
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
pub(crate) fn new_c_string_from_raw(ptr: *mut i8) -> CString {
    unsafe { CString::from_raw(ptr) }
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

#[allow(clippy::arc_with_non_send_sync)]
pub(crate) fn new_vipsimage(
    ctx: *mut bindings::VipsImage,
    buffer: Option<Arc<[u8]>>,
    refs: Option<Vec<Arc<Image>>>,
) -> VipsImage {
    VipsImage {
        image: Arc::new(Image {
            ctx,
            buffer,
            refs: refs.unwrap_or_default(),
        }),
    }
}

#[inline]
pub(crate) unsafe fn new_int_array(array: *mut i32, size: u64) -> Vec<i32> {
    std::slice::from_raw_parts(
        array,
        size as usize,
    )
    .to_vec()
}

#[inline]
pub(crate) unsafe fn new_double_array(array: *mut f64, size: u64) -> Vec<f64> {
    std::slice::from_raw_parts(
        array,
        size as usize,
    )
    .to_vec()
}

pub(crate) const G_TYPE_BOOLEAN: &str = "gboolean";
pub(crate) const G_TYPE_INT: &str = "gint";
pub(crate) const G_TYPE_UINT64: &str = "guint64";
pub(crate) const G_TYPE_DOUBLE: &str = "gdouble";
pub(crate) const G_TYPE_STRING: &str = "gchararray";

pub(crate) fn get_g_type(name: &str) -> Result<usize> {
    let type_name = new_c_string(name)?;
    Ok(unsafe { g_type_from_name(type_name.as_ptr()) })
}

pub(crate) fn g_warning() -> Result<()> {
    let domain = new_c_string("GLib-GObject")?;
    let format = new_c_string("%s")?;
    unsafe {
        g_log(
            domain.as_ptr(),
            GLogLevelFlags_G_LOG_LEVEL_WARNING,
            format.as_ptr(),
            vips_error_buffer(),
        )
    };
    Ok(())
}
