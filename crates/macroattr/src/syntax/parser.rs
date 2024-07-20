/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(dead_code)]

// @copy from [lombokrs](https://github.com/photowey/lombokrs)

// syntax/parser

// ----------------------------------------------------------------

use proc_macro::TokenStream;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse, AttributeArgs, Data, DataStruct, DeriveInput, Field, Fields, Lit, Meta, NestedMeta,
};

// ---------------------------------------------------------------- DeriveInput

pub(crate) fn try_derive_input(input: TokenStream) -> DeriveInput {
    parse(input).unwrap()
}

// ---------------------------------------------------------------- Field

pub(crate) fn try_named_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    let struct_name = &input.ident;

    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!(
            "syntax: struct:`{}` does not contain named fields!",
            struct_name
        ),
    }
}

pub(crate) fn try_unnamed_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    let struct_name = &input.ident;

    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => &fields.unnamed,
        _ => panic!(
            "syntax: struct:`{}` does not contain unnamed fields!",
            struct_name
        ),
    }
}

pub(crate) fn try_match_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    let struct_name = &input.ident;

    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => &fields.unnamed,
        _ => panic!(
            "syntax: struct:`{}` does not contain any fields!",
            struct_name
        ),
    }
}

// ----------------------------------------------------------------

#[doc(hidden)]
pub(crate) fn try_attribute_args(attr: &str, args: AttributeArgs) -> Option<String> {
    let mut attrbute = None;

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.is_ident(attr) {
                    if let Lit::Str(n) = nv.lit {
                        attrbute = Some(n.value());
                    }
                }
            }
            _ => {}
        }
    }

    attrbute
}

#[doc(hidden)]
pub(crate) fn try_first_attribute_args(args: AttributeArgs) -> Option<String> {
    match args.first() {
        Some(NestedMeta::Lit(Lit::Str(v))) => Some(v.value()),
        _ => None,
    }
}
