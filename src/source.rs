// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use crate::{
    bindings::{self, gint64},
    connection::VipsConnection,
    error::Error,
    utils::{self, new_c_string, path_to_cstring, result_cond, safe_result_cond},
    Result,
};
use std::{
    ffi::{c_int, c_void},
    io::{Read, Seek, SeekFrom},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct VipsSource {
    pub(crate) ctx: *mut bindings::VipsSource,
}

impl VipsSource {
    /// Create an source attached to a file descriptor. descriptor is closed with close() when source is finalized.
    pub fn new_from_descriptor(descriptor: i32) -> Result<VipsSource> {
        unsafe {
            let res = bindings::vips_source_new_from_descriptor(descriptor);
            result_cond(
                !res.is_null(),
                VipsSource {
                    ctx: res,
                },
                Error::InitializationError(
                    "Could not initialise VipsSource from descriptor".to_string(),
                ),
            )
        }
    }

    /// Create a source attached to a file.
    pub fn new_from_file<P: AsRef<Path>>(filename: P) -> Result<VipsSource> {
        unsafe {
            let filename_c_str = path_to_cstring(filename)?;
            let res = bindings::vips_source_new_from_file(filename_c_str.as_ptr());
            result_cond(
                !res.is_null(),
                VipsSource {
                    ctx: res,
                },
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
            result_cond(
                !res.is_null(),
                VipsSource {
                    ctx: res,
                },
                Error::InitializationError(
                    "Could not initialise VipsSource from memory".to_string(),
                ),
            )
        }
    }

    /// Create a source from an option string.
    pub fn new_from_options(option_str: &str) -> Result<VipsSource> {
        unsafe {
            let options = new_c_string(option_str)?;
            let res = bindings::vips_source_new_from_options(options.as_ptr());
            result_cond(
                !res.is_null(),
                VipsSource {
                    ctx: res,
                },
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
            result_cond(
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
            safe_result_cond(
                !result.is_null(),
                || {
                    std::slice::from_raw_parts(
                        result as *const u8,
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
            result_cond(
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
            result_cond(
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
            result_cond(
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

pub trait Reader: Read + Seek + Send + 'static {}
impl<T: Read + Seek + Send + 'static> Reader for T {}

struct SourceContext {
    reader: Box<dyn Reader>,
}

// VipsSourceCustom
impl VipsSource {
    pub fn new_from_reader<R: Reader>(input: R) -> Result<VipsSource> {
        unsafe {
            let source = bindings::vips_source_custom_new();

            if source.is_null() {
                return Err(
                    Error::InitializationError("Cannot create VipsSourceCustom".to_string()),
                );
            }

            let context = Box::new(
                SourceContext {
                    reader: Box::new(input),
                },
            );

            let user_data = Box::into_raw(context) as *mut c_void;

            bindings::g_signal_connect_data(
                source as *mut c_void,
                c"read".as_ptr(),
                Some(
                    std::mem::transmute::<
                        unsafe extern "C" fn(
                            source: *mut bindings::VipsSourceCustom,
                            buffer: *mut c_void,
                            length: gint64,
                            user_data: *mut c_void,
                        ) -> gint64,
                        unsafe extern "C" fn(),
                    >(on_read),
                ),
                user_data,
                Some(source_destroy),
                0,
            );

            bindings::g_signal_connect_data(
                source as *mut c_void,
                c"seek".as_ptr(),
                Some(
                    std::mem::transmute::<
                        unsafe extern "C" fn(
                            source: *mut bindings::VipsSourceCustom,
                            offset: gint64,
                            whence: c_int,
                            user_data: *mut c_void,
                        ) -> gint64,
                        unsafe extern "C" fn(),
                    >(on_seek),
                ),
                user_data,
                None,
                0,
            );

            Ok(
                VipsSource {
                    ctx: source as *mut bindings::VipsSource,
                },
            )
        }
    }
}

unsafe extern "C" fn on_read(
    _: *mut bindings::VipsSourceCustom,
    buffer: *mut c_void,
    length: gint64,
    user_data: *mut c_void,
) -> gint64 {
    let ctx = &mut *(user_data as *mut SourceContext);

    let slice = std::slice::from_raw_parts_mut(
        buffer as *mut u8,
        length as usize,
    );

    match ctx
        .reader
        .read(slice)
    {
        Ok(n) => n as gint64,
        Err(_) => -1,
    }
}

unsafe extern "C" fn on_seek(
    _: *mut bindings::VipsSourceCustom,
    offset: gint64,
    whence: c_int,
    user_data: *mut c_void,
) -> gint64 {
    let ctx = &mut *(user_data as *mut SourceContext);

    let pos = match whence {
        0 => SeekFrom::Start(offset as u64),
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return -1,
    };

    match ctx
        .reader
        .seek(pos)
    {
        Ok(n) => n as gint64,
        Err(_) => -1,
    }
}

unsafe extern "C" fn source_destroy(data: *mut c_void, _: *mut bindings::_GClosure) {
    let _ = Box::from_raw(data as *mut SourceContext);
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
