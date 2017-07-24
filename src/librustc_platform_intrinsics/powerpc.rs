// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// DO NOT EDIT: autogenerated by etc/platform-intrinsics/generator.py
// ignore-tidy-linelength

#![allow(unused_imports)]

use {Intrinsic, Type};
use IntrinsicDef::Named;

// The default inlining settings trigger a pathological behaviour in
// LLVM, which causes makes compilation very slow. See #28273.
#[inline(never)]
pub fn find(name: &str) -> Option<Intrinsic> {
    if !name.starts_with("powerpc") { return None }
    Some(match &name["powerpc".len()..] {
        "_vec_perm" => Intrinsic {
            inputs: { static INPUTS: [&'static Type; 3] = [&::I32x4, &::I32x4, &::I8x16]; &INPUTS },
            output: &::I32x4,
            definition: Named("llvm.ppc.altivec.vperm")
        },
        _ => return None,
    })
}
