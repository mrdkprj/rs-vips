// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use crate::bindings::{self, _VipsBlob, vips_area_unref};

#[derive(Debug, Clone)]
pub struct VipsBlob {
    pub(crate) ctx: *mut bindings::VipsBlob,
}

impl Drop for VipsBlob {
    fn drop(&mut self) {
        unsafe {
            if !self
                .ctx
                .is_null()
            {
                bindings::g_object_unref(self.ctx as _);
            }
        }
    }
}

impl From<*mut bindings::VipsBlob> for VipsBlob {
    fn from(value: *mut bindings::VipsBlob) -> Self {
        Self {
            ctx: value,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<u8>> for VipsBlob {
    fn into(self) -> Vec<u8> {
        unsafe {
            if self
                .ctx
                .is_null()
            {
                return Vec::new();
            }

            let mut size: u64 = 0;
            let bytes = bindings::vips_blob_get(
                self.ctx,
                &mut size,
            );
            // unref area
            unref_area(self.ctx);
            Vec::from_raw_parts(
                bytes as *mut u8,
                size as usize,
                size as usize,
            )
        }
    }
}

pub(crate) fn unref_area(blob: *mut _VipsBlob) {
    unsafe {
        (*blob)
            .area
            .free_fn = None;
        vips_area_unref(&mut (*blob).area);
    }
}
