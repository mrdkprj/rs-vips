use crate::{bindings, error::Error, utils, Result};
use std::ffi::*;

#[derive(Debug, Clone)]
pub struct VipsInterpolate {
    pub(crate) ctx: *mut bindings::VipsInterpolate,
}

impl Default for VipsInterpolate {
    fn default() -> VipsInterpolate {
        unsafe {
            VipsInterpolate {
                ctx: bindings::vips_interpolate_nearest_static(),
            }
        }
    }
}

impl VipsInterpolate {
    /// defaults to vips_interpolate_nearest_static
    pub fn new() -> VipsInterpolate {
        unsafe {
            VipsInterpolate {
                ctx: bindings::vips_interpolate_nearest_static(),
            }
        }
    }

    /// A convenience function that returns a nearest-neighbour interpolator you don’t need to free.
    pub fn new_from_neasest_static() -> VipsInterpolate {
        unsafe {
            VipsInterpolate {
                ctx: bindings::vips_interpolate_nearest_static(),
            }
        }
    }

    /// A convenience function that returns a bilinear interpolator you don’t need to free.
    pub fn new_from_bilinear_static() -> VipsInterpolate {
        unsafe {
            VipsInterpolate {
                ctx: bindings::vips_interpolate_bilinear_static(),
            }
        }
    }

    /// Look up an interpolator from a nickname and make one.
    pub fn new_from_name(name: &str) -> Result<VipsInterpolate> {
        unsafe {
            let nickname = utils::new_c_string(name)?;
            let res = bindings::vips_interpolate_new(nickname.as_ptr());
            if res.is_null() {
                Err(
                    Error::InitializationError(
                        "Cannot initialize interpolator with provided nickname".to_string(),
                    ),
                )
            } else {
                Ok(
                    VipsInterpolate {
                        ctx: res,
                    },
                )
            }
        }
    }

    /// Look up an interpolators desired window size.
    pub fn get_window_size(&self) -> i32 {
        unsafe { bindings::vips_interpolate_get_window_size(self.ctx) }
    }

    /// Look up an interpolators desired window offset.
    pub fn get_windows_offset(&self) -> i32 {
        unsafe { bindings::vips_interpolate_get_window_offset(self.ctx) }
    }
}

impl Drop for VipsInterpolate {
    fn drop(&mut self) {
        unsafe {
            if !self
                .ctx
                .is_null()
            {
                bindings::g_object_unref(self.ctx as *mut c_void);
            }
        }
    }
}

impl From<*mut bindings::VipsInterpolate> for VipsInterpolate {
    fn from(value: *mut bindings::VipsInterpolate) -> Self {
        Self {
            ctx: value,
        }
    }
}
