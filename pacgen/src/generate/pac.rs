use std::collections::{HashMap, HashSet};

use anyhow::Result;
use chiptool::ir;
use chiptool::util::{ToSanitizedPascalCase, ToSanitizedUpperCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::PeripheralMappings;

pub fn generate_pac(
    device: &ir::Device,
    device_name: &str,
    mapping: &PeripheralMappings,
) -> Result<TokenStream> {
    let mut items = TokenStream::new();

    let span = Span::call_site();

    let mut interrupts = TokenStream::new();
    let mut peripherals = TokenStream::new();
    let mut vectors = TokenStream::new();
    let mut names = vec![];
    let mut mods = HashSet::new();

    let unsuffixed_re = fancy_regex::Regex::new("\\d+(?<=$)")?;

    let mut pos = 0;
    let mut sorted_interrupts = (&device.interrupts).into_iter().collect::<Vec<_>>();
    sorted_interrupts.sort_by_key(|i| i.value);
    for i in sorted_interrupts {
        while pos < i.value {
            vectors.extend(quote!(Vector { _reserved: 0 },));
            pos += 1;
        }
        pos += 1;

        let name_uc = Ident::new(&i.name.to_sanitized_upper_case(), span);
        let description = format!(
            "{} - {}",
            i.value,
            i.description
                .as_ref()
                .map(|s| chiptool::util::respace(s))
                .as_ref()
                .map(|s| chiptool::util::escape_brackets(s))
                .unwrap_or_else(|| i.name.clone())
        );
        let value = chiptool::util::unsuffixed(i.value as u64);

        interrupts.extend(quote! {
            #[doc = #description]
            #name_uc = #value,
        });
        vectors.extend(quote!(Vector { _handler: #name_uc },));
        names.push(name_uc);
    }

    let mut sorted_peripheralss = (&device.peripherals).into_iter().collect::<Vec<_>>();
    sorted_peripheralss.sort_by_key(|i| i.base_address);
    // let mut peripherals_mods = HashMap::new();
    for p in sorted_peripheralss {
        let name = Ident::new(&p.name, span);
        let address = chiptool::util::hex_usize(p.base_address);
        let doc = chiptool::util::doc(&p.description);
        if let Some(block_name) = &p.block {
            if let Some(p0) = mapping.get_mapped_peripheral_name(device_name, &block_name) {
                let p0 = Ident::new(&p0, span);
                let p1 = match unsuffixed_re.captures(&p.name)? {
                    Some(caps) => {
                        let index = caps.get(0).unwrap().as_str();
                        p.name.strip_suffix(index).unwrap().to_string()
                    }
                    None => p.name.clone(),
                };
                let p1 = Ident::new(&p1.to_sanitized_pascal_case(), span);
                let path = quote!(super::#p0::#p1);
                peripherals.extend(quote! {
                    #doc
                    pub const #name: #path = unsafe { #path::from_ptr(#address as _) };
                });
                mods.insert(p0);
                continue;
            }
        }
        peripherals.extend(quote! {
            #doc
            pub const #name: *mut () = #address as _;
        })
    }

    let n = chiptool::util::unsuffixed(pos as u64);
    items.extend(quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Interrupt {
            #interrupts
        }

        unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
            #[inline(always)]
            fn number(self) -> u16 {
                self as u16
            }
        }

        #[cfg(feature = "rt")]
        mod _vectors {
            extern "C" {
                #(fn #names();)*
            }

            pub union Vector {
                _handler: unsafe extern "C" fn(),
                _reserved: u32,
            }

            #[link_section = ".vector_table.interrupts"]
            #[no_mangle]
            pub static __INTERRUPTS: [Vector; #n] = [
                #vectors
            ];
        }

        pub mod instance {
            #peripherals
        }
    });

    if let Some(nvic_priority_bits) = device.nvic_priority_bits {
        let bits = chiptool::util::unsuffixed(nvic_priority_bits as u64);
        items.extend(quote! {
            #[cfg(feature = "rt")]
            pub const NVIC_PRIO_BITS: u8 = #bits;
        });
    }

    // #[cfg(feature = "rt")]
    // pub use cortex_m_rt::interrupt;

    items.extend(quote! {
        #[cfg(feature = "rt")]
        pub use Interrupt as interrupt;
    });

    for m in mods {
        let path = format!("../../peripherals/{}.rs", m);
        items.extend(quote! {
            #[path = #path]
            pub mod #m;
        })
    }

    Ok(items)
}
