// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use crate::{
    bindings,
    error::Error,
    utils::{self, vips_source_result, vips_target_result},
    Result,
};
use std::{
    borrow::Cow,
    ffi::{c_int, c_void, CStr},
    path::Path,
};

struct VipsConnection;

#[derive(Debug, Clone)]
pub struct VipsSource {
    pub(crate) ctx: *mut bindings::VipsSource,
}

#[derive(Debug, Clone)]
pub struct VipsTarget {
    pub(crate) ctx: *mut bindings::VipsTarget,
}

impl VipsConnection {
    fn filename(ctx: *mut bindings::VipsConnection) -> Option<String> {
        unsafe {
            let result = bindings::vips_connection_filename(ctx);
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

    fn nick(ctx: *mut bindings::VipsConnection) -> Option<String> {
        unsafe {
            let result = bindings::vips_connection_nick(ctx);
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

impl VipsSource {
    /// Create an source attached to a file descriptor. descriptor is closed with close() when source is finalized.
    pub fn new_from_descriptor(descriptor: i32) -> Result<VipsSource> {
        unsafe {
            let res = bindings::vips_source_new_from_descriptor(descriptor);
            vips_source_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsSource from descriptor".to_string(),
                ),
            )
        }
    }

    /// Create a source attached to a file.
    pub fn new_from_file<P: AsRef<Path>>(filename: P) -> Result<VipsSource> {
        unsafe {
            let filename_c_str = utils::path_to_cstring(filename)?;
            let res = bindings::vips_source_new_from_file(filename_c_str.as_ptr());
            vips_source_result(
                res,
                Error::InitializationError("Could not initialise VipsSource from file".to_string()),
            )
        }
    }

    /// Create a source attached to an area of memory. You must not free data while the source is active.
    pub fn new_from_memory(buffer: &[u8]) -> Result<VipsSource> {
        unsafe {
            let res = bindings::vips_source_new_from_memory(
                buffer.as_ptr() as *const c_void,
                buffer.len(),
            );
            vips_source_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsSource from memory".to_string(),
                ),
            )
        }
    }

    /// Create a source from an option string.
    pub fn new_from_options(option_str: &str) -> Result<VipsSource> {
        unsafe {
            let options = utils::new_c_string(option_str)?;
            let res = bindings::vips_source_new_from_options(options.as_ptr());
            vips_source_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsSource from options".to_string(),
                ),
            )
        }
    }

    /// Minimise the source. As many resources as can be safely removed are removed.
    pub fn minimise(&mut self) {
        unsafe {
            bindings::vips_source_minimise(self.ctx);
        }
    }

    /// Restore the source after minimisation. This is called at the start of every source method, so loaders should not usually need this.
    pub fn unminimise(&mut self) -> Result<()> {
        unsafe {
            let result = bindings::vips_source_unminimise(self.ctx);
            utils::result(
                result,
                (),
                Error::OperationError("Error on vips unminimise".to_string()),
            )
        }
    }

    /// Signal the end of header read and the start of the pixel decode phase. After this, you can no longer seek on this source.
    pub fn decode(&mut self) -> Result<()> {
        unsafe {
            let result = bindings::vips_source_decode(self.ctx);
            utils::result(
                result,
                (),
                Error::OperationError("Error on vips decode".to_string()),
            )
        }
    }

    /// Read up to length bytes from source and store the bytes in buffer.
    pub fn read(&mut self, length: u64) -> Result<Vec<u8>> {
        unsafe {
            let mut bytes = vec![0u8; length as usize];
            let result = bindings::vips_source_read(
                self.ctx,
                bytes.as_mut_ptr() as *mut c_void,
                length as usize,
            );
            utils::result_cond(
                result != -1,
                bytes,
                Error::OperationError("Error on vips_source_read".to_string()),
            )
        }
    }

    /// Whether the source can be efficiently mapped into memory.
    pub fn is_mappable(&self) -> bool {
        unsafe { bindings::vips_source_is_mappable(self.ctx) == 1 }
    }

    /// Map the source entirely into memory and return a pointer to the start.
    pub fn map(&self) -> Result<Vec<u8>> {
        unsafe {
            let mut length = 0;
            let result = bindings::vips_source_map(
                self.ctx,
                &mut length,
            );
            utils::safe_result_cond(
                !result.is_null(),
                || {
                    std::slice::from_raw_parts(
                        result as *mut u8,
                        length,
                    )
                    .to_vec()
                },
                Error::OperationError("Error on vips map".to_string()),
            )
        }
    }

    /// Move the file read position.
    pub fn seek(&mut self, offset: i64, whence: i32) -> Result<i64> {
        unsafe {
            let result = bindings::vips_source_seek(
                self.ctx,
                offset,
                whence,
            );
            utils::result_cond(
                result != -1,
                result,
                Error::OperationError("Error on vips seek".to_string()),
            )
        }
    }

    /// Rewind the source to the start.
    pub fn rewind(&mut self) -> Result<()> {
        unsafe {
            let result = bindings::vips_source_rewind(self.ctx);
            utils::result_cond(
                result != -1,
                (),
                Error::OperationError("Error on vips rewind".to_string()),
            )
        }
    }

    /// Return the length in bytes of the source.
    pub fn length(&self) -> Result<i64> {
        unsafe {
            let result = bindings::vips_source_length(self.ctx);
            utils::result_cond(
                result != -1,
                result,
                Error::OperationError("Error on vips length".to_string()),
            )
        }
    }
}

// VipsConnection for VipsSource
impl VipsSource {
    pub fn filename(&self) -> Option<String> {
        unsafe { VipsConnection::filename(&mut (*self.ctx).parent_object) }
    }

    pub fn nick(&self) -> Option<String> {
        unsafe { VipsConnection::nick(&mut (*self.ctx).parent_object) }
    }
}

impl VipsTarget {
    /// Create a target attached to a file descriptor. descriptor is kept open until the target is finalized.
    pub fn new_to_descriptor(descriptor: i32) -> Result<VipsTarget> {
        unsafe {
            let res = bindings::vips_target_new_to_descriptor(descriptor);
            vips_target_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsTarget from descriptor".to_string(),
                ),
            )
        }
    }

    /// Create a target attached to a file.
    pub fn new_to_file<P: AsRef<Path>>(filename: P) -> Result<VipsTarget> {
        unsafe {
            let filename_c_str = utils::path_to_cstring(filename)?;
            let res = bindings::vips_target_new_to_file(filename_c_str.as_ptr());
            vips_target_result(
                res,
                Error::InitializationError("Could not initialise VipsTarget from file".to_string()),
            )
        }
    }

    /// Create a target which will write to a memory area. Read from blob to get memory.
    pub fn new_to_memory() -> Result<VipsTarget> {
        unsafe {
            let res = bindings::vips_target_new_to_memory();
            vips_target_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsTarget from memory".to_string(),
                ),
            )
        }
    }

    /// Write a single character ch to target.
    pub fn putc(&mut self, ch: char) -> Result<()> {
        unsafe {
            let res = bindings::vips_target_putc(
                self.ctx,
                ch as c_int,
            );
            utils::result_cond(
                res != -1,
                (),
                Error::OperationError("Could not write to target".to_string()),
            )
        }
    }

    /// Write buffer to the output.
    pub fn write(&mut self, buffer: &[u8]) -> Result<()> {
        unsafe {
            let res = bindings::vips_target_write(
                self.ctx,
                buffer.as_ptr() as *const c_void,
                buffer.len(),
            );
            utils::result_cond(
                res != -1,
                (),
                Error::OperationError("Could not write to target".to_string()),
            )
        }
    }

    /// Write a string to target.
    pub fn writes(&mut self, text: &str) -> Result<()> {
        unsafe {
            let cstr = utils::new_c_string(text)?;
            let res = bindings::vips_target_writes(
                self.ctx,
                cstr.as_ptr(),
            );
            utils::result_cond(
                res != -1,
                (),
                Error::OperationError("Could not write to target".to_string()),
            )
        }
    }

    /// Write str to target, but escape stuff that xml hates in text.
    pub fn write_amp(&mut self, text: &str) -> Result<()> {
        unsafe {
            let cstr = utils::new_c_string(text)?;
            let res = bindings::vips_target_write_amp(
                self.ctx,
                cstr.as_ptr(),
            );
            utils::result_cond(
                res != -1,
                (),
                Error::OperationError("Could not write to target".to_string()),
            )
        }
    }

    /// Call this at the end of write to make the target do any cleaning up.
    pub fn end(self) {
        unsafe {
            bindings::vips_target_end(self.ctx);
        }
    }

    pub fn get_blob(&self) -> Vec<u8> {
        unsafe {
            if self
                .ctx
                .is_null()
                || (*self.ctx)
                    .blob
                    .is_null()
            {
                return Vec::new();
            }
            let mut size = 0;
            let bytes = bindings::vips_blob_get(
                (*self.ctx).blob,
                &mut size,
            );
            let slice = std::slice::from_raw_parts(
                bytes as *const u8,
                size,
            );
            slice.to_vec()
        }
    }
}

// VipsConnection for VipsTarget
impl VipsTarget {
    pub fn filename(&self) -> Option<String> {
        unsafe { VipsConnection::filename(&mut (*self.ctx).parent_object) }
    }

    pub fn nick(&self) -> Option<String> {
        unsafe { VipsConnection::nick(&mut (*self.ctx).parent_object) }
    }
}

impl Drop for VipsSource {
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

impl Drop for VipsTarget {
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
