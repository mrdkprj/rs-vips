// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use crate::{
    bindings,
    connection::{VipsSource, VipsTarget},
    error::Error,
    ops::*,
    region::VipsBlob,
    utils::{self, ensure_null_terminated, vips_image_result, vips_image_result_ext},
    voption::{call, call_option_string_, Setter, VOption},
    Result,
};
use num_traits::{FromPrimitive, ToPrimitive};
use std::{
    ffi::{c_char, c_int, c_void, CStr},
    path::Path,
    ptr::null_mut,
};

const NULL: *const c_void = null_mut();

#[derive(Debug, Clone)]
pub struct VipsImage {
    pub(crate) ctx: *mut bindings::VipsImage,
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

    /// Creates a new `VipsImage` which, when written to, will create a memory image.
    pub fn new_memory() -> Result<VipsImage> {
        unsafe {
            let res = bindings::vips_image_new_memory();
            vips_image_result(
                res,
                Error::InitializationError("Could not generate object".to_string()),
            )
        }
    }

    /// Opens the named file for reading.
    pub fn new_from_file<P: AsRef<Path>>(filename: P) -> Result<VipsImage> {
        Self::new_from_file_with_opts(
            filename,
            VOption::new(),
        )
    }

    /// Opens the named file for reading.
    pub fn new_from_file_with_opts<P: AsRef<Path>>(
        filename: P,
        option: VOption,
    ) -> Result<VipsImage> {
        unsafe {
            let f = utils::path_to_cstring(filename)?;
            let filename_part = bindings::vips_filename_get_filename(f.as_ptr());
            let string_options = bindings::vips_filename_get_options(f.as_ptr());

            let operation = bindings::vips_foreign_find_load(filename_part);
            if operation.is_null() {
                return vips_image_result(
                    NULL as _,
                    Error::InitializationError("Could not find operation".to_string()),
                );
            }

            let mut out_out = VipsImage::from(null_mut());
            call_option_string_(
                operation,
                string_options,
                option
                    .set(
                        "filename",
                        filename_part,
                    )
                    .set(
                        "out",
                        &mut out_out,
                    ),
            )?;
            vips_image_result_ext(
                out_out,
                Error::InitializationError("Could not initialise VipsImage from file".to_string()),
            )
        }
    }

    /// Opens the named file for simultaneous reading and writing.
    pub fn new_from_file_rw<P: AsRef<Path>>(filename: P) -> Result<VipsImage> {
        unsafe {
            let f = utils::path_to_cstring(filename)?;
            let res = bindings::vips_image_new_from_file_RW(f.as_ptr());
            vips_image_result(
                res,
                Error::InitializationError("Could not initialise VipsImage from file".to_string()),
            )
        }
    }

    /// Maps the named file and returns a `VipsImage` you can use to read it.
    pub fn new_from_file_raw<P: AsRef<Path>>(
        filename: P,
        x_size: i32,
        y_size: i32,
        bands: i32,
        offset: u64,
    ) -> Result<VipsImage> {
        unsafe {
            let f = utils::path_to_cstring(filename)?;
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

    /// Loads an image from the formatted area of memory.
    pub fn new_from_buffer(buffer: &[u8], option_str: &str) -> Result<VipsImage> {
        Self::new_from_buffer_with_opts(
            buffer,
            option_str,
            VOption::new(),
        )
    }

    /// Loads an image from the formatted area of memory.
    pub fn new_from_buffer_with_opts(
        buffer: &[u8],
        option_str: &str,
        option: VOption,
    ) -> Result<VipsImage> {
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

            let vips_blob = bindings::vips_blob_new(
                None,
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
            );
            let mut out_out = VipsImage::from(null_mut());
            let blob = VipsBlob::from(vips_blob);
            call_option_string_(
                operation,
                utils::new_c_string(option_str)?.as_ptr(),
                option
                    .set(
                        "buffer",
                        &blob,
                    )
                    .set(
                        "out",
                        &mut out_out,
                    ),
            )?;
            blob.area_unref();
            vips_image_result_ext(
                out_out,
                Error::InitializationError(
                    "Could not initialise VipsImage from buffer".to_string(),
                ),
            )
        }
    }

    /// Loads an image from the formatted source.
    pub fn new_from_source(source: &VipsSource, option_str: &str) -> Result<VipsImage> {
        Self::new_from_source_with_opts(
            source,
            option_str,
            VOption::new(),
        )
    }

    /// Loads an image from the formatted source.
    pub fn new_from_source_with_opts(
        source: &VipsSource,
        option_str: &str,
        option: VOption,
    ) -> Result<VipsImage> {
        unsafe {
            let operation = bindings::vips_foreign_find_load_source(source.ctx);
            if operation.is_null() {
                return vips_image_result(
                    NULL as _,
                    Error::InitializationError(
                        "Could not initialise VipsImage from source".to_string(),
                    ),
                );
            }

            let mut out_out = VipsImage::from(null_mut());
            call_option_string_(
                operation,
                utils::new_c_string(option_str)?.as_ptr(),
                option
                    .set(
                        "source",
                        source,
                    )
                    .set(
                        "out",
                        &mut out_out,
                    ),
            )?;
            vips_image_result_ext(
                out_out,
                Error::InitializationError(
                    "Could not initialise VipsImage from buffer".to_string(),
                ),
            )
        }
    }

    /// Wraps a VipsImage around a memory area.
    ///
    /// # Safety
    ///
    /// Because VIPS is "borrowing" data from the caller, this function is extremely dangerous.
    /// Unless you are very careful, you will get crashes or memory corruption.
    /// Use `new_from_memory_copy()` instead if you are at all unsure.
    pub unsafe fn new_from_memory(
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

    /// Like `new_from_memory()`, but VIPS will make a copy of the memory area.
    pub fn new_from_memory_copy(
        buffer: &[u8],
        width: i32,
        height: i32,
        bands: i32,
        format: BandFormat,
    ) -> Result<VipsImage> {
        unsafe {
            if let Some(format) = format.to_i32() {
                let res = bindings::vips_image_new_from_memory_copy(
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

    /// Makes an image which is a matrix: a one-band `VIPS_FORMAT_DOUBLE` image held in memory.
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

    /// As `new_matrix()`, but initialise the matrix from the argument list. Same as `new_matrix_from_array()`.
    pub fn new_matrixv(width: i32, height: i32, array: &[f64]) -> Result<VipsImage> {
        Self::new_matrix_from_array(
            width,
            height,
            array,
        )
    }

    /// As `new_matrix()`, but initialise the matrix from the argument list. Same as `new_matrixv()`.
    pub fn new_matrix_from_array(width: i32, height: i32, array: &[f64]) -> Result<VipsImage> {
        unsafe {
            let res = bindings::vips_image_new_matrix_from_array(
                width,
                height,
                array.as_ptr(),
                array.len() as c_int,
            );
            vips_image_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsImage from matrix".to_string(),
                ),
            )
        }
    }

    /// Creates a new image with width, height, format, interpretation, resolution and offset taken from image,
    /// but with number of bands taken from `array` and the value of each band element set from the number of elements in the `array`.
    pub fn new_from_image(image: &VipsImage, array: &[f64]) -> Result<VipsImage> {
        unsafe {
            let res = bindings::vips_image_new_from_image(
                image.ctx,
                array.as_ptr(),
                array.len() as c_int,
            );
            vips_image_result(
                res,
                Error::InitializationError(
                    "Could not initialise VipsImage from Object".to_string(),
                ),
            )
        }
    }

    /// Creates a new image with width, height, format, interpretation, resolution and offset taken from image, but with one band and each pixel having the value `c`.
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

    /// Makes a `VipsImage` which, when written to, will create a temporary file on disc. The file will be automatically deleted when the image is destroyed.
    pub fn new_temp_file(format: &str) -> Result<VipsImage> {
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

    /// Returns the underlying VipsImage reference that this holds
    pub fn as_mut_ptr(&self) -> *mut bindings::VipsImage {
        self.ctx
    }

    /// Allocates memory, renders image into it, builds a new image around the memory area, and returns that.
    pub fn copy_memory(&self) -> Result<VipsImage> {
        unsafe {
            let result = bindings::vips_image_copy_memory(self.ctx);
            vips_image_result(
                result,
                Error::OperationError("Could not copy memory".to_string()),
            )
        }
    }

    /// Invalidates all pixel caches on image and any downstream images, that is, images which depend on this image.
    pub fn invalidate_all(&self) {
        unsafe {
            bindings::vips_image_invalidate_all(self.ctx);
        }
    }

    /// Minimises memory use on this image and any upstream images, that is, images which this image depends upon.
    pub fn minimise_all(&self) {
        unsafe {
            bindings::vips_image_minimise_all(self.ctx);
        }
    }

    /// Gets image ready for an in-place operation
    pub fn inplace(&mut self) -> Result<()> {
        unsafe {
            let res = bindings::vips_image_inplace(self.ctx);
            utils::result(
                res,
                (),
                Error::IOError("Cannot cannot be modified inplace".to_string()),
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

    pub fn iskilled(&self) -> bool {
        unsafe { bindings::vips_image_iskilled(self.ctx) == 1 }
    }

    pub fn isMSBfirst(&self) -> bool {
        unsafe { bindings::vips_image_isMSBfirst(self.ctx) == 1 }
    }

    pub fn isfile(&self) -> bool {
        unsafe { bindings::vips_image_isfile(self.ctx) == 1 }
    }

    pub fn ispartial(&self) -> bool {
        unsafe { bindings::vips_image_ispartial(self.ctx) == 1 }
    }

    pub fn hasalpha(&self) -> bool {
        unsafe { bindings::vips_image_hasalpha(self.ctx) == 1 }
    }

    /// Sets the VipsImage.kill flag on an image.
    pub fn set_kill(&self, flag: bool) {
        unsafe {
            bindings::vips_image_set_kill(
                self.ctx,
                if flag { 1 } else { 0 },
            );
        }
    }

    /// Enables progress reporting on an image.
    pub fn set_progress(&self, flag: bool) {
        unsafe {
            bindings::vips_image_set_progress(
                self.ctx,
                if flag { 1 } else { 0 },
            );
        }
    }

    /// Writes this image to another image.
    pub fn write(&self, other: &VipsImage) -> Result<()> {
        unsafe {
            let res = bindings::vips_image_write(
                self.ctx,
                other.ctx,
            );
            utils::result(
                res,
                (),
                Error::IOError("Cannot write input to output".to_string()),
            )
        }
    }

    /// Writes this image to a file on disc.
    pub fn write_to_file<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        self.write_to_file_with_opts(
            filename,
            VOption::new(),
        )
    }

    /// Writes this image to a file on disc.
    pub fn write_to_file_with_opts<P: AsRef<Path>>(
        &self,
        filename: P,
        option: VOption,
    ) -> Result<()> {
        unsafe {
            let f = utils::path_to_cstring(filename)?;
            let filename_part = bindings::vips_filename_get_filename(f.as_ptr());
            let string_options = bindings::vips_filename_get_options(f.as_ptr());

            let operation = bindings::vips_foreign_find_save(filename_part);
            if operation.is_null() {
                return utils::result(
                    -1,
                    (),
                    Error::IOError("Cannot write to file".to_string()),
                );
            }

            let res = call_option_string_(
                operation,
                string_options,
                option
                    .set("in", self)
                    .set(
                        "filename",
                        filename_part,
                    ),
            )?;
            utils::result(
                res,
                (),
                Error::IOError("Cannot write to file".to_string()),
            )
        }
    }

    /// Writes this image to memory.
    pub fn write_to_buffer(&self, suffix: &str) -> Result<Vec<u8>> {
        self.write_to_buffer_with_opts(
            suffix,
            VOption::new(),
        )
    }

    /// Writes this image to memory.
    pub fn write_to_buffer_with_opts(&self, suffix: &str, option: VOption) -> Result<Vec<u8>> {
        unsafe {
            let f = utils::new_c_string(suffix)?;
            let filename = bindings::vips_filename_get_filename(f.as_ptr());
            let string_options = bindings::vips_filename_get_options(f.as_ptr());

            /* Save with the new target API if we can. Fall back to the older
             * mechanism in case the saver we need has not been converted yet.
             *
             * We need to hide any errors from this first phase.
             */
            bindings::vips_error_freeze();
            let operation = bindings::vips_foreign_find_save_target(filename);
            bindings::vips_error_thaw();

            if !operation.is_null() {
                let target = VipsTarget::new_to_memory()?;
                let res = call_option_string_(
                    operation,
                    string_options,
                    option
                        .set("in", self)
                        .set(
                            "target",
                            &target,
                        ),
                )?;
                return utils::safe_result(
                    res,
                    target,
                    move |target| {
                        target
                            .get_blob()
                            .into()
                    },
                    Error::IOError("Cannot write to buffer".to_string()),
                );
            }

            let operation = bindings::vips_foreign_find_save_buffer(filename);
            if operation.is_null() {
                return utils::result(
                    -1,
                    Vec::new(),
                    Error::IOError("Cannot write to buffer".to_string()),
                );
            }

            let mut buffer_out = VipsBlob::from(null_mut());
            let res = call_option_string_(
                operation,
                string_options,
                option
                    .set("in", self)
                    .set(
                        "buffer",
                        &mut buffer_out,
                    ),
            )?;
            utils::result(
                res,
                buffer_out.into(),
                Error::IOError("Cannot write to buffer".to_string()),
            )
        }
    }

    /// Writes this image to a target.
    pub fn write_to_target(&self, suffix: &str, target: &VipsTarget) -> Result<()> {
        self.write_to_target_with_opts(
            suffix,
            target,
            VOption::new(),
        )
    }

    /// Writes this image to a target.
    pub fn write_to_target_with_opts(
        &self,
        suffix: &str,
        target: &VipsTarget,
        option: VOption,
    ) -> Result<()> {
        unsafe {
            let f = utils::new_c_string(suffix)?;
            let filename = bindings::vips_filename_get_filename(f.as_ptr());
            let string_options = bindings::vips_filename_get_options(f.as_ptr());

            let operation = bindings::vips_foreign_find_save_target(filename);

            if operation.is_null() {
                return utils::result(
                    -1,
                    (),
                    Error::IOError("Cannot write to target".to_string()),
                );
            }

            let res = call_option_string_(
                operation,
                string_options,
                option
                    .set("in", self)
                    .set(
                        "target",
                        target,
                    ),
            )?;
            utils::result(
                res,
                (),
                Error::IOError("Cannot write to target".to_string()),
            )
        }
    }

    /// Writes this image to a large memory array.
    pub fn write_to_memory(&self) -> Vec<u8> {
        unsafe {
            let mut buffer_buf_size: u64 = 0;
            let buffer_out = bindings::vips_image_write_to_memory(
                self.ctx,
                &mut buffer_buf_size,
            );
            let buffer = std::slice::from_raw_parts(
                buffer_out as *mut u8,
                buffer_buf_size as usize,
            )
            .to_vec();
            bindings::g_free(buffer_out);
            buffer
        }
    }

    pub fn decode_predict(
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

    pub fn decode(&self) -> Result<VipsImage> {
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

    pub fn encode(&self, coding: Coding) -> Result<VipsImage> {
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

    /// Reads the GType for a header field.
    pub fn get_typeof(&self, type_: impl AsRef<[u8]>) -> Result<u64> {
        unsafe {
            let type_name = ensure_null_terminated(type_)?;
            let gtype = bindings::vips_image_get_typeof(
                self.ctx,
                type_name.as_ptr(),
            );
            utils::result(
                0,
                gtype,
                Error::IOError("Cannot get type".to_string()),
            )
        }
    }

    /// Gets int from image under the name.
    pub fn get_int(&self, name: impl AsRef<[u8]>) -> Result<i32> {
        unsafe {
            let mut out = 0;
            let name = ensure_null_terminated(name)?;
            let res = bindings::vips_image_get_int(
                self.ctx,
                name.as_ptr(),
                &mut out,
            );
            utils::result(
                res,
                out,
                Error::IOError("Cannot get int".to_string()),
            )
        }
    }

    /// Attaches int as a metadata item on image as name.
    pub fn set_int(&mut self, name: impl AsRef<[u8]>, value: i32) -> Result<()> {
        unsafe {
            let name = ensure_null_terminated(name)?;
            bindings::vips_image_set_int(
                self.ctx,
                name.as_ptr(),
                value,
            );
            Ok(())
        }
    }

    /// Gets double from image under the name.
    pub fn get_double(&self, name: impl AsRef<[u8]>) -> Result<f64> {
        unsafe {
            let mut out = 0.0;
            let name = ensure_null_terminated(name)?;
            let res = bindings::vips_image_get_double(
                self.ctx,
                name.as_ptr(),
                &mut out,
            );
            utils::result(
                res,
                out,
                Error::IOError("Cannot get int".to_string()),
            )
        }
    }

    /// Attaches double as a metadata item on image as name.
    pub fn set_double(&mut self, name: impl AsRef<[u8]>, value: f64) -> Result<()> {
        unsafe {
            let name = ensure_null_terminated(name)?;
            bindings::vips_image_set_double(
                self.ctx,
                name.as_ptr(),
                value,
            );
            Ok(())
        }
    }

    /// Gets string from image under the name.
    pub fn get_string(&self, name: impl AsRef<[u8]>) -> Result<String> {
        unsafe {
            let mut out: *const c_char = std::ptr::null();
            let name = ensure_null_terminated(name)?;
            let res = bindings::vips_image_get_string(
                self.ctx,
                name.as_ptr(),
                &mut out,
            );
            utils::safe_result(
                res,
                out,
                move |out| {
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

    /// Attaches string as a metadata item on image as name.
    pub fn set_string(&mut self, name: impl AsRef<[u8]>, value: &str) -> Result<()> {
        unsafe {
            let name = ensure_null_terminated(name)?;
            let value = ensure_null_terminated(value)?;

            bindings::vips_image_set_string(
                self.ctx,
                name.as_ptr(),
                value.as_ptr(),
            );
            Ok(())
        }
    }

    /// Gets data from image under the name.
    pub fn get_blob(&self, name: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        unsafe {
            let mut out: *const c_void = std::ptr::null();
            let mut length = 0;
            let name = ensure_null_terminated(name)?;
            let res = bindings::vips_image_get_blob(
                self.ctx,
                name.as_ptr(),
                &mut out,
                &mut length,
            );
            utils::safe_result(
                res,
                out,
                move |out| {
                    std::slice::from_raw_parts(
                        out as *const u8,
                        length as usize,
                    )
                    .to_vec()
                },
                Error::IOError("Cannot get blob".to_string()),
            )
        }
    }

    /// Attaches data as a metadata item on image under the name.
    pub fn set_blob(&mut self, name: impl AsRef<[u8]>, blob: &[u8]) -> Result<()> {
        unsafe {
            let name = ensure_null_terminated(name)?;
            bindings::vips_image_set_blob(
                self.ctx,
                name.as_ptr(),
                None,
                blob.as_ptr() as *const c_void,
                blob.len() as u64,
            );
            Ok(())
        }
    }

    /// Gets an array of int from image under the name.
    pub fn get_array_int(&self, name: impl AsRef<[u8]>) -> Result<Vec<i32>> {
        unsafe {
            let mut out: *mut i32 = std::ptr::null_mut();
            let mut size = 0;
            let name = ensure_null_terminated(name)?;
            let res = bindings::vips_image_get_array_int(
                self.ctx,
                name.as_ptr(),
                &mut out,
                &mut size,
            );
            utils::safe_result(
                res,
                out,
                move |out| {
                    utils::new_int_array(
                        out,
                        size as u64,
                    )
                },
                Error::IOError("Cannot get array int".to_string()),
            )
        }
    }

    /// Attaches array as a metadata item on image as name.
    pub fn set_array_int(&mut self, name: impl AsRef<[u8]>, value: &[i32]) -> Result<()> {
        unsafe {
            let name = ensure_null_terminated(name)?;
            bindings::vips_image_set_array_int(
                self.ctx,
                name.as_ptr(),
                value.as_ptr(),
                value.len() as c_int,
            );
            Ok(())
        }
    }

    /// Gets an array of double from image under the name.
    pub fn get_array_double(&self, name: impl AsRef<[u8]>) -> Result<Vec<f64>> {
        unsafe {
            let mut out: *mut f64 = std::ptr::null_mut();
            let mut size = 0;
            let name = ensure_null_terminated(name)?;
            let res = bindings::vips_image_get_array_double(
                self.ctx,
                name.as_ptr(),
                &mut out,
                &mut size,
            );
            utils::safe_result(
                res,
                out,
                move |out| {
                    utils::new_double_array(
                        out,
                        size as u64,
                    )
                },
                Error::IOError("Cannot get array double".to_string()),
            )
        }
    }

    /// Attaches array as a metadata item on image as name.
    pub fn set_array_double(&mut self, name: impl AsRef<[u8]>, value: &[f64]) -> Result<()> {
        unsafe {
            let name = ensure_null_terminated(name)?;
            bindings::vips_image_set_array_double(
                self.ctx,
                name.as_ptr(),
                value.as_ptr(),
                value.len() as c_int,
            );
            Ok(())
        }
    }

    /// Attaches image as a metadata item on image as name.
    pub fn set_image(&mut self, name: impl AsRef<[u8]>, value: &VipsImage) -> Result<()> {
        unsafe {
            let name = ensure_null_terminated(name)?;
            bindings::vips_image_set_image(
                self.ctx,
                name.as_ptr(),
                value.ctx,
            );
            Ok(())
        }
    }

    /// Finds and removes an item of metadata.
    pub fn remove(&mut self, name: impl AsRef<[u8]>) -> Result<bool> {
        unsafe {
            let name = ensure_null_terminated(name)?;
            Ok(
                bindings::vips_image_remove(
                    self.ctx,
                    name.as_ptr(),
                ) == 1,
            )
        }
    }

    /// Returns the coordinates of the image minimum.
    pub fn minpos(&self) -> Result<(f64, f64)> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;

        let vips_op_response = call(
            "min",
            VOption::new()
                .set("in", self)
                .set(
                    "x",
                    &mut x,
                )
                .set(
                    "y",
                    &mut y,
                ),
        )?;
        utils::result(
            vips_op_response,
            (x, y),
            Error::OperationError("minpos failed".to_string()),
        )
    }

    /// Returns the coordinates of the image maximum.
    pub fn maxpos(&self) -> Result<(f64, f64)> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;

        let vips_op_response = call(
            "max",
            VOption::new()
                .set("in", self)
                .set(
                    "x",
                    &mut x,
                )
                .set(
                    "y",
                    &mut y,
                ),
        )?;
        utils::result(
            vips_op_response,
            (x, y),
            Error::OperationError("maxpos failed".to_string()),
        )
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

impl From<*mut bindings::VipsImage> for VipsImage {
    fn from(value: *mut bindings::VipsImage) -> Self {
        Self {
            ctx: value,
        }
    }
}
