// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use crate::{
    bindings::{self, gint64},
    connection::VipsConnection,
    error::Error,
    utils::{new_c_string, path_to_cstring, result_cond},
    Result,
};
use std::{
    ffi::{c_int, c_void},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct VipsTarget {
    pub(crate) ctx: *mut bindings::VipsTarget,
}

impl VipsTarget {
    /// Create a target attached to a file descriptor. descriptor is kept open until the target is finalized.
    pub fn new_to_descriptor(descriptor: i32) -> Result<VipsTarget> {
        unsafe {
            let res = bindings::vips_target_new_to_descriptor(descriptor);
            result_cond(
                !res.is_null(),
                VipsTarget {
                    ctx: res,
                },
                Error::InitializationError(
                    "Could not initialise VipsTarget from descriptor".to_string(),
                ),
            )
        }
    }

    /// Create a target attached to a file.
    pub fn new_to_file<P: AsRef<Path>>(filename: P) -> Result<VipsTarget> {
        unsafe {
            let filename_c_str = path_to_cstring(filename)?;
            let res = bindings::vips_target_new_to_file(filename_c_str.as_ptr());
            result_cond(
                !res.is_null(),
                VipsTarget {
                    ctx: res,
                },
                Error::InitializationError("Could not initialise VipsTarget from file".to_string()),
            )
        }
    }

    /// Create a target which will write to a memory area. Read from blob to get memory.
    pub fn new_to_memory() -> Result<VipsTarget> {
        unsafe {
            let res = bindings::vips_target_new_to_memory();
            result_cond(
                !res.is_null(),
                VipsTarget {
                    ctx: res,
                },
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
            result_cond(
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
            result_cond(
                res != -1,
                (),
                Error::OperationError("Could not write to target".to_string()),
            )
        }
    }

    /// Write a string to target.
    pub fn writes(&mut self, text: &str) -> Result<()> {
        unsafe {
            let cstr = new_c_string(text)?;
            let res = bindings::vips_target_writes(
                self.ctx,
                cstr.as_ptr(),
            );
            result_cond(
                res != -1,
                (),
                Error::OperationError("Could not write to target".to_string()),
            )
        }
    }

    /// Write str to target, but escape stuff that xml hates in text.
    pub fn write_amp(&mut self, text: &str) -> Result<()> {
        unsafe {
            let cstr = new_c_string(text)?;
            let res = bindings::vips_target_write_amp(
                self.ctx,
                cstr.as_ptr(),
            );
            result_cond(
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

pub trait Writer: Write + Read + Seek + Send + 'static {}
impl<T: Write + Read + Seek + Send + 'static> Writer for T {}
struct TargetContext {
    writer: Box<dyn Writer>,
}

impl VipsTarget {
    pub fn new_to_writer<W: Writer>(output: W) -> Result<VipsTarget> {
        unsafe {
            let target = bindings::vips_target_custom_new();

            if target.is_null() {
                return Err(
                    Error::InitializationError("Cannot create VipsTargetCustom".to_string()),
                );
            }

            let context = Box::new(
                TargetContext {
                    writer: Box::new(output),
                },
            );

            let user_data = Box::into_raw(context) as *mut c_void;

            bindings::g_signal_connect_data(
                target as *mut c_void,
                c"write".as_ptr(),
                Some(
                    std::mem::transmute::<
                        unsafe extern "C" fn(
                            target: *mut bindings::VipsTargetCustom,
                            data: *const c_void,
                            length: bindings::gint64,
                            user_data: *mut c_void,
                        ) -> bindings::gint64,
                        unsafe extern "C" fn(),
                    >(on_write),
                ),
                user_data,
                None,
                0,
            );

            bindings::g_signal_connect_data(
                target as *mut c_void,
                c"read".as_ptr(),
                Some(
                    std::mem::transmute::<
                        unsafe extern "C" fn(
                            target: *mut bindings::VipsTargetCustom,
                            buffer: *mut c_void,
                            length: gint64,
                            user_data: *mut c_void,
                        ) -> gint64,
                        unsafe extern "C" fn(),
                    >(on_read),
                ),
                user_data,
                None,
                0,
            );

            bindings::g_signal_connect_data(
                target as *mut c_void,
                c"seek".as_ptr(),
                Some(
                    std::mem::transmute::<
                        unsafe extern "C" fn(
                            target: *mut bindings::VipsTargetCustom,
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

            bindings::g_signal_connect_data(
                target as *mut c_void,
                c"end".as_ptr(),
                Some(
                    std::mem::transmute::<
                        unsafe extern "C" fn(
                            target: *mut bindings::VipsTargetCustom,
                            user_data: *mut c_void,
                        ) -> c_int,
                        unsafe extern "C" fn(),
                    >(on_end),
                ),
                user_data,
                None,
                0,
            );

            Ok(
                VipsTarget {
                    ctx: target as *mut bindings::VipsTarget,
                },
            )
        }
    }
}

unsafe extern "C" fn on_read(
    _: *mut bindings::VipsTargetCustom,
    buffer: *mut c_void,
    length: gint64,
    user_data: *mut c_void,
) -> gint64 {
    let ctx = &mut *(user_data as *mut TargetContext);

    let slice = std::slice::from_raw_parts_mut(
        buffer as *mut u8,
        length as usize,
    );

    match ctx
        .writer
        .read(slice)
    {
        Ok(n) => n as i64,
        Err(_) => -1,
    }
}

unsafe extern "C" fn on_seek(
    _: *mut bindings::VipsTargetCustom,
    offset: gint64,
    whence: c_int,
    user_data: *mut c_void,
) -> gint64 {
    let ctx = &mut *(user_data as *mut TargetContext);

    let pos = match whence {
        0 => SeekFrom::Start(offset as u64),
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return -1,
    };

    match ctx
        .writer
        .seek(pos)
    {
        Ok(n) => n as gint64,
        Err(_) => -1,
    }
}

unsafe extern "C" fn on_write(
    _: *mut bindings::VipsTargetCustom,
    data: *const c_void,
    length: gint64,
    user_data: *mut c_void,
) -> gint64 {
    let ctx = &mut *(user_data as *mut TargetContext);

    let slice = std::slice::from_raw_parts(
        data as *const u8,
        length as usize,
    );

    match ctx
        .writer
        .write(slice)
    {
        Ok(n) => n as gint64,
        Err(_) => -1,
    }
}

unsafe extern "C" fn on_end(_: *mut bindings::VipsTargetCustom, data: *mut c_void) -> i32 {
    let _ = Box::from_raw(data as *mut TargetContext);
    0
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
