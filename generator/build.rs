// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj
use inflector::cases::kebabcase::is_kebab_case;
use inflector::Inflector;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const DOC_PARAM_DETAILS: bool = false;

#[derive(Debug, Clone)]
struct Operation {
    name: String,
    vips_name: String,
    description: String,
    required: Vec<Parameter>,
    optional: Vec<Parameter>,
    output: Vec<Parameter>,
}

// Prevent the last "s" is treated as plural and removed
fn to_class_case(str: &str) -> String {
    if str.ends_with("s") && !str.ends_with("ss") {
        let mut modified = str.to_class_case();
        modified.push('s');
        modified
    } else {
        str.to_class_case()
    }
}

impl Operation {
    fn doc_base(&self) -> String {
        format!(
            "/// {}",
            self.description
        )
    }

    fn doc_required(&self) -> String {
        let required = self
            .required
            .iter()
            .map(|r| r.doc())
            .filter(|d| !d.is_empty())
            .collect::<Vec<_>>();

        if required.is_empty() {
            String::new()
        } else {
            required.join("\n///\n")
        }
    }

    fn doc_optional(&self) -> String {
        format!(
            "/// <ins>Optional arguments</ins>\n{}",
            self.optional
                .iter()
                .map(|p| {
                    format!(
                        "///\n{}",
                        p.doc_struct(),
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    fn doc_returns(&self) -> String {
        match self
            .output
            .len()
        {
            0 => String::new(),
            1 => format!(
                "\n/// returns `{}` - {}",
                self.output[0]
                    .param_type
                    .struct_type(),
                self.output[0].description
            ),
            _ => {
                let res = self
                    .output
                    .iter()
                    .map(|o| {
                        format!(
                            "/// {} - {}",
                            o.param_type
                                .struct_type(),
                            o.description
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                format!(
                    "\n/// Tuple (\n{}\n///)",
                    res
                )
            }
        }
    }

    #[allow(clippy::collapsible_else_if)]
    fn doc(&self, with_optional: bool) -> String {
        let base = self.doc_base();
        let required = self.doc_required();
        let returns = self.doc_returns();
        if !self
            .optional
            .is_empty()
            && with_optional
        {
            if required.is_empty() {
                format!(
                    "{}{}\n///\n{}",
                    base,
                    returns,
                    self.doc_optional(),
                )
            } else {
                format!(
                    "{}{}\n///\n{}\n///\n{}",
                    base,
                    returns,
                    required,
                    self.doc_optional(),
                )
            }
        } else {
            if required.is_empty() {
                format!(
                    "{}{}",
                    base, returns
                )
            } else {
                format!(
                    "{}{}\n///\n{}",
                    base, returns, required
                )
            }
        }
    }

    fn get_variables(&self) -> String {
        let mut vars = self
            .required
            .iter()
            .filter(|p| p.param_type == ParamType::ArrayByte)
            .map(|p| {
                format!(
                    r#"
                        let vips_blob = unsafe {{ vips_blob_new(
                            None,
                            {}.as_ptr() as _,
                            {}.len() as _
                        ) }};
                        let blob = VipsBlob::from(vips_blob);
                    "#,
                    p.name, p.name,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let out_vars = self
            .output
            .iter()
            .map(|p| p.declare_out_variable())
            .collect::<Vec<_>>()
            .join("\n");
        vars.push_str(&out_vars);
        vars
    }

    fn get_unref(&self) -> String {
        self.required
            .iter()
            .filter(|p| p.param_type == ParamType::ArrayByte)
            .map(|_| "blob.area_unref();")
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_method_call(&self, with_optional: bool) -> String {
        let mut all_params = Vec::new();
        all_params.append(
            &mut self
                .required
                .clone(),
        );
        all_params.append(
            &mut self
                .output
                .clone(),
        );
        all_params.sort_by_key(|p| p.order);
        let params = all_params
            .iter()
            .map(|p| {
                if self
                    .output
                    .contains(p)
                {
                    format!(
                        r#".set("{}", &mut {}_out)"#,
                        p.vips_name, p.name
                    )
                } else {
                    match p
                        .param_type
                        .clone()
                    {
                        ParamType::ArrayByte => {
                            format!(
                                r#".set("{}", &blob)"#,
                                p.name
                            )
                        }
                        ParamType::VipsImage {
                            ..
                        } => {
                            format!(
                                r#".set("{}", {})"#,
                                p.vips_name, p.name
                            )
                        }
                        ParamType::RefSelf => {
                            format!(
                                r#".set("{}", self)"#,
                                p.vips_name
                            )
                        }
                        ParamType::Enum {
                            ..
                        } => {
                            format!(
                                r#".set("{}", {} as i32)"#,
                                p.vips_name, p.name
                            )
                        }
                        _ => format!(
                            r#".set("{}", {})"#,
                            p.vips_name, p.name
                        ),
                    }
                }
            })
            .collect::<Vec<_>>()
            .join("");

        if with_optional {
            format!(
                "option{}",
                params
            )
        } else {
            format!(
                "VOption::new(){}",
                params
            )
        }
    }

    fn method_body(&self, with_optional: bool) -> String {
        let out_tuple = self
            .output
            .iter()
            .map(|p| p.as_out_param())
            .collect::<Vec<_>>()
            .join(",");
        let out_result = if self
            .output
            .len()
            > 1
        {
            format!(
                "({})",
                out_tuple
            )
        } else if self
            .output
            .is_empty()
        {
            String::from("()")
        } else {
            out_tuple
        };

        format!(
            r#"
                {}
                let vips_op_response = call("{}", {})?;
                {}
                utils::result(vips_op_response, {}, Error::OperationError("{} (vips_{}) failed".to_string()))
            "#,
            self.get_variables(),
            self.vips_name,
            self.get_method_call(with_optional),
            self.get_unref(),
            out_result,
            to_class_case(&self.name),
            self.vips_name,
        )
    }

    fn declaration(&self, with_optional: bool) -> String {
        let vips_name = if self.vips_name == "match" {
            String::from("matches")
        } else {
            self.vips_name
                .clone()
        };
        let name = if with_optional {
            format!(
                "{}_with_opts",
                vips_name
            )
        } else {
            vips_name
        };
        let params = if with_optional {
            let opt = "option: VOption".to_string();
            let params = self
                .required
                .iter()
                .map(|p| p.param_declaration())
                .collect::<Vec<_>>()
                .join(", ");
            if params.is_empty() {
                opt
            } else {
                format!(
                    "{}, {}",
                    params, opt
                )
            }
        } else {
            self.required
                .iter()
                .map(|p| p.param_declaration())
                .collect::<Vec<_>>()
                .join(", ")
        };
        let return_type = if self
            .output
            .is_empty()
        {
            String::from("()")
        } else if self
            .output
            .len()
            == 1
        {
            self.output[0]
                .param_type
                .struct_type()
        } else {
            let types = self
                .output
                .iter()
                .map(|p| {
                    p.param_type
                        .struct_type()
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "({})",
                types
            )
        };
        format!(
            "pub fn {}({}) -> Result<{}>",
            name, params, return_type
        )
    }

    fn enumeration(&self) -> Vec<String> {
        self.required
            .iter()
            .chain(
                self.optional
                    .iter(),
            )
            .chain(
                self.output
                    .iter(),
            )
            .map(|p| p.enumeration())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
    }

    fn body(&self) -> String {
        let mut main = format!(
            r#"
        {}
        {} {{
            {}
        }}
        "#,
            self.doc(false),
            self.declaration(false),
            self.method_body(false)
        );
        if !self
            .optional
            .is_empty()
        {
            main.push_str(
                format!(
                    r#"
        {}
        {} {{
            {}
        }}
        "#,
                    self.doc(true),
                    self.declaration(true),
                    self.method_body(true)
                )
                .as_str(),
            );
        }
        main
    }
}

#[derive(Debug, Clone)]
struct Parameter {
    order: u8,
    name: String,
    vips_name: String,
    description: String,
    param_type: ParamType,
    is_output: bool,
}

impl PartialEq for Parameter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Parameter {
    fn enumeration(&self) -> String {
        self.param_type
            .enumeration()
    }

    fn as_out_param(&self) -> String {
        match self.param_type {
            ParamType::ArrayByte | ParamType::VipsBlob => format!(
                "{}_out.into()",
                self.name
            ),

            _ => format!(
                "{}_out",
                self.name
            ),
        }
    }

    fn doc(&self) -> String {
        if self
            .param_type
            .param_type()
            .is_empty()
        {
            return String::new();
        }
        let mut main_doc = format!(
            "/// {}: `{}` -> {}",
            self.name,
            self.param_type
                .param_type(),
            self.description
        );

        if DOC_PARAM_DETAILS {
            let dc = self
                .param_type
                .doc();
            if !dc.is_empty() {
                main_doc.push('\n');
                main_doc.push_str(&dc);
            }
        }
        main_doc
    }

    fn doc_struct(&self) -> String {
        let mut main_doc = if self
            .param_type
            .option_type(self.is_output)
            .is_empty()
        {
            String::new()
        } else {
            format!(
                "/// {}: {} -> {}",
                self.name,
                self.param_type
                    .option_type(self.is_output),
                self.description
            )
        };

        if DOC_PARAM_DETAILS {
            let dc = self
                .param_type
                .doc();
            if !dc.is_empty() {
                main_doc.push('\n');
                main_doc.push_str(&dc);
            }
        }
        main_doc
    }

    fn declare_out_variable(&self) -> String {
        match self.param_type {
            ParamType::Bool {
                ..
            }
            | ParamType::Int {
                ..
            }
            | ParamType::Double {
                ..
            }
            | ParamType::UInt {
                ..
            } => format!(
                "let mut {}_out: {} = {};",
                self.name,
                self.param_type
                    .vips_out_type(),
                self.param_type
                    .default()
            ),
            ParamType::ArrayDouble | ParamType::ArrayInt | ParamType::ArrayImage => format!(
                "let mut {}_out: {} = Vec::new();",
                self.name,
                self.param_type
                    .vips_out_type()
            ),
            ParamType::ArrayByte | ParamType::VipsBlob => {
                format!(
                    "let mut {}_out = VipsBlob::from(null_mut());",
                    self.name,
                )
            }
            ParamType::VipsImage {
                ..
            } => {
                format!(
                    "let mut {}_out = VipsImage::from(null_mut());",
                    self.name,
                )
            }
            ParamType::VipsSource => {
                format!(
                    "let mut {}_out = VipsSource::from(null_mut());",
                    self.name,
                )
            }
            ParamType::VipsTarget => {
                format!(
                    "let mut {}_out = VipsTarget::from(null_mut());",
                    self.name,
                )
            }
            ParamType::VipsInterpolate => format!(
                "let mut {}_out = VipsInterpolate::from(null_mut());",
                self.name,
            ),
            _ => format!(
                "let mut {}_out: {} = null_mut();",
                self.name,
                self.param_type
                    .vips_out_type()
            ),
        }
    }

    fn param_declaration(&self) -> String {
        if self
            .param_type
            .param_type()
            .is_empty()
        {
            "&self".to_string()
        } else {
            format!(
                "{}: {}",
                self.name,
                self.param_type
                    .param_type()
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ParamType {
    Int {
        min: i32,
        max: i32,
        default: i32,
    },
    UInt {
        min: u64,
        max: u64,
        default: u64,
    },
    Double {
        min: f64,
        max: f64,
        default: f64,
    },
    Str,
    Enum {
        name: String,
        entries: Vec<Enumeration>,
        default: i32,
    },
    Bool {
        default: bool,
    },
    ArrayInt,
    ArrayDouble,
    ArrayImage,
    ArrayByte,
    VipsInterpolate,
    VipsSource,
    VipsTarget,
    VipsImage {
        prev: Option<String>,
    },
    VipsBlob,
    RefSelf,
}

impl ParamType {
    fn doc(&self) -> String {
        match self {
            ParamType::Int {
                min,
                max,
                default,
            } => {
                format!(
                    "/// min: {}, max: {}, default: {}",
                    min, max, default
                )
            }
            ParamType::UInt {
                min,
                max,
                default,
            } => {
                format!(
                    "/// min: {}, max: {}, default: {}",
                    min, max, default
                )
            }
            ParamType::Double {
                min,
                max,
                default,
            } => {
                format!(
                    "/// min: {}, max: {}, default: {}",
                    min, max, default
                )
            }
            ParamType::Bool {
                default,
            } => format!(
                "/// default: {}",
                default
            ),
            ParamType::Enum {
                entries,
                default,
                ..
            } => entries
                .iter()
                .map(|e| {
                    if *default == e.value {
                        format!(
                            "{} [DEFAULT]",
                            e.doc()
                        )
                    } else {
                        e.doc()
                            .to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
            _ => String::new(),
        }
    }

    fn struct_type(&self) -> String {
        match self {
            ParamType::Int {
                ..
            } => String::from("i32"),
            ParamType::UInt {
                ..
            } => String::from("u64"),
            ParamType::Double {
                ..
            } => String::from("f64"),
            ParamType::Str => String::from("String"),
            ParamType::Bool {
                ..
            } => String::from("bool"),
            ParamType::ArrayInt => String::from("Vec<i32>"),
            ParamType::ArrayDouble => String::from("Vec<f64>"),
            ParamType::ArrayByte => String::from("Vec<u8>"),
            ParamType::ArrayImage => String::from("Vec<VipsImage>"),
            ParamType::VipsInterpolate => String::from("VipsInterpolate"),
            ParamType::VipsImage {
                ..
            } => String::from("VipsImage"),
            ParamType::VipsSource => String::from("VipsSource"),
            ParamType::VipsTarget => String::from("VipsTarget"),
            ParamType::VipsBlob => String::from("Vec<u8>"),
            ParamType::Enum {
                name,
                ..
            } => Self::enum_name(name),
            ParamType::RefSelf => String::new(),
        }
    }

    fn option_type(&self, is_output: bool) -> String {
        let opt_type = match self {
            ParamType::Int {
                ..
            } => {
                if is_output {
                    String::from("&mut i32")
                } else {
                    String::from("i32")
                }
            }
            ParamType::UInt {
                ..
            } => String::from("u64"),
            ParamType::Double {
                ..
            } => {
                if is_output {
                    String::from("&mut f64")
                } else {
                    String::from("f64")
                }
            }
            ParamType::Str => String::from("&str"),
            ParamType::Bool {
                ..
            } => {
                if is_output {
                    String::from("&mut bool")
                } else {
                    String::from("bool")
                }
            }
            ParamType::ArrayInt => String::from("&[i32]"),
            ParamType::ArrayDouble => {
                if is_output {
                    String::from("&mut Vec<f64>")
                } else {
                    String::from("&[f64]")
                }
            }
            ParamType::ArrayByte => String::from("&[u8]"),
            ParamType::ArrayImage => String::from("&[VipsImage]"),
            ParamType::VipsInterpolate => String::from("&VipsInterpolate"),
            ParamType::VipsImage {
                ..
            } => {
                if is_output {
                    String::from("&mut VipsImage")
                } else {
                    String::from("&VipsImage")
                }
            }
            ParamType::VipsSource => String::from("&VipsSource"),
            ParamType::VipsTarget => String::from("&VipsTarget"),
            ParamType::VipsBlob => {
                if is_output {
                    String::from("&mut VipsBlob")
                } else {
                    String::from("&VipsBlob")
                }
            }
            ParamType::Enum {
                name,
                ..
            } => Self::enum_name(name),
            ParamType::RefSelf => String::new(),
        };

        match self {
            ParamType::Enum {
                ..
            } => format!(
                "[`{}`]",
                opt_type
            ),
            _ => format!(
                "`{}`",
                opt_type
            ),
        }
    }

    fn param_type(&self) -> String {
        match self {
            ParamType::Int {
                ..
            } => String::from("i32"),
            ParamType::UInt {
                ..
            } => String::from("u64"),
            ParamType::Double {
                ..
            } => String::from("f64"),
            ParamType::Str => String::from("&str"),
            ParamType::Bool {
                ..
            } => String::from("bool"),
            ParamType::ArrayInt => String::from("&[i32]"),
            ParamType::ArrayDouble => String::from("&[f64]"),
            ParamType::ArrayByte => String::from("&[u8]"),
            ParamType::ArrayImage => String::from("&[VipsImage]"),
            ParamType::VipsInterpolate => String::from("&VipsInterpolate"),
            ParamType::VipsImage {
                ..
            } => String::from("&VipsImage"),
            ParamType::VipsSource => String::from("&VipsSource"),
            ParamType::VipsTarget => String::from("&VipsTarget"),
            ParamType::VipsBlob => String::from("&[u8]"),
            ParamType::Enum {
                name,
                ..
            } => Self::enum_name(name),
            ParamType::RefSelf => String::new(),
        }
    }

    fn enum_name(name: &str) -> String {
        let split: Vec<&str> = name
            .split("Vips")
            .collect();
        if split.len() > 1 {
            split[1].to_string()
        } else {
            split[0].to_string()
        }
    }

    fn vips_out_type(&self) -> String {
        match self {
            ParamType::Int {
                ..
            } => String::from("i32"),
            ParamType::UInt {
                ..
            } => String::from("u64"),
            ParamType::Double {
                ..
            } => String::from("f64"),
            ParamType::Str => String::from("&str"),
            ParamType::Bool {
                ..
            } => String::from("bool"),
            ParamType::ArrayInt => String::from("Vec<i32>"),
            ParamType::ArrayDouble => String::from("Vec<f64>"),
            ParamType::ArrayByte => String::from("Vec<u8>"),
            _ => String::new(),
        }
    }

    fn default(&self) -> String {
        match self {
            ParamType::Int {
                default,
                ..
            } => default.to_string(),
            ParamType::UInt {
                default,
                ..
            } => default.to_string(),
            ParamType::Double {
                default,
                ..
            } => format!(
                "{:.1}",
                default
            ),
            ParamType::Str => String::from("String::new()"),
            ParamType::Bool {
                default,
                ..
            } => default.to_string(),
            ParamType::ArrayInt => String::from("Vec::new()"),
            ParamType::ArrayDouble => String::from("Vec::new()"),
            ParamType::ArrayByte => String::from("Vec::new()"),
            ParamType::ArrayImage => String::from("Vec::new()"),
            ParamType::VipsInterpolate => String::from("VipsInterpolate::new()"),
            ParamType::VipsImage {
                ..
            } => String::from("VipsImage::new()"),
            ParamType::VipsSource => String::from("VipsSource::new()"),
            ParamType::VipsTarget => String::from("VipsTarget::new()"),
            ParamType::VipsBlob => String::from("Vec::new()"),
            ParamType::Enum {
                name,
                entries,
                default,
            } => entries
                .iter()
                .filter(|e| *default == e.value)
                .map(|e| {
                    format!(
                        "{}::{}",
                        Self::enum_name(name),
                        to_class_case(&e.nick)
                    )
                })
                .collect::<Vec<_>>()[0]
                .clone(),
            ParamType::RefSelf => String::new(),
        }
    }

    fn enumeration(&self) -> String {
        match self {
            ParamType::Enum {
                name,
                entries,
                ..
            } => {
                let enum_entries = entries
                    .iter()
                    .map(|e| e.code())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    r#"
                #[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
                pub enum {} {{
                    {}
                }}
                "#,
                    Self::enum_name(name),
                    enum_entries
                )
            }
            _ => String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Enumeration {
    name: String,
    nick: String,
    value: i32,
}

impl Enumeration {
    fn doc(&self) -> String {
        format!(
            "///  `{}` -> {} = {}",
            to_class_case(&self.nick),
            self.name,
            self.value
        )
    }

    fn code(&self) -> String {
        format!(
            "{}\n{} = {},",
            self.doc(),
            if self.name == "VIPS_INTERPRETATION_LABS" {
                String::from("Labs")
            } else {
                to_class_case(&self.nick)
            },
            self.value
        )
    }
}

fn parse_param(
    param_list: Vec<&str>,
    order: u8,
    prev: Option<String>,
) -> (
    bool,
    Parameter,
) {
    let (mut param_name, is_output) = if param_list[0].starts_with("OUTPUT:") {
        let splited: Vec<&str> = param_list[0]
            .split("OUTPUT:")
            .collect();
        (
            String::from(splited[1]),
            true,
        )
    } else {
        (
            String::from(param_list[0]),
            false,
        )
    };
    let vips_name = param_name.clone();
    if ["in", "ref"].contains(&param_name.as_str()) {
        param_name = format!(
            "{}p",
            param_name
        );
    }
    let description = param_list[2];
    let param_type = if param_list[3].starts_with("string") {
        ParamType::Str
    } else if param_list[3].starts_with("VipsImage") {
        if order == 0 && !is_output {
            ParamType::RefSelf
        } else {
            ParamType::VipsImage {
                prev,
            }
        }
    } else if param_list[3].starts_with("VipsBlob") {
        ParamType::VipsBlob
    } else if param_list[3].starts_with("VipsInterpolate") {
        ParamType::VipsInterpolate
    } else if param_list[3].starts_with("VipsSource") {
        ParamType::VipsSource
    } else if param_list[3].starts_with("VipsTarget") {
        ParamType::VipsTarget
    } else if param_list[3].starts_with("bool") {
        let default = param_list[3]
            .split(':')
            .collect::<Vec<&str>>()[1]
            == "1";
        ParamType::Bool {
            default,
        }
    } else if param_list[3].starts_with("int") {
        let strs: Vec<&str> = param_list[3]
            .split(':')
            .collect();

        let min = strs[1]
            .parse()
            .expect("Cannot parse number");
        let max = strs[2]
            .parse()
            .expect("Cannot parse number");
        let default = strs[3]
            .parse()
            .expect("Cannot parse number");
        ParamType::Int {
            min,
            max,
            default,
        }
    } else if param_list[3].starts_with("double") {
        let strs: Vec<&str> = param_list[3]
            .split(':')
            .collect();
        let min = strs[1]
            .parse()
            .expect("Cannot parse number");
        let max = strs[2]
            .parse()
            .expect("Cannot parse number");
        let default: f64 = strs[3]
            .parse()
            .expect("Cannot parse number");
        ParamType::Double {
            min,
            max,
            default,
        }
    } else if param_list[3].starts_with("uint64") {
        let strs: Vec<&str> = param_list[3]
            .split(':')
            .collect();

        let min = strs[1]
            .parse()
            .expect("Cannot parse number");
        let max = strs[2]
            .parse()
            .expect("Cannot parse number");
        let default = strs[3]
            .parse()
            .expect("Cannot parse number");
        ParamType::UInt {
            min,
            max,
            default,
        }
    } else if param_list[3].starts_with("byte-data") {
        ParamType::ArrayByte
    } else if param_list[3].starts_with("array of int") {
        ParamType::ArrayInt
    } else if param_list[3].starts_with("array of double") {
        ParamType::ArrayDouble
    } else if param_list[3].starts_with("array of images") {
        ParamType::ArrayImage
    } else if param_list[3].starts_with("enum") || param_list[3].starts_with("flags") {
        let enum_name = param_list[3]
            .split('-')
            .collect::<Vec<_>>()[1];

        let enum_values = param_list
            .iter()
            .take(param_list.len() - 1)
            .skip(4)
            .map(|param| {
                let enum_strs = param
                    .split(':')
                    .collect::<Vec<_>>();

                let value = enum_strs[0]
                    .parse()
                    .expect("Cannot parse number");
                let nick = enum_strs[1].to_string();
                let name = enum_strs[2].to_string();

                Enumeration {
                    name,
                    nick,
                    value,
                }
            })
            .collect::<Vec<_>>();

        let default = param_list
            .last()
            .and_then(|param| {
                param
                    .parse::<i32>()
                    .ok()
            })
            .expect("can't get default");

        ParamType::Enum {
            name: enum_name.to_string(),
            entries: enum_values,
            default,
        }
    } else {
        panic!(
            "Unsupported type: {}",
            param_list[3]
        )
    };
    let name = if is_kebab_case(&param_name) { param_name.to_snake_case() } else { param_name };
    let vips_name = if is_kebab_case(&vips_name) { vips_name.to_snake_case() } else { vips_name };
    (
        is_output,
        Parameter {
            order,
            name,
            vips_name,
            description: description.to_string(),
            param_type,
            is_output,
        },
    )
}

fn parse_output(output: String) -> Vec<Operation> {
    let mut out: Vec<&str> = output
        .split("OPERATION:")
        .filter(|op| !op.is_empty())
        .collect();
    out.sort();
    out.into_iter()
        .map(
            |op_str: &str| {
                let mut required: Vec<Parameter> = Vec::new();
                let mut optional: Vec<Parameter> = Vec::new();
                let mut output: Vec<Parameter> = Vec::new();

                let mut op_iter = op_str
                    .lines()
                    .filter(|op| !op.is_empty());

                let op_vals: Vec<&str> = op_iter
                    .by_ref()
                    .take_while(|line| *line != "REQUIRED:")
                    .collect();

                let name_split = op_vals[0]
                    .split(':')
                    .collect::<Vec<_>>();
                let description = op_vals[1].to_string();

                let mut required_vals = op_iter
                    .by_ref()
                    .take_while(|line| *line != "OPTIONAL:")
                    .skip(1)
                    .peekable(); // skip the first line PARAM:
                let mut order: u8 = 0;
                while required_vals
                    .peek()
                    .is_some()
                {
                    let prev = if !required.is_empty() && order == 1 {
                        match required[0].param_type {
                            ParamType::ArrayImage => Some(
                                required[0]
                                    .name
                                    .clone(),
                            ),
                            _ => None,
                        }
                    } else {
                        None
                    };
                    let (is_output, param) = parse_param(
                        required_vals
                            .by_ref()
                            .take_while(|line| *line != "PARAM:")
                            .collect(),
                        order,
                        prev,
                    );
                    if is_output {
                        output.push(param);
                    } else {
                        required.push(param);
                    }
                    order += 1;
                }

                let mut optionals = op_iter
                    .skip(1)
                    .peekable();
                while optionals
                    .peek()
                    .is_some()
                {
                    let param_list = optionals
                        .by_ref()
                        .take_while(|line| *line != "PARAM:")
                        .collect();

                    let (_, param) = parse_param(
                        param_list,
                        0,
                        None,
                    );
                    optional.push(param);
                }

                Operation {
                    name: if name_split[0] == "match" {
                        String::from("matches")
                    } else if name_split[1] == "crop" {
                        String::from("crop")
                    } else {
                        String::from(name_split[0]).to_snake_case()
                    },
                    vips_name: if name_split[1] == "crop" {
                        String::from(name_split[1])
                    } else {
                        String::from(name_split[0])
                    },
                    description,
                    required,
                    optional,
                    output,
                }
            },
        )
        .collect()
}

fn run(mut cmd: Command) -> Vec<String> {
    let output = cmd
        .output()
        .expect("Couldn't run pkg-config");
    split_flags(&output.stdout[..])
}

fn split_flags(output: &[u8]) -> Vec<String> {
    let mut word = Vec::new();
    let mut words = Vec::new();
    let mut escaped = false;

    for &b in output {
        match b {
            _ if escaped => {
                escaped = false;
                word.push(b);
            }
            b'\\' => escaped = true,
            b' ' | b'\n' | b'\r' => {
                if !word.is_empty() {
                    words.push(String::from_utf8(word).unwrap());
                    word = Vec::new();
                }
            }
            _ => word.push(b),
        }
    }

    if !word.is_empty() {
        words.push(String::from_utf8(word).unwrap());
    }

    words
}

fn rustfmt_path() -> io::Result<PathBuf> {
    if let Ok(rustfmt) = env::var("RUSTFMT") {
        return Ok(rustfmt.into());
    }
    match which::which("rustfmt") {
        Ok(p) => Ok(p),
        Err(e) => Err(
            io::Error::new(
                io::ErrorKind::Other,
                format!("{}", e),
            ),
        ),
    }
}

fn rustfmt_generated_string(source: &str) -> io::Result<String> {
    let rustfmt = rustfmt_path()?;
    let mut cmd = Command::new(&*rustfmt);

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut child_stdin = child
        .stdin
        .take()
        .unwrap();
    let mut child_stdout = child
        .stdout
        .take()
        .unwrap();

    let source = source.to_owned();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(source.as_bytes());
        source
    });

    let mut output = vec![];
    io::copy(
        &mut child_stdout,
        &mut output,
    )?;

    let status = child.wait()?;
    let source = stdin_handle
        .join()
        .expect(
            "The thread writing to rustfmt's stdin doesn't do \
         anything that could panic",
        );

    match String::from_utf8(output) {
        Ok(bindings) => match status.code() {
            Some(0) => Ok(bindings),
            Some(2) => Err(
                io::Error::new(
                    io::ErrorKind::Other,
                    "Rustfmt parsing errors.".to_string(),
                ),
            ),
            Some(3) => {
                println!("Rustfmt could not format some lines.");
                Ok(bindings)
            }
            _ => Err(
                io::Error::new(
                    io::ErrorKind::Other,
                    "Internal rustfmt error".to_string(),
                ),
            ),
        },
        _ => Ok(source),
    }
}

fn add_missiong(methods: &mut String) {
    methods.push_str(
        r#"
            // Alias for operator overload
            pub(crate) fn add_image(&self, right: &VipsImage) -> Result<VipsImage> {
                self.add(right)
            }
        "#,
    );

    methods.push_str(
        r#"
        /// VipsBandjoin (bandjoin), bandwise join two images
        /// returns `VipsImage` - Output image
        ///
        /// other: `VipsImage` -> Input images
        pub fn bandjoin_with(self, other: VipsImage) -> Result<VipsImage> {
            Self::bandjoin(&[self, other])
        }
    "#,
    );

    methods.push_str(
        r#"
        /// VipsMedian (median), median filter of the specified size.
        pub fn median(&self, size: i32) -> Result<VipsImage> {
            self.rank(size, size, (size * size) / 2)
        }
    "#,
    );
}

fn generate_opts(out_path: PathBuf) {
    let vips_introspection = Command::new("./introspect")
        .output()
        .expect("Failed to run vips introspection");

    let output =
        String::from_utf8(vips_introspection.stdout).expect("Could not parse introspection output");
    let operations = parse_output(output);

    let mut methods = String::new();
    operations
        .iter()
        .for_each(
            |operation| {
                methods.push_str(
                    operation
                        .body()
                        .as_str(),
                )
            },
        );

    add_missiong(&mut methods);

    let mut enums: Vec<String> = operations
        .iter()
        .flat_map(|o| {
            o.enumeration()
                .into_iter()
        })
        .collect();
    enums.sort();
    enums.dedup(); // not working

    let ops_content = format!(
        r#"
    // (c) Copyright 2019-2025 OLX
    // (c) Copyright 2025 mrdkprj
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::upper_case_acronyms)]
    use crate::bindings::{{vips_blob_new}};
    use crate::connection::VipsSource;
    use crate::connection::VipsTarget;
    use crate::error::*;
    use crate::region::VipsBlob;
    use crate::utils;
    use crate::voption::{{call, Setter, VOption}};
    use crate::Result;
    use crate::VipsImage;
    use std::ffi::c_void;
    use std::ptr::null_mut;

    const NULL: *const c_void = null_mut();

    {}

    impl VipsImage {{
        {}
    }}
    "#,
        enums.join("\n"),
        methods
    );

    let ops_formated = if let Ok(formated) = rustfmt_generated_string(&ops_content) {
        formated
    } else {
        ops_content
    };

    let mut file_ops = File::create(out_path.join("ops.rs")).expect("Can't create file");
    file_ops
        .write_all(ops_formated.as_bytes())
        .expect("Can't write to file");
}

fn main() {
    let out_path = PathBuf::from(env::var("BINDINGS_DIR").unwrap());

    println!("cargo:rustc-link-lib=vips");
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    println!("cargo:rerun-if-changed=vips.h");
    let mut cmd = Command::new("pkg-config");
    cmd.args(["--cflags", "vips"]);
    let flags = run(cmd);

    let mut generator = bindgen::Builder::default()
        .header("vips.h")
        // ignore clippy
        .raw_line("#![allow(clippy::all)]")
        // ignore warnings
        .raw_line("#![allow(warnings)]")
        // replace c_long with rust type
        .raw_line("pub type size_t = u64;")
        .raw_line("pub type gint64 = i64;")
        .raw_line("pub type guint64 = u64;")
        .raw_line("pub type gssize = u64;")
        .raw_line("pub type gsize = u64;")
        .raw_line("pub type gintptr = i64;")
        .raw_line("pub type guintptr = u64;")
        /*
        The previous version of bindgen(v0.53) that was used to generate the bindings had the
        following parameter (size_t_is_usize) set to false by default. In the current
        release (v.0.63.0) it is set to true by default which might actually make sense as
        usize could be a better alternative for size_t. For the sake of backward compatibility I've
        explicitly set it to false thus we'll have the same bindings output. Later we can look into
        converting this crate to use `usize` for `size_t`.
        More details are available here: https://github.com/rust-lang/rust-bindgen/issues/1901
         */
        .size_t_is_usize(false)
        .blocklist_type("max_align_t")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_type("size_t")
        .blocklist_type("gint64")
        .blocklist_type("guint64")
        .blocklist_type("gssize")
        .blocklist_type("gsize")
        .blocklist_type("gintptr")
        .blocklist_type("guintptr")
        .constified_enum("*")
        .generate_comments(true)
        .impl_debug(true)
        .impl_partialeq(true)
        .derive_debug(true)
        .derive_eq(true)
        .formatter(bindgen::Formatter::Rustfmt);
    for flag in flags.into_iter() {
        generator = generator.clang_arg(flag);
    }
    let bindings = generator
        .generate()
        .expect("Unable to generate bindings");

    let mut cmd_introspect = Command::new("pkg-config");
    cmd_introspect.args(["--cflags", "--libs", "vips"]);
    let instrospect_flags = run(cmd_introspect);

    let mut cc_builder = cc::Build::new();
    for flag in instrospect_flags.into_iter() {
        cc_builder.flag(&flag);
    }
    let mut cc_cmd = cc_builder
        .no_default_flags(true)
        .out_dir("./")
        .flag("-ointrospect")
        .flag("-g")
        .get_compiler()
        .to_command();
    let result = cc_cmd
        .arg("introspect.c")
        .status();
    if result.is_ok()
        && !result
            .unwrap()
            .success()
    {
        let mut cmd = Command::new("./compile.sh");
        let res = cmd
            .status()
            .expect("Couldn't compile introspect.c");
        if !res.success() {
            panic!("Failed to compile introspect.c");
        }
    }

    // Create bindings.rs
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Create ops.rs
    generate_opts(out_path);
}
