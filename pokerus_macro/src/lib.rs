use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use syn::parse_macro_input;
mod metang {
    use syn::ExprLit;
    use syn::parse::{Parse, ParseStream};

    pub struct MetangEnumInput {
        pub file_path: String,
        pub repr_type: syn::Type,
        pub enum_name: syn::Ident
    }

    impl Parse for MetangEnumInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let file_expr: syn::Expr = input.parse()?;
            let file_path = match file_expr {
                syn::Expr::Lit(ExprLit { lit: syn::Lit::Str(path), .. }) => path.value(),
                _ => return Err(input.error("Expected string literal for file_path"))
            };

            input.parse::<syn::Token![,]>()?;

            let repr_type: syn::Type = input.parse()?;

            input.parse::<syn::Token![,]>()?;

            let enum_name = input.parse()?;

            Ok(MetangEnumInput { file_path, repr_type, enum_name })
        }
    }
}

fn read_meta_file(file_path: String) -> File {
    let path = if let Some(manifest_dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        let mut buf = PathBuf::from(manifest_dir);
        buf.push(file_path);
        buf
    } else {
        file_path.into()
    };

    let file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file {}: {}", path.display(), e),
    };

    file
}

#[proc_macro]
pub fn metang_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as metang::MetangEnumInput);

    let repr_type = &input.repr_type;
    let enum_name = &input.enum_name;

    let meta_file = read_meta_file(input.file_path);

    let reader = io::BufReader::new(meta_file);

    let mut current_value: i64 = 0;
    let step: i64 = 1;

    let mut variants = vec![];
    let mut matches = vec![];
    let mut canonical = vec![];

    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (name, value) = if let Some((name, value)) = line.split_once('=') {
            let name = name.trim();
            let value = value.trim().parse::<i64>().expect("Invalid value");
            current_value = value;
            (name, value)
        }
        else {
            current_value += step;
            (line, current_value)
        };

        let name = syn::Ident::new(name, proc_macro2::Span::call_site());
        let literal = syn::LitInt::new(&value.to_string(), proc_macro2::Span::call_site());
        variants.push(quote::quote! { #name = #literal });
        if !canonical.contains(&value) {
            canonical.push(value);
            matches.push(quote::quote! { #literal => #enum_name::#name, });
        }
    }

    let quoted = quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(#repr_type)]
        pub enum #enum_name {
            #(#variants),*
        }

        impl From<#repr_type> for #enum_name {
            fn from(value: #repr_type) -> Self {
                match value {
                    #(#matches)*
                    _ => unreachable!(),
                }
            }
        }

        impl Into<#repr_type> for #enum_name {
            fn into(self) -> #repr_type {
                self as #repr_type
            }
        }
    };

    println!("{}", quoted);

    proc_macro::TokenStream::from(quoted)
}