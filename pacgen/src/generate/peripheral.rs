use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use chiptool::{ir, util};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn generate_peripheral(ir: &mut ir::IR) -> Result<TokenStream> {
    let mut items = TokenStream::new();

    chiptool::transform::Sanitize {}.run(ir).unwrap();
    chiptool::transform::sort::Sort {}.run(ir).unwrap();
    chiptool::transform::delete_num_suffix::DeleteNumSuffix {}
        .run(ir)
        .unwrap();

    let mut temp_items = TokenStream::new();
    for (n, b) in sorted_map(&ir.blocks, |name, _| name.clone()) {
        temp_items.extend(generate_block(b, n.to_string())?);
    }
    items.extend(quote!(#temp_items));

    let mut temp_items = TokenStream::new();
    for (n, f) in sorted_map(&ir.fieldsets, |name, _| name.clone()) {
        temp_items.extend(generate_fieldset(f, ir, n.clone())?);
    }
    items.extend(quote! {
        pub mod fieldset {
            #temp_items
        }
    });

    let mut temp_items = TokenStream::new();
    for (n, e) in sorted_map(&ir.enums, |name, _| name.clone()) {
        temp_items.extend(generate_enum(e, ir, n.clone())?);
    }
    items.extend(quote! {
        pub mod enumm {
            #temp_items
        }
    });

    Ok(items)
}

fn generate_block(block: &ir::Block, name: String) -> Result<TokenStream> {
    let mut items = TokenStream::new();
    let span = Span::call_site();

    for i in sorted(&block.items, |i| (i.byte_offset, i.name.clone())) {
        let name = Ident::new(&i.name, span);
        let offset = util::hex_usize(i.byte_offset as u64);

        let doc = util::doc(&i.description);

        match &i.inner {
            ir::BlockItemInner::Register(r) => {
                let reg_ty = if let Some(fieldset_path) = &r.fieldset {
                    let fieldset_path = Ident::new(&fieldset_path, span);
                    quote! { fieldset::#fieldset_path }
                } else {
                    match r.bit_size {
                        8 => quote!(u8),
                        16 => quote!(u16),
                        32 => quote!(u32),
                        64 => quote!(u64),
                        _ => panic!("Invalid bit size: {}", r.bit_size),
                    }
                };

                let access = match r.access {
                    ir::Access::Read => quote!(crate::common::R),
                    ir::Access::Write => quote!(crate::common::W),
                    ir::Access::ReadWrite => quote!(crate::common::RW),
                };

                let ty = quote!(crate::common::Reg<#reg_ty, #access>);
                if let Some(array) = &i.array {
                    let (len, offs_expr) = process_array(array);
                    items.extend(quote! {
                        #doc
                        #[inline(always)]
                        pub const fn #name(self, n: usize) -> #ty {
                            assert!(n < #len);
                            unsafe { crate::common::Reg::from_ptr(self.ptr.add(#offset + #offs_expr) as _) }
                        }
                    });
                } else {
                    items.extend(quote! {
                        #doc
                        #[inline(always)]
                        pub const fn #name(self) -> #ty {
                            unsafe { crate::common::Reg::from_ptr(self.ptr.add(#offset) as _) }
                        }
                    });
                }
            }
            ir::BlockItemInner::Block(b) => {
                let block_path = &b.block;
                let ty = TokenStream::from_str(&block_path).unwrap();

                if let Some(array) = &i.array {
                    let (len, offs_expr) = process_array(array);

                    items.extend(quote! {
                        #doc
                        #[inline(always)]
                        pub const fn #name(self, n: usize) -> #ty {
                            assert!(n < #len);
                            unsafe { #ty::from_ptr(self.ptr.add(#offset + #offs_expr) as _) }
                        }
                    });
                } else {
                    items.extend(quote! {
                        #doc
                        #[inline(always)]
                        pub const fn #name(self) -> #ty {
                            unsafe { #ty::from_ptr(self.ptr.add(#offset) as _) }
                        }
                    });
                }
            }
        }
    }

    let name = Ident::new(&name, span);
    let doc = chiptool::util::doc(&block.description);
    let items = quote! {
        #doc
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct #name {
            ptr: *mut u8
        }

        unsafe impl Send for #name {}
        unsafe impl Sync for #name {}

        impl #name {
            #[inline(always)]
            pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
                Self { ptr: ptr as _ }
            }

            #[inline(always)]
            pub const fn as_ptr(&self) -> *mut () {
                self.ptr as _
            }

            #items
        }
    };

    Ok(items)
}

fn generate_fieldset(
    fieldset: &ir::FieldSet,
    peripheral: &ir::IR,
    name: String,
) -> Result<TokenStream> {
    let span = Span::call_site();
    let mut items = TokenStream::new();

    let ty = match fieldset.bit_size {
        1..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        33..=64 => quote!(u64),
        _ => panic!("Invalid bit size: {}", fieldset.bit_size),
    };

    for f in sorted(&fieldset.fields, |f| (f.bit_offset.clone(), f.name.clone())) {
        let name = Ident::new(&f.name, span);
        let name_set = Ident::new(&format!("set_{}", f.name), span);
        let off_in_reg = f.bit_offset.clone();
        let mask = util::hex(1u64.wrapping_shl(f.bit_size).wrapping_sub(1));
        let doc = util::doc(&f.description);
        let field_ty: TokenStream;
        let to_bits: TokenStream;
        let from_bits: TokenStream;

        if let Some(e_path) = &f.enumm {
            let Some(e) = peripheral.enums.get(e_path) else {
                panic!("Enum not found: {}", e_path);
            };

            let enum_ty = match e.bit_size {
                1..=8 => quote!(u8),
                9..=16 => quote!(u16),
                17..=32 => quote!(u32),
                33..=64 => quote!(u64),
                _ => panic!("Invalid bit size: {}", e.bit_size),
            };

            let e_path = Ident::new(&e_path, span);
            field_ty = quote!(super::enumm::#e_path);
            to_bits = quote!(val.to_bits() as #ty);
            from_bits = quote!(#field_ty::from_bits(val as #enum_ty))
        } else {
            field_ty = match f.bit_size {
                1 => quote!(bool),
                2..=8 => quote!(u8),
                9..=16 => quote!(u16),
                17..=32 => quote!(u32),
                33..=64 => quote!(u64),
                _ => panic!("Invalid bit size: {}", f.bit_size),
            };
            to_bits = quote!(val as #ty);
            from_bits = if f.bit_size == 1 {
                quote!(val != 0)
            } else {
                quote!(val as #field_ty)
            }
        }

        match off_in_reg {
            ir::BitOffset::Regular(off_in_reg) => {
                let off_in_reg = off_in_reg as usize;
                if let Some(array) = &f.array {
                    let (len, offs_expr) = process_array(array);
                    items.extend(quote! {
                        #doc
                        #[inline(always)]
                        pub const fn #name(self, n: usize) -> #field_ty {
                            assert!(n < #len);
                            let offs = #off_in_reg + #offs_expr;
                            let val = (self.0 >> offs) & #mask;
                            #from_bits
                        }

                        #doc
                        #[inline(always)]
                        pub fn #name_set(&mut self, n: usize, val: #field_ty) {
                            assert!(n < #len);
                            let offs = #off_in_reg + #offs_expr;
                            self.0 = (self.0 & !(#mask << offs)) | (((#to_bits) & #mask) << offs);
                        }
                    });
                } else {
                    items.extend(quote! {
                        #doc
                        #[inline(always)]
                        pub const fn #name(&self) -> #field_ty {
                            let val = (self.0>> #off_in_reg) & #mask;
                            #from_bits
                        }

                        #doc
                        #[inline(always)]
                        pub fn #name_set(&mut self, val: #field_ty) {
                            self.0 = (self.0 & !(#mask << #off_in_reg)) | (((#to_bits) & #mask) << #off_in_reg);
                        }
                    });
                }
            }
            ir::BitOffset::Cursed(ranges) => {
                let mut off_in_reg = Vec::new();
                let mut mask = Vec::new();

                let mut off_in_val = vec![0];
                for (index, range) in ranges.iter().enumerate() {
                    off_in_reg.push(*range.start() as usize);
                    mask.push(util::hex(
                        1u64.wrapping_shl(range.end() - range.start() + 1)
                            .wrapping_sub(1),
                    ));
                    if index < ranges.len() - 1 {
                        off_in_val
                            .push(off_in_val[index] + ((range.end() - range.start()) as usize + 1));
                    }
                }

                if let Some(array) = &f.array {
                    let (len, offs_expr) = process_array(array);
                    items.extend(quote! {
                        #doc
                        #[inline(always)]
                        pub const fn #name(&self, n: usize) -> #field_ty {
                            assert!(n < #len);
                            let mut val = 0;
                            #(
                                let offs = #off_in_reg + #offs_expr;
                                val += (((self.0 >> offs) & #mask) << #off_in_val);
                            )*
                            #from_bits
                        }

                        #doc
                        #[inline(always)]
                        pub fn #name_set(&mut self, n: usize, val: #field_ty) {
                            assert!(n < #len);
                            #(
                                let offs = #off_in_reg + #offs_expr;
                                self.0 = (self.0 & !(#mask << offs)) | (((#to_bits >> #off_in_val) & #mask) << offs);
                            )*
                        }
                    });
                } else {
                    items.extend(quote! {
                        #doc
                        #[inline(always)]
                        pub const fn #name(&self) -> #field_ty {
                            let mut val = 0;
                            #( val += (((self.0 >> #off_in_reg) & #mask) << #off_in_val); )*
                            #from_bits
                        }

                        #doc
                        #[inline(always)]
                        pub fn #name_set(&mut self, val: #field_ty) {
                            #( self.0 = (self.0 & !(#mask << #off_in_reg)) | (((#to_bits >> #off_in_val) & #mask) << #off_in_reg); )*
                        }
                    });
                }
            }
        }
    }

    let name = Ident::new(&name, span);
    let doc = util::doc(&fieldset.description);

    let out = quote! {
        #doc
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct #name (pub #ty);

        impl #name {
            #items
        }

        impl Default for #name {
            #[inline(always)]
            fn default() -> Self {
                #name(0)
            }
        }
    };

    Ok(out)
}

fn generate_enum(enumm: &ir::Enum, _peripheral: &ir::IR, name: String) -> Result<TokenStream> {
    let span = Span::call_site();

    let newtype = enumm.bit_size > 8 || (enumm.variants.len() < 6 && enumm.bit_size > 4);

    let ty = match enumm.bit_size {
        1..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        33..=64 => quote!(u64),
        _ => panic!("Invalid bit size: {}", enumm.bit_size),
    };

    let name = Ident::new(&name, span);
    let doc = util::doc(&enumm.description);
    let mask = util::hex(1u64.wrapping_shl(enumm.bit_size).wrapping_sub(1));

    let mut items = TokenStream::new();

    if newtype {
        let mut temp_items = TokenStream::new();
        for f in sorted(&enumm.variants, |f| (f.value, f.name.clone())) {
            let name = Ident::new(&f.name, span);
            let value = util::hex(f.value);
            let doc = util::doc(&f.description);
            temp_items.extend(quote! {
                #doc
                pub const #name: Self = Self(#value);
            });
        }
        items.extend(quote! {
            #doc
            #[repr(transparent)]
            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
            pub struct #name (pub #ty);

            impl #name {
                #items
            }

            impl #name {
                pub const fn from_bits(val: #ty) -> #name {
                    Self(val & #mask)
                }

                pub const fn to_bits(self) -> #ty {
                    self.0
                }
            }
        });

        // let mut names = vec![];
        // let mut values = vec![];
        // let mut docs = vec![];

        // for f in sorted(&enumm.variants, |f| (f.value, f.name.clone())) {
        //     names.push(Ident::new(&f.name, span));
        //     values.push(util::hex(f.value));
        //     docs.push(util::doc(&f.description));
        // }

        // items.extend(quote! {
        //     #doc
        //     #[repr(transparent)]
        //     #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        //     pub struct #name (pub #ty);

        //     impl #name {
        //         #(
        //             #docs
        //             pub const #names: Self = Self(#values);
        //         )*
        //     }

        //     impl #name {
        //         pub const fn from_bits(val: #ty) -> Self {
        //             Self(val & #mask)
        //         }

        //         pub const fn to_bits(self) -> #ty {
        //             self.0
        //         }
        //     }
        // });
    } else {
        let mut names = vec![];
        let mut values = vec![];
        let mut docs = vec![];
        let variants: HashMap<_, _> = enumm.variants.iter().map(|v| (v.value, v)).collect();

        for val in 0..(1 << enumm.bit_size) {
            if let Some(f) = variants.get(&val) {
                names.push(Ident::new(&f.name, span));
                values.push(util::hex(f.value));
                docs.push(util::doc(&f.description));
            } else {
                names.push(Ident::new(&format!("_RESERVED_{:x}", val), span));
                values.push(util::hex(val));
                docs.push(quote!(#[doc="Reserved"]));
            }
        }

        items.extend(quote! {
            #doc
            #[repr(#ty)]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub enum #name {
                #(
                    #docs
                    #names = #values,
                )*
            }

            impl #name {
                #[inline(always)]
                pub const fn from_bits(val: #ty) -> Self {
                    unsafe { core::mem::transmute(val & #mask) }
                }

                #[inline(always)]
                pub const fn to_bits(self) -> #ty {
                    unsafe { core::mem::transmute(self) }
                }
            }
        });
    }

    items.extend(quote! {
        impl From<#ty> for #name {
            #[inline(always)]
            fn from(val: #ty) -> #name {
                #name::from_bits(val)
            }
        }

        impl From<#name> for #ty {
            #[inline(always)]
            fn from(val: #name) -> #ty {
                #name::to_bits(val)
            }
        }
    });

    Ok(items)
}

fn sorted<'a, T: 'a, F, Z>(
    v: impl IntoIterator<Item = &'a T>,
    by: F,
) -> impl IntoIterator<Item = &'a T>
where
    F: Fn(&T) -> Z,
    Z: Ord,
{
    let mut v = v.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|v| by(*v));
    v
}

fn sorted_map<'a, K: 'a, V: 'a, F, Z>(
    v: impl IntoIterator<Item = (&'a K, &'a V)>,
    by: F,
) -> impl IntoIterator<Item = (&'a K, &'a V)>
where
    F: Fn(&K, &V) -> Z,
    Z: Ord,
{
    let mut v = v.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|&(k, v)| by(k, v));
    v
}

fn process_array(array: &ir::Array) -> (usize, TokenStream) {
    match array {
        ir::Array::Regular(array) => {
            let len = array.len as usize;
            let stride = array.stride as usize;
            let offs_expr = quote!(n*#stride);
            (len, offs_expr)
        }
        ir::Array::Cursed(array) => {
            let len = array.offsets.len();
            let offsets = array
                .offsets
                .iter()
                .map(|&x| x as usize)
                .collect::<Vec<_>>();
            let offs_expr = quote!(([#(#offsets),*][n] as usize));
            (len, offs_expr)
        }
    }
}
