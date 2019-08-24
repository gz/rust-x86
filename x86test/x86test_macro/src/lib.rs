//! This implements the x86test macro to run and also customize the execution
//! of KVM based unit-tests.
#![feature(proc_macro_diagnostic)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{AttributeArgs, Ident, ItemFn, Lit, Meta, MetaList, NestedMeta};

use quote::quote;

/// Parse two integer literals from args (e.g., like (1, 2)).
fn parse_two_ints(args: Punctuated<NestedMeta, Comma>) -> (u64, u64) {
    if args.len() != 2 {
        args.span()
            .unstable()
            .error("needs two numbers as parameters")
            .emit();
    }

    let a = if let NestedMeta::Literal(Lit::Int(first)) = &args[0] {
        first.value()
    } else {
        args[0]
            .span()
            .unstable()
            .error("first parameter not an int literal")
            .emit();
        0
    };

    let b = if let NestedMeta::Literal(Lit::Int(second)) = &args[1] {
        second.value()
    } else {
        args[1]
            .span()
            .unstable()
            .error("second parameter not an int literal")
            .emit();
        0
    };

    (a, b)
}

fn should_panic(fun: &ItemFn) -> bool {
    fun.attrs.iter().find(|&attr| {
        attr.path.segments.iter().find(|&path_segment| path_segment.ident == "should_panic").is_some()
    }).is_some()
}

/// The `x86test` macro adds and initializes a `X86TestFn` struct for
/// every test function. That `X86TestFn` in turn is annotated with
/// `#[test_case]` therefore all these structs are aggregated with
/// by the custom test framework runner which is declared in `runner.rs`.
///
/// # Example
/// As an example, if we add x86test to a function, we do the following:
///
/// ```no-run
/// #[x86test(ram(0x10000, 0x11000), ioport(0x1, 0xfe), should_panic)]
/// fn use_the_port() {
///     unsafe {
///         kassert!(x86::io::inw(0x1) == 0xff, "`inw` instruction didn't read correct value");
///     }
/// }
/// ```
///
/// Will expand to:
///
/// ```no-run
/// fn use_the_port() {
///     unsafe {
///         kassert!(x86::io::inw(0x1) == 0xff, "`inw` instruction didn't read correct value");
///     }
/// }
///
/// #[allow(non_upper_case_globals)]
/// #[test_case]
/// static use_the_port_genkvmtest: X86TestFn = X86TestFn {
///     name: "use_the_port",
///     ignore: false,
///     identity_map: true,
///     physical_memory: (0x10000, 0x11000),
///     ioport_reads: (0x1, 0xfe),
///     should_panic: true,
///     testfn: x86test::StaticTestFn(|| {
///         use_the_port()
///         // Tell our "hypervisor" that we finished the test
///         unsafe { x86::io::outw(0xf4, 0x00); }
///     })
/// };
/// ```
#[proc_macro_attribute]
pub fn x86test(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Vec<NestedMeta> = syn::parse_macro_input!(args as AttributeArgs);
    let input_fn = syn::parse_macro_input!(input as ItemFn);

    let mut physical_memory: (u64, u64) = (0, 0);
    let mut ioport_reads: (u64, u64) = (0, 0);
    let should_panic = should_panic(&input_fn);

    // Parse the arguments of x86test:
    // #[x86test(ram(0xdead, 12), ioport(0x1, 0xfe))]
    // will push (0xdead, 12) to physical_memory and (0x1, 0xfe) to ioport_reads:
    for arg in args {
        if let NestedMeta::Meta(Meta::List(MetaList {
            ident,
            paren_token: _,
            nested,
        })) = arg
        {
            match ident.to_string().as_str() {
                "ram" => {
                    physical_memory = parse_two_ints(nested);
                }
                "ioport" => {
                    ioport_reads = parse_two_ints(nested);
                }
                _ => unreachable!("unsupported attribute"),
            }
        }
    }

    let physical_memory_tuple = {
        let (a, b) = physical_memory;
        quote! { (#a, #b) }
    };

    let ioport_reads_tuple = {
        let (a, b) = ioport_reads;
        quote! { (#a as u16, #b as u32) }
    };

    let struct_name = format!("{}_genkvmtest", input_fn.ident);
    let struct_ident = Ident::new(struct_name.as_str(), Span::call_site());
    let test_name = format!("{}", input_fn.ident);
    let fn_ident = input_fn.ident.clone();

    let ast = quote! {
        #[allow(non_upper_case_globals, unused_attributes)]
        #[test_case]
        static #struct_ident: X86TestFn = X86TestFn {
            name: #test_name,
            ignore: false,
            identity_map: true,
            physical_memory: #physical_memory_tuple,
            ioport_reads: #ioport_reads_tuple,
            should_panic: #should_panic,
            testfn: x86test::StaticTestFn(|| {
                #fn_ident();
                // Tell our "hypervisor" that we finished the test
                unsafe { x86::io::outw(0xf4, 0x00); }
            })
        };

        // Suppress unused attribute #[should_panic] warning (XXX: there is probably a better way to do this)
        #[allow(unused_attributes)]
        #input_fn
    };

    ast.into()
}
