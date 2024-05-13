use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{parse_quote, ItemFn, Meta, Result, Type};

use crate::utils::{bail, error};

pub fn attr_main(args: TokenStream2, item: TokenStream2) -> Result<TokenStream2> {
    const INVALID: &str = "expected `async fn main() -> AnyResult<()>`";

    let mut main = syn::parse2::<ItemFn>(item).map_err(|err| error!(err.span(), INVALID))?;

    if main.sig.ident != "main" {
        bail!(main.sig.ident.span(), INVALID);
    }

    if main.sig.asyncness.is_none() {
        bail!(main.sig.span(), INVALID);
    }

    // TODO: don't require Result
    match &main.sig.output {
        syn::ReturnType::Default => bail!(main.sig.output.span(), INVALID),
        syn::ReturnType::Type(_, ty) => match &**ty {
            Type::Path(path) if *path == parse_quote!(AnyResult<()>) => {}
            _ => bail!(ty.span(), INVALID),
        },
    }

    main.sig.ident = parse_quote!(__main);

    // parse args
    let args = Punctuated::<Meta, syn::Token![,]>::parse_terminated.parse2(args)?;
    let mut args_ty = None;
    for meta in args {
        if let Meta::NameValue(kv) = meta {
            if kv.path.is_ident("args") {
                args_ty = Some(kv.value);
            }
        }
    }

    let init = match args_ty {
        Some(ty) => quote! {
            let args = stud::bin::init_with_args::<#ty>(std::module_path!()).await?;
            __main(args).await?;
        },
        None => quote! {
            stud::bin::init(std::module_path!()).await?;
            __main().await?;
        },
    };

    Ok(quote! {
        #main
        async fn _main() -> stud::error::AnyResult<()> {
            #init
            Ok(())
        }
        fn main() {
            if let Err(e) = stud::rt::run(_main()) {
                stud::log::error!("\n{e:?}")
            }
        }
    })
}
