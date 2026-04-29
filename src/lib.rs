// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj

#![allow(clippy::needless_doctest_main)]
//! This is a safe wrapper for [libvips](https://libvips.github.io/libvips/) C library. It is made on top of the C API and based on the introspection API results.
//!
//! This crate itself is not well documented, but it has no logic or special behavior in comparison to libvips itself. All calls and types described in the official libvips [docs](https://libvips.github.io/libvips/API/current/) are just translated to rust types. All defaults also respected.
//!
//! ## About this crate
//!
//! This is a fork of [libvips-rust-bindings](https://github.com/olxgroup-oss/libvips-rust-bindings).
//!
//! This crate is different from it in that
//!
//! - [`VipsImage`] implements vips operations
//! - this uses [`voption::VOption`] for optional arguments of some vips operations instead of structs to prevent unnecessary default values
//! - this supports operator overloads
//! - this supports some operations to VipsImage like [`VipsImage::get_int()`] and [`VipsImage::set_int()`].
//!
//! ## How to use it
//!
//! Vips needs to be initialized. You have to call [`Vips::init()`] at least once before any operations.
//!
//! Shutdown is optional. You can shut down by [`Vips::shutdown()`]. Once Vips is shut down, all operations including [`Vips::init()`] are no longer available.
//!
//! Many vips operations have optional arguments. Basically there'll be a regular call with only the required parameters and an additional with the suffix `with_opts` which takes [`voption::VOption`] containing optional arguments.
//!
//! ```no_run
//! let option = VOption::new().set("embedded", true).set("depth", 16);
//! ```
//!
//! The error messages in the libvips error buffer are appended to the errors themselves.
//!
//! Most (if not all) vips operations don't mutate the underlying `VipsImage` object, so they'll return a new object for this. The implementation of `VipsImage` in this crate takes care of freeing the internal pointer after it is dropped.
//!
//! ## Threads
//! libvips is threaded and thread-safe.
//!
//! [`VipsImage`] is thread-safe as its underlying `VipsImage` object is immutable and can be shared between threads.
//!
//! The exception is the drawing operations, such as [`VipsImage::draw_circle()`]. These operations modify their input image.
//!
//! To ensure thread safety, this crate internally makes a copy of the image in memory before calling the draw operation by [vips_image_copy_memory](https://www.libvips.org/API/current/method.Image.copy_memory.html).
//!
//! Be aware that [`VipsImage`] is not thread-safe at v0.6.0 and earlier.
//!
//! ### Example
//!
//! ```no_run
//! use rs_vips::{voption::{VOption, Setter}, Vips, VipsImage};
//!
//! fn main() {
//!     // this initializes the libvips library.
//!     Vips::init("Test Libvips").expect("Cannot initialize libvips");
//!     // if you want leak checking, turn it on.
//!     Vips::leak_set(true);
//!     // set number of threads in libvips's threadpool
//!     Vips::concurrency_set(2);
//!
//!     // loads an image from file
//!     let image = VipsImage::new_from_file("test.png").unwrap();
//!
//!     // will resize the image and return a new instance.
//!     // libvips works most of the time with immutable objects, so it will return a new object
//!     // the VipsImage struct implements Drop, which will free the memory
//!     let resized = image.resize(0.5).unwrap();
//!
//!     // save with optional parameters
//!     match resized.jpegsave_with_opts(
//!         "output.jpeg",
//!         VOption::new()
//!             .set("q", 90)
//!             .set("background", &[255.0])
//!             .set("strip", true)
//!             .set("interlace", true)
//!             .set("optimize_scans", true)
//!             .set("optimize_coding", true),
//!     ) {
//!         Err(ex) => println!("error: {}", ex),
//!         Ok(_) => println!("Great Success!"),
//!     }
//!
//!     // only when you are done with Vips, shut it down.
//!     // this is optional as described in the official document.
//!     // To clean up, thread_shut_down may be enough.
//!     // Vips::thread_shutdown()
//!     Vips::shutdown();
//! }
//! ```

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod bindings;
mod connection;
/// Vips Enumerations
pub mod enums;
pub mod error;
mod image;
mod interpolate;
pub mod operator;
mod ops;
mod region;
mod source;
mod target;
mod utils;
/// VOption, a list of name-value pairs
pub mod voption;

use error::Error;
pub use image::*;
pub use interpolate::*;
pub use region::*;
pub use source::*;
use std::ffi::CStr;
pub use target::*;

pub type Result<T> = std::result::Result<T, error::Error>;

/// Basic utility struct. Use it to initialize/shutdown the system
pub struct Vips;

impl Vips {
    /// Starts up libvips
    pub fn init(name: &str) -> Result<()> {
        let c_name = utils::new_c_string(name)?;
        let res = unsafe { bindings::vips_init(c_name.as_ptr()) };
        if res == 0 {
            Ok(())
        } else {
            Err(Error::InitializationError("Failed to init libvips".to_string()))
        }
    }

    /// Turn on or off vips leak checking.
    pub fn leak_set(leak: bool) {
        unsafe { bindings::vips_leak_set(if leak { 1 } else { 0 }) };
    }

    /// A structure available to eval callbacks giving information on evaluation progress
    pub fn progress_set(flag: bool) {
        unsafe {
            bindings::vips_progress_set(if flag { 1 } else { 0 });
        }
    }

    /// Return the number of bytes at which we flip between open via memory and open via disc
    pub fn get_disc_threshold() -> u64 {
        unsafe { bindings::vips_get_disc_threshold() }
    }

    /// Get the VIPS version as a static string, including a build date and time.
    pub fn version_string() -> Result<String> {
        unsafe {
            let version_string = bindings::vips_version_string();
            if version_string.is_null() {
                return Err(Error::InitializationError("Cannot get version_string".to_string()));
            }
            let version = CStr::from_ptr(version_string);
            let version_str = version
                .to_str()
                .map_err(|_| Error::InitializationError("Error initializing string".to_string()))?;
            Ok(version_str.to_string())
        }
    }

    /// Free any thread-private data and flush any profiling information.
    pub fn thread_shutdown() {
        unsafe {
            bindings::vips_thread_shutdown();
        }
    }

    /// Get a pointer to the start of the error buffer as string
    pub fn error_buffer() -> Result<String> {
        unsafe {
            let error_buffer = bindings::vips_error_buffer();
            if error_buffer.is_null() {
                return Ok(String::new());
            }
            let buffer = CStr::from_ptr(error_buffer);
            let buffer_str = buffer
                .to_str()
                .map_err(|_| Error::InitializationError("Error initializing string".to_string()))?;
            Ok(buffer_str.to_string())
        }
    }

    /// Stop errors being logged. Use `error_thaw` to unfreeze
    pub fn freeze_error_buffer() {
        unsafe {
            bindings::vips_error_freeze();
        }
    }

    /// Clear and reset the error buffer.
    pub fn error_clear() {
        unsafe {
            bindings::vips_error_clear();
        }
    }

    /// Re-enable error logging.
    pub fn error_thaw() {
        unsafe {
            bindings::vips_error_thaw();
        }
    }

    /// Sends a formatted error message to stderr, then sends the contents of the error buffer, if any, then shuts down vips and terminates the program with an error code.
    pub fn error_exit(error: &str) -> Result<()> {
        unsafe {
            let c_str_error = utils::new_c_string(error)?;
            bindings::vips_error_exit(c_str_error.as_ptr());
        }
    }

    /// Print the whole operation cache to stdout. Handy for debugging.
    pub fn cache_print() {
        unsafe {
            bindings::vips_cache_print();
        }
    }

    /// Set the maximum number of operations we keep in cache.
    pub fn cache_set_max(max: i32) {
        unsafe {
            bindings::vips_cache_set_max(max);
        }
    }

    /// Set the maximum amount of tracked memory we allow before we start dropping cached operations.
    pub fn cache_set_max_mem(max: usize) {
        unsafe {
            bindings::vips_cache_set_max_mem(max);
        }
    }

    /// Set the maximum number of tracked files we allow before we start dropping cached operations.
    pub fn cache_set_max_files(max: i32) {
        unsafe {
            bindings::vips_cache_set_max_files(max);
        }
    }

    /// Get the maximum number of operations we keep in cache.
    pub fn cache_get_max() -> i32 {
        unsafe { bindings::vips_cache_get_max() }
    }

    /// Get the maximum amount of tracked memory we allow before we start dropping cached operations.
    pub fn cache_get_max_mem() -> usize {
        unsafe { bindings::vips_cache_get_max_mem() }
    }

    /// Get the maximum number of tracked files we allow before we start dropping cached operations.
    pub fn cache_get_max_files() -> i32 {
        unsafe { bindings::vips_cache_get_max_files() }
    }

    /// Get the current number of operations in cache.
    pub fn cache_get_size() -> i32 {
        unsafe { bindings::vips_cache_get_size() }
    }

    /// Handy for debugging. Print the operation cache to stdout just before exit.
    pub fn cache_set_dump(flag: bool) {
        unsafe {
            bindings::vips_cache_set_dump(if flag { 1 } else { 0 });
        }
    }

    /// Handy for debugging. Print operation cache actions to stdout as we run
    pub fn cache_set_trace(flag: bool) {
        unsafe {
            bindings::vips_cache_set_trace(if flag { 1 } else { 0 });
        }
    }

    /// set the number of worker threads for vips to operate
    pub fn concurrency_set(max: i32) {
        unsafe {
            bindings::vips_concurrency_set(max);
        }
    }

    /// get the number of worker threads that vips is operating
    pub fn concurrency_get() -> i32 {
        unsafe { bindings::vips_concurrency_get() }
    }

    /// Returns the number of bytes currently allocated via vips_malloc() and friends.
    pub fn tracked_get_mem() -> usize {
        unsafe { bindings::vips_tracked_get_mem() }
    }

    /// Returns the largest number of bytes simultaneously allocated via vips_tracked_malloc().
    pub fn tracked_get_mem_highwater() -> usize {
        unsafe { bindings::vips_tracked_get_mem_highwater() }
    }

    /// Returns the number of active allocations.
    pub fn tracked_get_allocs() -> i32 {
        unsafe { bindings::vips_tracked_get_allocs() }
    }

    /// Returns the number of open files.
    pub fn tracked_get_files() -> i32 {
        unsafe { bindings::vips_tracked_get_files() }
    }

    /// If a source does not support mmap or seek and the source is used with a loader that can only work from memory, then the data will be automatically read into memory to EOF before the loader starts.
    pub fn pipe_read_limit_set(limit: i64) {
        unsafe {
            bindings::vips_pipe_read_limit_set(limit);
        }
    }

    /// Call this to drop caches, close plugins, terminate background threads, and finalize any internal library testing.
    /// vips_shutdown() is optional.
    pub fn shutdown() {
        unsafe {
            bindings::vips_shutdown();
        }
    }
}
