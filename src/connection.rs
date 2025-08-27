use crate::{
    bindings,
    error::Error,
    region::VipsBlob,
    utils::{self, vips_source_result, vips_target_result},
    Result,
};
use std::{
    borrow::Cow,
    ffi::{c_void, CStr, CString},
    ptr::null_mut,
};

#[derive(Debug, Clone)]
pub struct VipsConnection {
    pub(crate) ctx: *mut bindings::VipsConnection,
}

#[derive(Debug, Clone)]
pub struct VipsSource {
    pub(crate) ctx: *mut bindings::VipsSource,
}

#[derive(Debug, Clone)]
pub struct VipsTarget {
    pub(crate) ctx: *mut bindings::VipsTarget,
}

impl VipsConnection {
    pub fn connection_filename(&self) -> Option<String> {
        unsafe {
            let result = bindings::vips_connection_filename(self.ctx);
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

    pub fn connection_nick(&self) -> Option<String> {
        unsafe {
            let result = bindings::vips_connection_nick(self.ctx);
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
    pub fn new_from_descriptor(descriptor: i32) -> Result<Self> {
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
    pub fn new_from_file(filename: &str) -> Result<Self> {
        unsafe {
            let f = utils::new_c_string(filename)?;
            let res = bindings::vips_source_new_from_file(f.as_ptr());
            vips_source_result(
                res,
                Error::InitializationError("Could not initialise VipsSource from file".to_string()),
            )
        }
    }

    // Create a source attached to an area of memory.
    // not sure if it this is safe
    // should test before making it public
    fn new_from_blob(blob: VipsBlob) -> Result<Self> {
        unsafe {
            let res = bindings::vips_source_new_from_blob(blob.ctx);
            vips_source_result(
                res,
                Error::InitializationError("Could not initialise VipsSource from blob".to_string()),
            )
        }
    }

    /// Create a source attached to an area of memory. You must not free data while the source is active.
    pub fn new_from_memory(buffer: &[u8]) -> Result<Self> {
        unsafe {
            let res = bindings::vips_source_new_from_memory(
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
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
    pub fn new_from_options(option_str: &str) -> Result<Self> {
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
            let bytes: *mut c_void = null_mut();
            let result = bindings::vips_source_read(
                self.ctx,
                bytes,
                length,
            );
            if result != -1 {
                let buffer = Vec::from_raw_parts(
                    bytes as *mut u8,
                    result as usize,
                    result as usize,
                );
                Ok(buffer)
            } else {
                Err(Error::OperationError("Error on vips read".to_string()))
            }
        }
    }

    /// Whether the source can be efficiently mapped into memory.
    pub fn is_mappable(&self) -> bool {
        unsafe { bindings::vips_source_is_mappable(self.ctx) == 1 }
    }

    /// Move the file read position.
    pub fn seek(&mut self, offset: i64, whence: i32) -> Result<i64> {
        unsafe {
            let result = bindings::vips_source_seek(
                self.ctx,
                offset,
                whence,
            );
            if result == -1 {
                Err(Error::OperationError("Error on vips seek".to_string()))
            } else {
                Ok(result)
            }
        }
    }

    /// Rewind the source to the start.
    pub fn rewind(&mut self) -> Result<()> {
        unsafe {
            let result = bindings::vips_source_rewind(self.ctx);
            if result == -1 {
                Err(Error::OperationError("Error on vips rewind".to_string()))
            } else {
                Ok(())
            }
        }
    }

    /// Return the length in bytes of the source.
    pub fn length(&self) -> Result<i64> {
        unsafe {
            let result = bindings::vips_source_length(self.ctx);
            if result == -1 {
                Err(Error::OperationError("Error on vips length".to_string()))
            } else {
                Ok(result)
            }
        }
    }
}

impl<'a> VipsSource {
    /// Map the source entirely into memory and return a pointer to the start.
    pub fn map(&'a self) -> Result<&'a [u8]> {
        unsafe {
            let length: *mut u64 = null_mut();
            let result = bindings::vips_source_map(
                self.ctx,
                length,
            );
            if length.is_null() {
                Err(Error::OperationError("Error on vips map".to_string()))
            } else {
                let size = (*length)
                    .try_into()
                    .map_err(|_| Error::OperationError("Can't get size of array".to_string()))?;
                Ok(
                    std::slice::from_raw_parts(
                        result as *mut u8,
                        size,
                    ),
                )
            }
        }
    }
}

impl VipsTarget {
    /// Create a target attached to a file descriptor. descriptor is kept open until the target is finalized.
    pub fn new_to_descriptor(descriptor: i32) -> Result<Self> {
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
    pub fn new_to_file(filename: &str) -> Result<Self> {
        unsafe {
            let f = utils::new_c_string(filename)?;
            let res = bindings::vips_target_new_to_file(f.as_ptr());
            vips_target_result(
                res,
                Error::InitializationError("Could not initialise VipsTarget from file".to_string()),
            )
        }
    }

    /// Create a target which will write to a memory area. Read from blob to get memory.
    pub fn new_to_memory() -> Result<Self> {
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

    /// Call this at the end of write to make the target do any cleaning up.
    pub fn end(self) {
        unsafe {
            bindings::vips_target_end(self.ctx);
        }
    }

    /// Write a single character ch to target.
    pub fn putc(&mut self, ch: char) -> Result<()> {
        unsafe {
            let res = bindings::vips_target_putc(
                self.ctx,
                ch as i32,
            );
            if res == -1 {
                Err(Error::OperationError("Could not write to buffer".to_string()))
            } else {
                Ok(())
            }
        }
    }

    /// Write buffer to the output.
    pub fn write(&mut self, buffer: &[u8]) -> Result<()> {
        unsafe {
            let res = bindings::vips_target_write(
                self.ctx,
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
            );
            if res == -1 {
                Err(Error::OperationError("Could not write to buffer".to_string()))
            } else {
                Ok(())
            }
        }
    }

    /// Write str to target.
    pub fn write_amp(&mut self, text: &str) -> Result<()> {
        unsafe {
            let cstr = CString::new(text)
                .map_err(|_| Error::OperationError("Cannot initialize C string".to_string()))?;
            let res = bindings::vips_target_write_amp(
                self.ctx,
                cstr.as_ptr(),
            );
            if res == -1 {
                Err(Error::OperationError("Could not write to buffer".to_string()))
            } else {
                Ok(())
            }
        }
    }

    /// Write a null-terminated string to target.
    pub fn writes(&mut self, text: &str) -> Result<()> {
        unsafe {
            let cstr = CString::new(text)
                .map_err(|_| Error::OperationError("Cannot initialize C string".to_string()))?;
            let res = bindings::vips_target_writes(
                self.ctx,
                cstr.as_ptr(),
            );
            if res == -1 {
                Err(Error::OperationError("Could not write to buffer".to_string()))
            } else {
                Ok(())
            }
        }
    }

    pub fn get_blob(&self) -> VipsBlob {
        unsafe {
            VipsBlob {
                ctx: (*self.ctx).blob,
            }
        }
    }
}

impl Drop for VipsConnection {
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

impl From<*mut bindings::VipsSource> for VipsSource {
    fn from(value: *mut bindings::VipsSource) -> Self {
        Self {
            ctx: value,
        }
    }
}

impl From<*mut bindings::VipsTarget> for VipsTarget {
    fn from(value: *mut bindings::VipsTarget) -> Self {
        Self {
            ctx: value,
        }
    }
}
