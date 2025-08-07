#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod bindings;
pub mod error;
mod image;
pub mod operator;
/// Vips Enumerations
pub mod ops;
pub mod utils;
/// VOption, a list of name-value pairs
pub mod voption;

use error::Error;
pub use image::*;
use std::ffi::*;
pub type Result<T> = std::result::Result<T, error::Error>;

pub struct Vips;

/// That's the main type of this crate. Use it to initialize the system
impl Vips {
    /// Starts up libvips
    pub fn init(name: &str, detect_leak: bool) -> Result<()> {
        let cstring = utils::new_c_string(name);
        if let Ok(c_name) = cstring {
            let res = unsafe { bindings::vips_init(c_name.as_ptr()) };
            let result = if res == 0 {
                Ok(())
            } else {
                Err(Error::InitializationError("Failed to init libvips".to_string()))
            };

            if detect_leak {
                unsafe { bindings::vips_leak_set(1) };
            };

            result
        } else {
            Err(Error::InitializationError("Failed to convert rust string to C string".to_string()))
        }
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
            let version = CStr::from_ptr(bindings::vips_version_string());
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
            let buffer = CStr::from_ptr(bindings::vips_error_buffer());
            let buffer_str = buffer
                .to_str()
                .map_err(|_| Error::InitializationError("Error initializing string".to_string()))?;
            Ok(buffer_str.to_string())
        }
    }

    /// Format the string in the style of printf() and append to the error buffer.
    pub fn error(domain: &str, error: &str) -> Result<()> {
        unsafe {
            let c_str_error = utils::new_c_string(error)?;
            let c_str_domain = utils::new_c_string(domain)?;
            bindings::vips_error(
                c_str_domain.as_ptr(),
                c_str_error.as_ptr(),
            );
            Ok(())
        }
    }

    /// Format the string in the style of printf() and append to the error buffer. Then create and append a localised message based on the system error code, usually the value of errno
    pub fn error_system(code: i32, domain: &str, error: &str) -> Result<()> {
        unsafe {
            let c_str_error = utils::new_c_string(error)?;
            let c_str_domain = utils::new_c_string(domain)?;
            bindings::vips_error_system(
                code,
                c_str_domain.as_ptr(),
                c_str_error.as_ptr(),
            );
            Ok(())
        }
    }

    /// Stop errors being logged. Use [func@error_thaw] to unfreeze
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
    pub fn cache_set_max_mem(max: u64) {
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
    pub fn cache_get_max_mem() -> u64 {
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
    pub fn tracked_get_mem() -> u64 {
        unsafe { bindings::vips_tracked_get_mem() }
    }

    /// Returns the largest number of bytes simultaneously allocated via vips_tracked_malloc().
    pub fn tracked_get_mem_highwater() -> u64 {
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
