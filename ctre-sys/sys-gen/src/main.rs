// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bindgen;
use std::env;
use std::path::PathBuf;

fn output_dir() -> PathBuf {
    ctre_sys_dir().join("src")
}

fn ctre_sys_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("..")
}

#[derive(Debug)]
struct BindgenCallbacks;

impl bindgen::callbacks::ParseCallbacks for BindgenCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        // note that returning `None` leaves the variant name unchanged in the generated bindings
        match enum_name {
            Some("tResourceType") => {
                Some(original_variant_name["kResourceType_".len()..].to_owned())
            }
            Some(enum_name) => {
                if original_variant_name.starts_with(enum_name) {
                    Some(original_variant_name[enum_name.len() + 1..].to_owned())
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .derive_default(true)
        .header(format!(
            "{}",
            ctre_sys_dir().join("gen").join("all_wrapper.hpp").display()
        ))
        .enable_cxx_namespaces()
        .whitelist_function("(c_MotController|c_CANifier|c_Logger|c_PigeonIMU)_[A-Za-z0-9_]+")
        .whitelist_type("ctre::phoenix::(ParamEnum|CANifierControlFrame|CANifierStatusFrame|CANifierVelocityMeasPeriod)")
        .whitelist_type("ctre::phoenix::motorcontrol::(ControlFrame|ControlFrameEnhanced|DemandType|FeedbackDevice|RemoteFeedbackDevice|FollowerType|LimitSwitchSource|RemoteLimitSwitchSource|LimitSwitchNormal|NeutralMode|RemoteSensorSource|SensorTerm|StatusFrameEnhanced|StatusFrame|VelocityMeasPeriod)")
        .whitelist_type("ctre::phoenix::motion::(SetValueMotionProfile|TrajectoryPoint|MotionProfileStatus)")
        .whitelist_type("ctre::phoenix::sensors::(PigeonIMU_ControlFrame|PigeonIMU_StatusFrame)")
        .whitelist_type("CANifier_CCI::GeneralPin")
        .default_enum_style(bindgen::EnumVariation::Rust)
        .parse_callbacks(Box::new(BindgenCallbacks))
        .clang_arg(format!("-I{}", ctre_sys_dir().join("include").display()))
        .clang_arg("-xc++")
        // .clang_arg("-nostdinc")
        // .clang_arg("-nostdinc++")
        .clang_arg("-std=c++14");
    println!("builder_args: {:?}", bindings.command_line_flags());
    let out = bindings.generate().expect("Unable to generate bindings");

    out.write_to_file(output_dir().join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!();
}

fn main() {
    generate_bindings();
}
