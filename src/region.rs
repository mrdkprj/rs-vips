use crate::{
    bindings::{self, free},
    error::Error,
    utils, Result, VipsImage,
};
use std::ffi::c_void;

pub struct VipsRegion {
    pub(crate) ctx: *mut bindings::VipsRegion,
}

impl VipsRegion {
    pub fn new(image: &VipsImage) -> Result<VipsRegion> {
        unsafe {
            let res = bindings::vips_region_new(
                image
                    .image
                    .ctx,
            );
            utils::result_cond(
                !res.is_null(),
                VipsRegion {
                    ctx: res,
                },
                Error::InitializationError("Cannot initialize VipsRegion".to_string()),
            )
        }
    }

    pub fn width(&self) -> i32 {
        unsafe { bindings::vips_region_width(self.ctx) }
    }

    pub fn height(&self) -> i32 {
        unsafe { bindings::vips_region_height(self.ctx) }
    }

    /// Generate an area of pixels and return a copy
    pub fn fetch(&self, left: i32, top: i32, width: i32, height: i32) -> Result<Vec<u8>> {
        unsafe {
            let mut len = 0;
            let ptr = bindings::vips_region_fetch(
                self.ctx,
                left,
                top,
                width,
                height,
                &mut len,
            );
            utils::safe_result_cond(
                !ptr.is_null(),
                || {
                    let buffer = std::slice::from_raw_parts(
                        ptr as *const u8,
                        len,
                    )
                    .to_vec();
                    free(ptr as *mut c_void);
                    buffer
                },
                Error::OperationError("Error on vips_region_fetch".to_string()),
            )
        }
    }
}

impl Drop for VipsRegion {
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
