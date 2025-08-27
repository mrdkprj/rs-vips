#![allow(unused_imports)]
use crate::{
    bindings::{
        g_log, g_object_get_property, g_object_ref, g_object_set_property, g_object_unref,
        g_type_check_instance_is_a, g_value_dup_boxed, g_value_get_boolean, g_value_get_double,
        g_value_get_int, g_value_get_object, g_value_get_string, g_value_init, g_value_set_boolean,
        g_value_set_boxed, g_value_set_double, g_value_set_enum, g_value_set_int,
        g_value_set_object, g_value_set_string, g_value_set_uint64, g_value_unset, vips_area_unref,
        vips_array_double_get_type, vips_array_image_get_type, vips_array_int_get_type,
        vips_blob_get_type, vips_cache_operation_buildp, vips_enum_from_nick, vips_error_buffer,
        vips_error_clear, vips_image_get_type, vips_interpolate_get_type, vips_object_get_argument,
        vips_object_set_from_string, vips_object_unref_outputs, vips_operation_new,
        vips_source_get_type, vips_target_get_type, vips_value_get_array_double,
        vips_value_get_array_image, vips_value_set_array_double, vips_value_set_array_image,
        vips_value_set_array_int, GLogLevelFlags_G_LOG_LEVEL_WARNING, GParamSpec, GTypeInstance,
        GValue, VipsArgumentClass, VipsArgumentInstance, VipsBlob, VipsImage, VipsObject,
        VipsOperation,
    },
    unref_area,
    utils::{
        get_g_type, new_c_string, G_TYPE_BOOLEAN, G_TYPE_DOUBLE, G_TYPE_INT, G_TYPE_STRING,
        G_TYPE_UINT64,
    },
};
use std::{mem::MaybeUninit, os::raw::c_void};

/// Runs the vips operation with options
pub fn call(operation: &str, option: VOption) -> std::os::raw::c_int {
    call_option_string(
        operation,
        "",
        option,
    )
}

/// Runs the vips operation with options
pub fn call_option_string(
    operation: &str,
    option_string: &str,
    option: VOption,
) -> std::os::raw::c_int {
    let operation = new_c_string(operation).unwrap();
    let option_string = new_c_string(option_string).unwrap();
    call_option_string_(
        operation.as_ptr(),
        option_string.as_ptr() as _,
        option,
    )
}

pub(crate) fn call_option_string_(
    operation: *const i8,
    option_string: *mut i8,
    option: VOption,
) -> std::os::raw::c_int {
    unsafe {
        let mut vips_operation = vips_operation_new(operation);

        if !option_string.is_null()
            && vips_object_set_from_string(
                vips_operation as _,
                option_string,
            ) < 0
        {
            vips_object_unref_outputs(vips_operation as _);
            g_object_unref(vips_operation as _);
            return 1;
        }

        set_opreration(
            vips_operation,
            &option,
        );

        let result = vips_cache_operation_buildp(&mut vips_operation);

        if result < 0 {
            vips_object_unref_outputs(vips_operation as _);
            g_object_unref(vips_operation as _);
            return 1;
        }

        get_operation(
            vips_operation,
            option,
        );

        g_object_unref(vips_operation as _);

        result
    }
}

enum VipsValue<'a> {
    Bool(bool),
    MutBool(&'a mut bool),
    Int(i32),
    MutInt(&'a mut i32),
    Uint(u64),
    Double(f64),
    MutDouble(&'a mut f64),
    Str(&'a str),
    Image(&'a crate::VipsImage),
    MutImage(&'a mut crate::VipsImage),
    IntArray(&'a [i32]),
    DoubleArray(&'a [f64]),
    DoubleVec(Vec<f64>),
    MutDoubleArray(&'a mut Vec<f64>),
    ImageArray(&'a [crate::VipsImage]),
    Blob(&'a crate::region::VipsBlob),
    MutBlob(&'a mut crate::region::VipsBlob),
    Target(&'a crate::connection::VipsTarget),
    Source(&'a crate::connection::VipsSource),
    Interpolate(&'a crate::interpolate::VipsInterpolate),
}

struct Pair<'a> {
    input: bool,
    name: String,
    value: VipsValue<'a>,
}

impl<'a> Pair<'a> {
    fn input(name: &str, value: VipsValue<'a>) -> Self {
        Self {
            input: true,
            name: name.to_string(),
            value,
        }
    }

    fn output(name: &str, value: VipsValue<'a>) -> Self {
        Self {
            input: false,
            name: name.to_string(),
            value,
        }
    }
}

/// VOption, a list of name-value pairs
#[derive(Default)]
pub struct VOption<'a> {
    options: Vec<Pair<'a>>,
}

impl<'a> VOption<'a> {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
        }
    }
}

fn get_operation(vips_operation: *mut VipsOperation, option: VOption) {
    unsafe {
        for opt in option.options {
            if opt.input {
                continue;
            }

            let mut gvalue = MaybeUninit::<GValue>::zeroed();
            let gvalue_ptr = gvalue.as_mut_ptr();
            let name = new_c_string(opt.name).unwrap();

            match opt.value {
                VipsValue::MutBool(out) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_BOOLEAN),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        gvalue_ptr,
                    );
                    *out = g_value_get_boolean(gvalue_ptr) != 0;
                }
                VipsValue::MutInt(out) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_INT),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        gvalue_ptr,
                    );
                    *out = g_value_get_int(gvalue_ptr);
                }
                VipsValue::MutDouble(out) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_DOUBLE),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        gvalue_ptr,
                    );
                    *out = g_value_get_double(gvalue_ptr);
                }
                VipsValue::MutDoubleArray(out) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_array_double_get_type(),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        gvalue_ptr,
                    );
                    let mut len: i32 = 0;
                    let array = vips_value_get_array_double(
                        gvalue_ptr,
                        &mut len,
                    );
                    let result = std::slice::from_raw_parts(
                        array,
                        len as usize,
                    );
                    out.extend(result);
                }
                VipsValue::MutBlob(out) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_blob_get_type(),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        gvalue_ptr,
                    );
                    let out_blob: *mut VipsBlob = g_value_dup_boxed(gvalue_ptr).cast();
                    out.ctx = out_blob;
                }
                VipsValue::MutImage(out) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_image_get_type(),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        gvalue_ptr,
                    );
                    let out_image: *mut VipsImage = g_value_get_object(gvalue_ptr).cast();
                    out.ctx = out_image;
                }
                _ => {}
            }
            g_value_unset(gvalue_ptr);
        }
    }
}

fn set_opreration(operation: *mut VipsOperation, option: &VOption) {
    unsafe {
        for pair in &option.options {
            if !pair.input {
                continue;
            }

            let mut gvalue = MaybeUninit::<GValue>::zeroed();
            let gvalue_ptr = gvalue.as_mut_ptr();

            match pair.value {
                VipsValue::Bool(value) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_BOOLEAN),
                    );
                    g_value_set_boolean(
                        gvalue_ptr,
                        value.into(),
                    );
                }
                VipsValue::Int(value) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_INT),
                    );
                    g_value_set_int(
                        gvalue_ptr,
                        value,
                    );
                }
                VipsValue::Uint(value) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_UINT64),
                    );
                    g_value_set_uint64(
                        gvalue_ptr,
                        value,
                    );
                }
                VipsValue::Double(value) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_DOUBLE),
                    );
                    g_value_set_double(
                        gvalue_ptr,
                        value,
                    );
                }
                VipsValue::Str(value) => {
                    let str = new_c_string(value).unwrap();
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_STRING),
                    );
                    g_value_set_string(
                        gvalue_ptr,
                        str.as_ptr(),
                    );
                }
                VipsValue::IntArray(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_array_int_get_type(),
                    );
                    vips_value_set_array_int(
                        gvalue_ptr,
                        value.as_ptr(),
                        value.len() as _,
                    );
                }
                VipsValue::DoubleArray(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_array_double_get_type(),
                    );
                    vips_value_set_array_double(
                        gvalue_ptr,
                        value.as_ptr(),
                        value.len() as _,
                    );
                }
                VipsValue::Image(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_image_get_type(),
                    );
                    g_value_set_object(
                        gvalue_ptr,
                        value.ctx as *mut c_void,
                    );
                }
                VipsValue::ImageArray(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_array_image_get_type(),
                    );
                    vips_value_set_array_image(
                        gvalue_ptr,
                        value.len() as _,
                    );
                    let array = vips_value_get_array_image(
                        gvalue_ptr,
                        &mut 0,
                    );
                    let array = std::slice::from_raw_parts_mut(
                        array,
                        value.len() as _,
                    );
                    for i in 0..value.len() {
                        g_object_ref(value[i].ctx as _);
                        array[i] = value[i].ctx;
                    }
                }
                VipsValue::Blob(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_blob_get_type(),
                    );
                    g_value_set_boxed(
                        gvalue_ptr,
                        value.ctx as *const c_void,
                    );
                }
                VipsValue::Source(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_source_get_type(),
                    );
                    g_value_set_object(
                        gvalue_ptr,
                        value.ctx as *mut c_void,
                    );
                }
                VipsValue::Target(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_target_get_type(),
                    );
                    g_value_set_object(
                        gvalue_ptr,
                        value.ctx as *mut c_void,
                    );
                }
                VipsValue::Interpolate(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_interpolate_get_type(),
                    );
                    g_value_set_object(
                        gvalue_ptr,
                        value.ctx as *mut c_void,
                    );
                }
                _ => {}
            }

            set_property(
                operation,
                &pair.name,
                gvalue_ptr,
            );
            g_value_unset(gvalue_ptr);
        }
    }
}

fn set_property(operation: *mut VipsOperation, name: &str, value: *mut GValue) {
    unsafe {
        let object: *mut VipsObject = operation.cast();
        let name = new_c_string(name).unwrap();

        let mut pspec: *mut GParamSpec = std::ptr::null_mut();
        let mut argument_class: *mut VipsArgumentClass = std::ptr::null_mut();
        let mut argument_instance: *mut VipsArgumentInstance = std::ptr::null_mut();
        if vips_object_get_argument(
            object,
            name.as_ptr(),
            &mut pspec,
            &mut argument_class,
            &mut argument_instance,
        ) < 0
        {
            g_warning();
            vips_error_clear();
            return;
        }

        let is_param_spec_enum = g_type_check_instance_is_a(
            pspec as *mut GTypeInstance,
            get_g_type("GParamEnum"),
        ) != 0;

        if is_param_spec_enum && (*value).g_type == get_g_type(G_TYPE_STRING) {
            let pspec_type = (*pspec).value_type;
            let enum_value = vips_enum_from_nick(
                (*object).nickname,
                pspec_type,
                g_value_get_string(value),
            );
            if enum_value < 0 {
                g_warning();
                vips_error_clear();
                return;
            }

            let mut gvalue = MaybeUninit::<GValue>::zeroed();
            let value2 = gvalue.as_mut_ptr();
            g_value_init(
                value2,
                pspec_type,
            );
            g_value_set_enum(
                value2,
                enum_value,
            );
            g_object_set_property(
                object.cast(),
                name.as_ptr(),
                value2,
            );
            g_value_unset(value2);
        } else {
            g_object_set_property(
                object.cast(),
                name.as_ptr(),
                value,
            );
        }
    }
}

fn g_warning() {
    let domain = new_c_string("GLib-GObject").unwrap();
    let format = new_c_string("%s").unwrap();
    unsafe {
        g_log(
            domain.as_ptr(),
            GLogLevelFlags_G_LOG_LEVEL_WARNING,
            format.as_ptr(),
            vips_error_buffer(),
        )
    };
}

/// Set the value of a name-value pair of VOption
pub trait Setter<'a, T> {
    fn set(self, name: &str, value: T) -> VOption<'a>;
    fn add(&mut self, name: &str, value: T);
}

// input bool
impl<'a> Setter<'a, bool> for VOption<'a> {
    fn set(mut self, name: &str, value: bool) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Bool(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: bool) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Bool(value),
                ),
            );
    }
}

// input i32
impl<'a> Setter<'a, i32> for VOption<'a> {
    fn set(mut self, name: &str, value: i32) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Int(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: i32) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Int(value),
                ),
            );
    }
}

// input u64
impl<'a> Setter<'a, u64> for VOption<'a> {
    fn set(mut self, name: &str, value: u64) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Uint(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: u64) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Uint(value),
                ),
            );
    }
}

// input f64
impl<'a> Setter<'a, f64> for VOption<'a> {
    fn set(mut self, name: &str, value: f64) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Double(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: f64) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Double(value),
                ),
            );
    }
}

// input &str
impl<'a> Setter<'a, &'a str> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a str) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Str(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a str) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Str(value),
                ),
            );
    }
}

impl<'a> Setter<'a, &'a String> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a String) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Str(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a String) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Str(value),
                ),
            );
    }
}

// input VipsImage
impl<'a> Setter<'a, &'a crate::VipsImage> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a crate::VipsImage) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Image(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a crate::VipsImage) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Image(value),
                ),
            );
    }
}

// input &[i32]
impl<'a> Setter<'a, &'a [i32]> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a [i32]) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::IntArray(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a [i32]) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::IntArray(value),
                ),
            );
    }
}

impl<'a, const N: usize> Setter<'a, &'a [i32; N]> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a [i32; N]) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::IntArray(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a [i32; N]) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::IntArray(value),
                ),
            );
    }
}

// input &[f64]
impl<'a> Setter<'a, &'a [f64]> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a [f64]) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::DoubleArray(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a [f64]) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::DoubleArray(value),
                ),
            );
    }
}

impl<'a, const N: usize> Setter<'a, &'a [f64; N]> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a [f64; N]) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::DoubleArray(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a [f64; N]) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::DoubleArray(value),
                ),
            );
    }
}

// input &[VipsImage]
impl<'a> Setter<'a, &'a [crate::VipsImage]> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a [crate::VipsImage]) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::ImageArray(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a [crate::VipsImage]) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::ImageArray(value),
                ),
            );
    }
}

impl<'a, const N: usize> Setter<'a, &'a [crate::VipsImage; N]> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a [crate::VipsImage; N]) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::ImageArray(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a [crate::VipsImage; N]) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::ImageArray(value),
                ),
            );
    }
}

// input VipsBlob
impl<'a> Setter<'a, &'a crate::region::VipsBlob> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a crate::region::VipsBlob) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Blob(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a crate::region::VipsBlob) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Blob(value),
                ),
            );
    }
}

// input VipsTarget
impl<'a> Setter<'a, &'a crate::connection::VipsTarget> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a crate::connection::VipsTarget) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Target(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a crate::connection::VipsTarget) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Target(value),
                ),
            );
    }
}

// input VipsSource
impl<'a> Setter<'a, &'a crate::connection::VipsSource> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a crate::connection::VipsSource) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Source(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a crate::connection::VipsSource) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Source(value),
                ),
            );
    }
}

// input VipsInterpolate
impl<'a> Setter<'a, &'a crate::interpolate::VipsInterpolate> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a crate::interpolate::VipsInterpolate) -> VOption<'a> {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Interpolate(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a crate::interpolate::VipsInterpolate) {
        self.options
            .push(
                Pair::input(
                    name,
                    VipsValue::Interpolate(value),
                ),
            );
    }
}

// output bool
impl<'a> Setter<'a, &'a mut bool> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a mut bool) -> VOption<'a> {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutBool(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a mut bool) {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutBool(value),
                ),
            );
    }
}

// output i32
impl<'a> Setter<'a, &'a mut i32> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a mut i32) -> VOption<'a> {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutInt(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a mut i32) {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutInt(value),
                ),
            );
    }
}

// output f64
impl<'a> Setter<'a, &'a mut f64> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a mut f64) -> VOption<'a> {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutDouble(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a mut f64) {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutDouble(value),
                ),
            );
    }
}

// output VipsImage
impl<'a> Setter<'a, &'a mut crate::VipsImage> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a mut crate::VipsImage) -> VOption<'a> {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutImage(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a mut crate::VipsImage) {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutImage(value),
                ),
            );
    }
}

// output Vec<f64>
impl<'a> Setter<'a, &'a mut Vec<f64>> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a mut Vec<f64>) -> VOption<'a> {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutDoubleArray(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a mut Vec<f64>) {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutDoubleArray(value),
                ),
            );
    }
}

// output VipsBlob
impl<'a> Setter<'a, &'a mut crate::region::VipsBlob> for VOption<'a> {
    fn set(mut self, name: &str, value: &'a mut crate::region::VipsBlob) -> VOption<'a> {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutBlob(value),
                ),
            );
        self
    }
    fn add(&mut self, name: &str, value: &'a mut crate::region::VipsBlob) {
        self.options
            .push(
                Pair::output(
                    name,
                    VipsValue::MutBlob(value),
                ),
            );
    }
}
