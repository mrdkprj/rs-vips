use crate::{
    bindings,
    error::Error,
    ops::*,
    utils::{
        self, ensure_null_terminated, vips_image_result, vips_image_result_ext, vips_source_result,
        vips_target_result,
    },
    voption::{call, Setter, VOption},
    Result,
};
use num_traits::{FromPrimitive, ToPrimitive};
use std::borrow::Cow;
use std::convert::TryInto;
use std::ffi::*;
use std::ptr::null_mut;

const NULL: *const c_void = null_mut();

#[derive(Debug, Clone)]
pub struct VipsImage {
    pub(crate) ctx: *mut bindings::VipsImage,
}

#[derive(Debug, Clone)]
pub struct VipsInterpolate {
    pub(crate) ctx: *mut bindings::VipsInterpolate,
}

#[derive(Debug, Clone)]
pub struct VipsBlob {
    pub(crate) ctx: *mut bindings::VipsBlob,
}

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

impl Default for VipsImage {
    fn default() -> VipsImage {
        VipsImage {
            ctx: unsafe { bindings::vips_image_new() },
        }
    }
}
/// This is the main type of vips. It represents an image and most operations will take one as input and output a new one.
/// In the moment this type is not thread safe. Be careful working within thread environments.
impl VipsImage {
    pub fn new() -> VipsImage {
        VipsImage {
            ctx: unsafe { bindings::vips_image_new() },
        }
    }

    pub fn new_memory() -> Result<VipsImage> {
        unsafe {
            let res = bindings::vips_image_new_memory();
            vips_image_result(
                res,
                Error::InitializationError("Could not generate object".to_string()),
            )
        }
    }

    pub fn new_from_file(filename: &str) -> Result<VipsImage> {
        unsafe {
            let f = utils::new_c_string(filename)?;
            let res = bindings::vips_image_new_from_file(
                f.as_ptr(),
                NULL,
            );
            vips_image_result(
                res,
                Error::InitializationError("Could not initialise VipsImage from file".to_string()),
            )
        }
    }

    pub fn new_from_file_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        unsafe {
            let f = utils::new_c_string(filename)?;
            let operation = bindings::vips_foreign_find_load(f.as_ptr());
            if operation.is_null() {
                return vips_image_result(
                    NULL as _,
                    Error::InitializationError("Could not find operation".to_string()),
                );
            }

            let mut out_out = VipsImage::from(null_mut());
            call(
                CStr::from_ptr(operation)
                    .to_str()
                    .unwrap(),
                option
                    .set(
                        "filename",
                        filename,
                    )
                    .set(
                        "out",
                        &mut out_out,
                    ),
            );
            vips_image_result_ext(
                out_out,
                Error::InitializationError("Could not initialise VipsImage from file".to_string()),
            )
        }
    }

    pub fn new_from_file_rw(filename: &str) -> Result<VipsImage> {
        unsafe {
            let f = utils::new_c_string(filename)?;
            let res = bindings::vips_image_new_from_file_RW(f.as_ptr());
            vips_image_result(
                res,
                Error::InitializationError("Could not initialise VipsImage from file".to_string()),
            )
        }
    }

    pub fn new_from_file_raw(
        filename: &str,
        x_size: i32,
        y_size: i32,
        bands: i32,
        offset: u64,
    ) -> Result<VipsImage> {
        unsafe {
            let f = utils::new_c_string(filename)?;
            let res = bindings::vips_image_new_from_file_raw(
                f.as_ptr(),
                x_size,
                y_size,
                bands,
                offset,
            );
            vips_image_result(
                res,
                Error::InitializationError("Could not initialise VipsImage from file".to_string()),
            )
        }
    }

    pub fn new_from_file_access(filename: &str, access: Access, memory: bool) -> Result<VipsImage> {
        unsafe {
            let access_str = utils::new_c_string("access")?;
            let memory_str = utils::new_c_string("memory")?;
            let f = utils::new_c_string(filename)?;
            let res = bindings::vips_image_new_from_file(
                f.as_ptr(),
                access_str.as_ptr(),
                access as i32,
                memory_str.as_ptr(),
                if memory { 1 } else { 0 },
                NULL,
            );
            vips_image_result(
                res,
                Error::InitializationError("Could not initialise VipsImage from file".to_string()),
            )
        }
    }

    pub fn new_from_buffer(buffer: &[u8], option_str: &str) -> Result<VipsImage> {
        unsafe {
            let options = utils::new_c_string(option_str)?;
            let res = bindings::vips_image_new_from_buffer(
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
                options.as_ptr(),
                NULL,
            );
            vips_image_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsImage from buffer".to_string(),
                ),
            )
        }
    }

    pub fn new_from_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        unsafe {
            let operation = bindings::vips_foreign_find_load_buffer(
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
            );
            if operation.is_null() {
                return vips_image_result(
                    NULL as _,
                    Error::InitializationError(
                        "Could not initialise VipsImage from buffer".to_string(),
                    ),
                );
            }

            let mut out_out = VipsImage::from(null_mut());
            call(
                CStr::from_ptr(operation)
                    .to_str()
                    .unwrap(),
                option
                    .set(
                        "buffer",
                        buffer,
                    )
                    .set(
                        "out",
                        &mut out_out,
                    ),
            );
            vips_image_result_ext(
                out_out,
                Error::InitializationError(
                    "Could not initialise VipsImage from buffer".to_string(),
                ),
            )
        }
    }

    pub fn new_from_memory(
        buffer: &[u8],
        width: i32,
        height: i32,
        bands: i32,
        format: BandFormat,
    ) -> Result<VipsImage> {
        unsafe {
            if let Some(format) = format.to_i32() {
                let res = bindings::vips_image_new_from_memory(
                    buffer.as_ptr() as *const c_void,
                    buffer.len() as u64,
                    width,
                    height,
                    bands,
                    format,
                );
                vips_image_result(
                    res,
                    Error::InitializationError(
                        "Could not initialise VipsImage from memory".to_string(),
                    ),
                )
            } else {
                Err(Error::InitializationError(
                    "Invalid BandFormat. Please file a bug report, as this should never happen.".to_string(),
                ))
            }
        }
    }

    pub fn new_matrix(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let res = bindings::vips_image_new_matrix(
                width,
                height,
            );
            vips_image_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsImage from matrix".to_string(),
                ),
            )
        }
    }

    pub fn new_matrixv(width: i32, height: i32, array: &[f64]) -> Result<VipsImage> {
        unsafe {
            let matrix = bindings::vips_image_new_matrix(
                width,
                height,
            );

            let mut i = 0;
            for y in 0..height {
                for x in 0..width {
                    *utils::vips_matrix(
                        &*matrix,
                        x,
                        y,
                    ) = array[i];
                    i += 1;
                }
            }
            vips_image_result(
                matrix,
                Error::InitializationError(
                    "Could not initialise VipsImage from matrix".to_string(),
                ),
            )
        }
    }

    pub fn new_matrix_from_array(width: i32, height: i32, array: &[f64]) -> Result<VipsImage> {
        unsafe {
            let res = bindings::vips_image_new_matrix_from_array(
                width,
                height,
                array.as_ptr(),
                array.len() as i32,
            );
            vips_image_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsImage from matrix".to_string(),
                ),
            )
        }
    }

    pub fn new_from_image(image: &VipsImage, array: &[f64]) -> Result<VipsImage> {
        unsafe {
            let res = bindings::vips_image_new_from_image(
                image.ctx,
                array.as_ptr(),
                array.len() as i32,
            );
            vips_image_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsImage from Object".to_string(),
                ),
            )
        }
    }

    pub fn new_from_image1(image: &VipsImage, c: f64) -> Result<VipsImage> {
        unsafe {
            let res = bindings::vips_image_new_from_image1(
                image.ctx,
                c,
            );
            vips_image_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsImage from Object".to_string(),
                ),
            )
        }
    }

    pub fn image_new_temp_file(format: &str) -> Result<VipsImage> {
        unsafe {
            let format_c_str = utils::new_c_string(format)?;
            let res = bindings::vips_image_new_temp_file(format_c_str.as_ptr());
            vips_image_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsImage from format".to_string(),
                ),
            )
        }
    }

    pub fn image_copy_memory(image: VipsImage) -> Result<VipsImage> {
        unsafe {
            let result = bindings::vips_image_copy_memory(image.ctx);
            vips_image_result(
                result,
                Error::OperationError("Could not copy memory".to_string()),
            )
        }
    }

    pub fn image_wio_input(&mut self) -> Result<()> {
        unsafe {
            let result = bindings::vips_image_wio_input(self.ctx);
            utils::result(
                result,
                (),
                Error::OperationError("Error on vips image_wio_input".to_string()),
            )
        }
    }

    pub fn get_filename(&self) -> std::result::Result<&str, std::str::Utf8Error> {
        unsafe {
            let filename = bindings::vips_image_get_filename(self.ctx);
            let res = CStr::from_ptr(filename);
            res.to_str()
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { bindings::vips_image_get_width(self.ctx) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { bindings::vips_image_get_height(self.ctx) }
    }

    pub fn get_xoffset(&self) -> i32 {
        unsafe { bindings::vips_image_get_xoffset(self.ctx) }
    }

    pub fn get_yoffset(&self) -> i32 {
        unsafe { bindings::vips_image_get_yoffset(self.ctx) }
    }

    pub fn get_scale(&self) -> f64 {
        unsafe { bindings::vips_image_get_scale(self.ctx) }
    }

    pub fn get_offset(&self) -> f64 {
        unsafe { bindings::vips_image_get_offset(self.ctx) }
    }

    pub fn get_xres(&self) -> f64 {
        unsafe { bindings::vips_image_get_xres(self.ctx) }
    }

    pub fn get_yres(&self) -> f64 {
        unsafe { bindings::vips_image_get_yres(self.ctx) }
    }

    pub fn get_bands(&self) -> i32 {
        unsafe { bindings::vips_image_get_bands(self.ctx) }
    }

    pub fn get_page_height(&self) -> i32 {
        unsafe { bindings::vips_image_get_page_height(self.ctx) }
    }

    pub fn get_n_pages(&self) -> i32 {
        unsafe { bindings::vips_image_get_n_pages(self.ctx) }
    }

    pub fn get_coding(&self) -> Result<Coding> {
        unsafe {
            let res = bindings::vips_image_get_format(self.ctx);
            let format_enum = FromPrimitive::from_i32(res);
            format_enum.ok_or(Error::IOError("Could get format from image".to_string()))
        }
    }

    pub fn get_format(&self) -> Result<BandFormat> {
        unsafe {
            let res = bindings::vips_image_get_format(self.ctx);
            let format_enum = FromPrimitive::from_i32(res);
            format_enum.ok_or(Error::IOError("Could get format from image".to_string()))
        }
    }

    pub fn guess_format(&self) -> Result<BandFormat> {
        unsafe {
            let res = bindings::vips_image_guess_format(self.ctx);
            let format_enum = FromPrimitive::from_i32(res);
            format_enum.ok_or(Error::IOError("Could get format from image".to_string()))
        }
    }

    pub fn get_orientation(&self) -> i32 {
        unsafe { bindings::vips_image_get_orientation(self.ctx) }
    }

    pub fn get_interpretation(&self) -> Result<Interpretation> {
        unsafe {
            let res = bindings::vips_image_get_interpretation(self.ctx);
            let format_enum = FromPrimitive::from_i32(res);
            format_enum.ok_or(Error::IOError("Could get format from image".to_string()))
        }
    }

    pub fn guess_interpretation(&self) -> Result<Interpretation> {
        unsafe {
            let res = bindings::vips_image_guess_interpretation(self.ctx);
            let format_enum = FromPrimitive::from_i32(res);
            format_enum.ok_or(Error::IOError("Could get format from image".to_string()))
        }
    }

    pub fn image_set_delete_on_close(&mut self, flag: bool) {
        unsafe {
            bindings::vips_image_set_delete_on_close(
                self.ctx,
                if flag { 1 } else { 0 },
            );
        }
    }

    pub fn image_invalidate_all(&self) {
        unsafe {
            bindings::vips_image_invalidate_all(self.ctx);
        }
    }

    pub fn image_minimise_all(&self) {
        unsafe {
            bindings::vips_image_minimise_all(self.ctx);
        }
    }

    pub fn image_iskilled(&self) -> bool {
        unsafe { bindings::vips_image_iskilled(self.ctx) == 1 }
    }

    pub fn image_isMSBfirst(&self) -> bool {
        unsafe { bindings::vips_image_isMSBfirst(self.ctx) == 1 }
    }

    pub fn image_isfile(&self) -> bool {
        unsafe { bindings::vips_image_isfile(self.ctx) == 1 }
    }

    pub fn image_ispartial(&self) -> bool {
        unsafe { bindings::vips_image_ispartial(self.ctx) == 1 }
    }

    pub fn image_hasalpha(&self) -> bool {
        unsafe { bindings::vips_image_hasalpha(self.ctx) == 1 }
    }

    pub fn image_set_kill(&self, flag: bool) {
        unsafe {
            bindings::vips_image_set_kill(
                self.ctx,
                if flag { 1 } else { 0 },
            );
        }
    }

    pub fn image_set_progress(&self, flag: bool) {
        unsafe {
            bindings::vips_image_set_progress(
                self.ctx,
                if flag { 1 } else { 0 },
            );
        }
    }

    pub fn image_write(&self) -> Result<VipsImage> {
        unsafe {
            let out: *mut bindings::VipsImage = null_mut();
            let res = bindings::vips_image_write(
                self.ctx,
                out,
            );
            utils::result(
                res,
                VipsImage {
                    ctx: out,
                },
                Error::IOError("Cannot write input to output".to_string()),
            )
        }
    }

    pub fn image_pio_input(&mut self) -> Result<()> {
        unsafe {
            let res = bindings::vips_image_pio_input(self.ctx);
            utils::result(
                res,
                (),
                Error::IOError("Cannot read image".to_string()),
            )
        }
    }

    pub fn image_pio_output(&mut self) -> Result<()> {
        unsafe {
            let res = bindings::vips_image_pio_output(self.ctx);
            utils::result(
                res,
                (),
                Error::IOError("Cannot write image".to_string()),
            )
        }
    }

    pub fn image_inplace(&self) -> Result<()> {
        unsafe {
            let res = bindings::vips_image_inplace(self.ctx);
            utils::result(
                res,
                (),
                Error::IOError("Cannot cannot be modified inplace".to_string()),
            )
        }
    }

    pub fn image_write_to_file(&self, filename: &str) -> Result<()> {
        unsafe {
            let file_c_str = utils::new_c_string(filename)?;
            let res = bindings::vips_image_write_to_file(
                self.ctx,
                file_c_str.as_ptr(),
                NULL,
            );
            utils::result(
                res,
                (),
                Error::IOError("Cannot write to file".to_string()),
            )
        }
    }

    pub fn image_write_prepare(&self) -> Result<()> {
        unsafe {
            let res = bindings::vips_image_write_prepare(self.ctx);
            utils::result(
                res,
                (),
                Error::IOError("Cannot prepare file to write".to_string()),
            )
        }
    }

    pub fn image_write_to_buffer(&self, suffix: &str) -> Result<Vec<u8>> {
        unsafe {
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();
            let suffix_c_str = utils::new_c_string(suffix)?;
            let res = bindings::vips_image_write_to_buffer(
                self.ctx,
                suffix_c_str.as_ptr(),
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                res,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::IOError("Cannot write content to buffer".to_string()),
            )
        }
    }

    pub fn image_write_to_memory(&self) -> Vec<u8> {
        unsafe {
            let mut buffer_buf_size: u64 = 0;
            let buffer_out = bindings::vips_image_write_to_memory(
                self.ctx,
                &mut buffer_buf_size,
            );
            let buf = std::slice::from_raw_parts(
                buffer_out as *mut u8,
                buffer_buf_size as usize,
            )
            .to_vec();
            bindings::g_free(buffer_out);
            buf
        }
    }

    pub fn image_decode_predict(
        &self,
    ) -> Result<(
        i32,
        BandFormat,
    )> {
        unsafe {
            let mut out_bands = 0;
            let mut out_format = 0;
            let res = bindings::vips_image_decode_predict(
                self.ctx,
                &mut out_bands,
                &mut out_format,
            );
            let format_enum = FromPrimitive::from_i32(out_format);
            if let Some(format_enum) = format_enum {
                utils::result(
                    res,
                    (
                        out_bands,
                        format_enum,
                    ),
                    Error::IOError("Could not predict image format".to_string()),
                )
            } else {
                Err(Error::IOError("Could not predict image format".to_string()))
            }
        }
    }

    pub fn image_decode(&self) -> Result<VipsImage> {
        unsafe {
            let mut out: *mut bindings::VipsImage = null_mut();
            let res = bindings::vips_image_decode(
                self.ctx,
                &mut out,
            );
            utils::result(
                res,
                VipsImage {
                    ctx: out,
                },
                Error::IOError("Cannot decode image".to_string()),
            )
        }
    }

    pub fn image_encode(&self, coding: Coding) -> Result<VipsImage> {
        unsafe {
            let mut out: *mut bindings::VipsImage = null_mut();
            let res = bindings::vips_image_encode(
                self.ctx,
                &mut out,
                coding as i32,
            );
            utils::result(
                res,
                VipsImage {
                    ctx: out,
                },
                Error::IOError("Cannot encode image".to_string()),
            )
        }
    }
}

impl VipsImage {
    pub fn as_mut_ptr(&self) -> *mut bindings::VipsImage {
        self.ctx
    }

    /// Read the GType for a header field. Returns zero if there is no field of that name.
    pub fn get_typeof(&self, type_: impl AsRef<[u8]>) -> u64 {
        unsafe {
            bindings::vips_image_get_typeof(
                self.ctx as _,
                ensure_null_terminated(type_).as_ptr() as _,
            )
        }
    }

    pub fn get_int(&self, name: impl AsRef<[u8]>) -> Result<i32> {
        unsafe {
            let mut out = 0;
            let res = bindings::vips_image_get_int(
                self.ctx as _,
                ensure_null_terminated(name).as_ptr() as _,
                &mut out,
            );
            utils::result(
                res,
                out,
                Error::IOError("Cannot get int".to_string()),
            )
        }
    }

    pub fn set_int(&self, name: impl AsRef<[u8]>, value: i32) {
        unsafe {
            bindings::vips_image_set_int(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                value,
            );
        }
    }

    pub fn get_double(&self, name: impl AsRef<[u8]>) -> Result<f64> {
        unsafe {
            let mut out = 0.0;
            let res = bindings::vips_image_get_double(
                self.ctx as _,
                ensure_null_terminated(name).as_ptr() as _,
                &mut out,
            );
            utils::result(
                res,
                out,
                Error::IOError("Cannot get int".to_string()),
            )
        }
    }

    pub fn set_double(&self, name: impl AsRef<[u8]>, value: f64) {
        unsafe {
            bindings::vips_image_set_double(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                value,
            );
        }
    }

    pub fn get_string(&self, name: impl AsRef<[u8]>) -> Result<String> {
        unsafe {
            let mut out: *const c_char = std::ptr::null();
            let res = bindings::vips_image_get_string(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                &mut out,
            );
            utils::safe_result(
                res,
                out,
                |out| {
                    if let Ok(cstr) = CStr::from_ptr(out).to_str() {
                        cstr.to_string()
                    } else {
                        String::new()
                    }
                },
                Error::IOError("Cannot get string".to_string()),
            )
        }
    }

    pub fn set_string(&self, name: impl AsRef<[u8]>, value: &str) {
        unsafe {
            bindings::vips_image_set_string(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                value.as_ptr() as _,
            )
        };
    }

    pub fn get_blob(&self, name: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        unsafe {
            let mut out: *const c_void = std::ptr::null();
            let mut length = 0;
            let res = bindings::vips_image_get_blob(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                &mut out,
                &mut length,
            );
            utils::safe_result(
                res,
                out,
                |out| {
                    std::slice::from_raw_parts(
                        out as *const u8,
                        length as _,
                    )
                    .to_vec()
                },
                Error::IOError("Cannot get blob".to_string()),
            )
        }
    }

    pub fn set_blob(&self, name: impl AsRef<[u8]>, blob: &[u8]) {
        unsafe {
            bindings::vips_image_set_blob(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                None,
                blob.as_ptr() as _,
                blob.len() as _,
            )
        };
    }

    pub fn get_array_int(&self, name: impl AsRef<[u8]>) -> Result<Vec<i32>> {
        unsafe {
            let mut out: *mut i32 = std::ptr::null_mut();
            let mut size = 0;
            let res = bindings::vips_image_get_array_int(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                &mut out,
                &mut size,
            );
            utils::safe_result(
                res,
                out,
                |out| {
                    utils::new_int_array(
                        out,
                        size as _,
                    )
                },
                Error::IOError("Cannot get array int".to_string()),
            )
        }
    }

    pub fn set_array_int(&self, name: impl AsRef<[u8]>, value: &[i32]) {
        unsafe {
            bindings::vips_image_set_array_int(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                value.as_ptr(),
                value.len() as _,
            )
        };
    }

    pub fn get_array_double(&self, name: impl AsRef<[u8]>) -> Result<Vec<f64>> {
        unsafe {
            let mut out: *mut f64 = std::ptr::null_mut();
            let mut size = 0;
            let res = bindings::vips_image_get_array_double(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                &mut out,
                &mut size,
            );
            utils::safe_result(
                res,
                out,
                |out| {
                    utils::new_double_array(
                        out,
                        size as _,
                    )
                },
                Error::IOError("Cannot get array double".to_string()),
            )
        }
    }

    pub fn set_array_double(&self, name: impl AsRef<[u8]>, value: &[f64]) {
        unsafe {
            bindings::vips_image_set_array_double(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
                value.as_ptr(),
                value.len() as _,
            )
        };
    }

    pub fn remove(&self, name: impl AsRef<[u8]>) -> bool {
        unsafe {
            bindings::vips_image_remove(
                self.ctx,
                ensure_null_terminated(name).as_ptr() as _,
            ) == 1
        }
    }

    pub fn minpos(&self) -> Result<(f64, f64)> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;

        let vips_op_response = call(
            "min",
            VOption::new()
                .set(
                    "in",
                    &VipsImage::from(self.ctx),
                )
                .set(
                    "x",
                    &mut x,
                )
                .set(
                    "y",
                    &mut y,
                ),
        );
        utils::result(
            vips_op_response,
            (x, y),
            Error::OperationError("minpos failed".to_string()),
        )
    }

    pub fn maxpos(&self) -> Result<(f64, f64)> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;

        let vips_op_response = call(
            "max",
            VOption::new()
                .set(
                    "in",
                    &VipsImage::from(self.ctx),
                )
                .set(
                    "x",
                    &mut x,
                )
                .set(
                    "y",
                    &mut y,
                ),
        );
        utils::result(
            vips_op_response,
            (x, y),
            Error::OperationError("maxpos failed".to_string()),
        )
    }
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

    pub fn minimise(&mut self) {
        unsafe {
            bindings::vips_source_minimise(self.ctx);
        }
    }

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

    pub fn is_mappable(&self) -> bool {
        unsafe { bindings::vips_source_is_mappable(self.ctx) == 1 }
    }

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

    pub fn finish(self) {
        unsafe {
            bindings::vips_target_end(self.ctx);
        }
    }

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

    pub fn new_from_neasest_static() -> VipsInterpolate {
        unsafe {
            VipsInterpolate {
                ctx: bindings::vips_interpolate_nearest_static(),
            }
        }
    }

    pub fn new_from_bilinear_static() -> VipsInterpolate {
        unsafe {
            VipsInterpolate {
                ctx: bindings::vips_interpolate_bilinear_static(),
            }
        }
    }

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

    pub fn get_window_size(&self) -> i32 {
        unsafe { bindings::vips_interpolate_get_window_size(self.ctx) }
    }

    pub fn get_windows_offset(&self) -> i32 {
        unsafe { bindings::vips_interpolate_get_window_offset(self.ctx) }
    }
}

impl Drop for VipsImage {
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

impl Drop for VipsBlob {
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

impl From<*mut bindings::VipsImage> for VipsImage {
    fn from(value: *mut bindings::VipsImage) -> Self {
        Self {
            ctx: value,
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

impl From<*mut bindings::VipsInterpolate> for VipsInterpolate {
    fn from(value: *mut bindings::VipsInterpolate) -> Self {
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
            Vec::from_raw_parts(
                bytes as *mut u8,
                size as usize,
                size as usize,
            )
        }
    }
}
