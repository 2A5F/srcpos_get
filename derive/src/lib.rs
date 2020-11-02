use proc_macro::TokenStream;
use proc_macro2;
use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput, Ident, LitInt};

/// # Example
/// ```
/// # use srcpos_get::*;
/// #[derive(GetLoc)]
/// struct A {
///    loc: Loc,
/// }
///
/// #[derive(GetLoc)]
/// struct B {
///     #[loc]
///     a: Loc,
/// }
///
/// #[derive(GetLoc)]
/// struct C(Loc);
///
/// #[derive(GetLoc)]
/// struct D(u8, #[loc] Loc);
///
/// #[derive(GetLoc)]
/// enum E {
///     A(Loc),
///     B(u8, #[loc] Loc),
///     C {
///         loc: Loc,
///     },
///     D {
///         #[loc]
///         a: Loc,
///         _b: u8,
///     },
/// }
/// ```
#[proc_macro_derive(GetLoc, attributes(loc))]
pub fn derive_get_loc(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        syn::Data::Struct(v) => match v.fields {
            syn::Fields::Named(v) => {
                let mut has_loc = false;
                let mut loc_id: Option<&Ident> = None;
                for f in v.named.iter() {
                    #[allow(unused_assignments)]
                    if f.ident.as_ref().map(|id| id == "loc").unwrap_or(false) {
                        has_loc = true;
                        loc_id = f.ident.as_ref();
                        break;
                    }
                    for attr in f.attrs.iter() {
                        let path = &attr.path;
                        if path.get_ident().map(|id| id == "loc").unwrap_or(false) {
                            if !has_loc && loc_id.is_some() {
                                return syn::Error::new(
                                    Span::call_site(),
                                    "Cannot have multiple loc",
                                )
                                .to_compile_error()
                                .into();
                            }
                            loc_id = f.ident.as_ref();
                            if loc_id.is_none() {
                                return syn::Error::new(Span::call_site(), "Field has no name")
                                    .to_compile_error()
                                    .into();
                            }
                        }
                    }
                }
                if loc_id.is_none() {
                    return syn::Error::new(Span::call_site(), "Not found field loc")
                        .to_compile_error()
                        .into();
                }
                let loc_id = loc_id.unwrap();
                let imp = quote! {
                    impl #impl_generics ::srcpos_get::GetLoc for #name #ty_generics #where_clause {
                        fn loc(&self) -> ::srcpos_get::Loc {
                            ::srcpos_get::GetLoc::loc(&self.#loc_id)
                        }
                    }
                };
                return imp.into();
            }
            syn::Fields::Unnamed(v) => {
                let mut loc_id: Option<usize> = None;
                for (i, f) in v.unnamed.iter().enumerate() {
                    for attr in f.attrs.iter() {
                        let path = &attr.path;
                        if path.get_ident().map(|id| id == "loc").unwrap_or(false) {
                            if loc_id.is_some() {
                                return syn::Error::new(
                                    Span::call_site(),
                                    "Cannot have multiple loc",
                                )
                                .to_compile_error()
                                .into();
                            }
                            loc_id = Some(i);
                        }
                    }
                }
                if v.unnamed.len() == 0 {
                    return syn::Error::new(Span::call_site(), "There is nothing to get")
                        .to_compile_error()
                        .into();
                }
                if v.unnamed.len() == 1 && loc_id.is_none() {
                    loc_id = Some(0);
                }
                if loc_id.is_none() {
                    return syn::Error::new(Span::call_site(), "Not found field loc")
                        .to_compile_error()
                        .into();
                }
                let loc_id = loc_id.unwrap().to_string();
                let loc_id = LitInt::new(loc_id.as_str(), Span::call_site());
                let imp = quote! {
                    impl #impl_generics ::srcpos_get::GetLoc for #name #ty_generics #where_clause {
                        fn loc(&self) -> ::srcpos_get::Loc {
                            ::srcpos_get::GetLoc::loc(&self.#loc_id)
                        }
                    }
                };
                return imp.into();
            }
            syn::Fields::Unit => {
                return syn::Error::new(Span::call_site(), "There is nothing to get")
                    .to_compile_error()
                    .into();
            }
        },
        syn::Data::Enum(v) => {
            let mut vimps = vec![];
            if v.variants.len() == 0 {
                return syn::Error::new(Span::call_site(), "There is nothing to get")
                    .to_compile_error()
                    .into();
            }
            for variant in v.variants.iter() {
                match &variant.fields {
                    syn::Fields::Named(v) => {
                        let mut has_loc = false;
                        let mut loc_id: Option<&Ident> = None;
                        for f in v.named.iter() {
                            #[allow(unused_assignments)]
                            if f.ident.as_ref().map(|id| id == "loc").unwrap_or(false) {
                                has_loc = true;
                                loc_id = f.ident.as_ref();
                                break;
                            }
                            for attr in f.attrs.iter() {
                                let path = &attr.path;
                                if path.get_ident().map(|id| id == "loc").unwrap_or(false) {
                                    if !has_loc && loc_id.is_some() {
                                        return syn::Error::new(
                                            variant.ident.span(),
                                            "[GetLoc] Cannot have multiple loc",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                    loc_id = f.ident.as_ref();
                                    if loc_id.is_none() {
                                        return syn::Error::new(
                                            variant.ident.span(),
                                            "[GetLoc] Field has no name",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                }
                            }
                        }
                        if loc_id.is_none() {
                            return syn::Error::new(
                                variant.ident.span(),
                                "[GetLoc] Not found field loc",
                            )
                            .to_compile_error()
                            .into();
                        }
                        let loc_id = loc_id.unwrap();
                        let vname = &variant.ident;
                        let imp = quote_spanned! { variant.ident.span() => Self::#vname { #loc_id, .. } => ::srcpos_get::GetLoc::loc(#loc_id) };
                        vimps.push(imp);
                    }
                    syn::Fields::Unnamed(v) => {
                        let mut loc_id: Option<usize> = None;
                        for (i, f) in v.unnamed.iter().enumerate() {
                            for attr in f.attrs.iter() {
                                let path = &attr.path;
                                if path.get_ident().map(|id| id == "loc").unwrap_or(false) {
                                    if loc_id.is_some() {
                                        return syn::Error::new(
                                            variant.ident.span(),
                                            "[GetLoc] Cannot have multiple loc",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                    loc_id = Some(i);
                                }
                            }
                        }
                        if v.unnamed.len() == 0 {
                            return syn::Error::new(
                                variant.ident.span(),
                                "[GetLoc] There is nothing to get",
                            )
                            .to_compile_error()
                            .into();
                        }
                        if v.unnamed.len() == 1 && loc_id.is_none() {
                            loc_id = Some(0);
                        }
                        if loc_id.is_none() {
                            return syn::Error::new(
                                variant.ident.span(),
                                "[GetLoc] Not found field loc",
                            )
                            .to_compile_error()
                            .into();
                        }
                        let loc_id = loc_id.unwrap();
                        let ids = (0..(variant.fields.len())).into_iter().map(|i| {
                            if i != loc_id {
                                format_ident!("_")
                            } else {
                                format_ident!("v{}", i)
                            }
                        });
                        let loc_id = format_ident!("v{}", loc_id);
                        let vname = &variant.ident;
                        let imp = quote_spanned! { variant.ident.span() => Self::#vname(#(#ids),*) => ::srcpos_get::GetLoc::loc(#loc_id) };
                        vimps.push(imp);
                    }
                    syn::Fields::Unit => {
                        return syn::Error::new(
                            variant.ident.span(),
                            "[GetLoc] There is nothing to get",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }
            let imp = quote! {
                impl #impl_generics ::srcpos_get::GetLoc for #name #ty_generics #where_clause {
                    fn loc(&self) -> ::srcpos_get::Loc {
                        match self {
                            #(#vimps),*
                        }
                    }
                }
            };
            return imp.into();
        }
        syn::Data::Union(_) => {
            return syn::Error::new(Span::call_site(), "Does not support union")
                .to_compile_error()
                .into();
        }
    }
}

/// # Example
/// ```
/// # use srcpos_get::*;
/// #[derive(GetPos)]
/// struct A {
///    pos: Pos,
/// }
///
/// #[derive(GetPos)]
/// struct B {
///     #[pos]
///     a: Pos,
/// }
///
/// #[derive(GetPos)]
/// struct C(Pos);
///
/// #[derive(GetPos)]
/// struct D(u8, #[pos] Pos);
///
/// #[derive(GetPos)]
/// enum E {
///     A(Pos),
///     B(u8, #[pos] Pos),
///     C {
///         pos: Pos,
///     },
///     D {
///         #[pos]
///         a: Pos,
///         _b: u8,
///     },
/// }
/// ```
#[proc_macro_derive(GetPos, attributes(pos))]
pub fn derive_get_pos(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        syn::Data::Struct(v) => match v.fields {
            syn::Fields::Named(v) => {
                let mut has_loc = false;
                let mut loc_id: Option<&Ident> = None;
                for f in v.named.iter() {
                    #[allow(unused_assignments)]
                    if f.ident.as_ref().map(|id| id == "pos").unwrap_or(false) {
                        has_loc = true;
                        loc_id = f.ident.as_ref();
                        break;
                    }
                    for attr in f.attrs.iter() {
                        let path = &attr.path;
                        if path.get_ident().map(|id| id == "pos").unwrap_or(false) {
                            if !has_loc && loc_id.is_some() {
                                return syn::Error::new(
                                    Span::call_site(),
                                    "Cannot have multiple pos",
                                )
                                .to_compile_error()
                                .into();
                            }
                            loc_id = f.ident.as_ref();
                            if loc_id.is_none() {
                                return syn::Error::new(Span::call_site(), "Field has no name")
                                    .to_compile_error()
                                    .into();
                            }
                        }
                    }
                }
                if loc_id.is_none() {
                    return syn::Error::new(Span::call_site(), "Not found field pos")
                        .to_compile_error()
                        .into();
                }
                let loc_id = loc_id.unwrap();
                let imp = quote! {
                    impl #impl_generics ::srcpos_get::GetPos for #name #ty_generics #where_clause {
                        fn pos(&self) -> ::srcpos_get::Pos {
                            ::srcpos_get::GetPos::pos(&self.#loc_id)
                        }
                    }
                };
                return imp.into();
            }
            syn::Fields::Unnamed(v) => {
                let mut loc_id: Option<usize> = None;
                for (i, f) in v.unnamed.iter().enumerate() {
                    for attr in f.attrs.iter() {
                        let path = &attr.path;
                        if path.get_ident().map(|id| id == "pos").unwrap_or(false) {
                            if loc_id.is_some() {
                                return syn::Error::new(
                                    Span::call_site(),
                                    "Cannot have multiple pos",
                                )
                                .to_compile_error()
                                .into();
                            }
                            loc_id = Some(i);
                        }
                    }
                }
                if v.unnamed.len() == 0 {
                    return syn::Error::new(Span::call_site(), "There is nothing to get")
                        .to_compile_error()
                        .into();
                }
                if v.unnamed.len() == 1 && loc_id.is_none() {
                    loc_id = Some(0);
                }
                if loc_id.is_none() {
                    return syn::Error::new(Span::call_site(), "Not found field pos")
                        .to_compile_error()
                        .into();
                }
                let loc_id = loc_id.unwrap().to_string();
                let loc_id = LitInt::new(loc_id.as_str(), Span::call_site());
                let imp = quote! {
                    impl #impl_generics ::srcpos_get::GetPos for #name #ty_generics #where_clause {
                        fn pos(&self) -> ::srcpos_get::Pos {
                            ::srcpos_get::GetPos::pos(&self.#loc_id)
                        }
                    }
                };
                return imp.into();
            }
            syn::Fields::Unit => {
                return syn::Error::new(Span::call_site(), "There is nothing to get")
                    .to_compile_error()
                    .into();
            }
        },
        syn::Data::Enum(v) => {
            let mut vimps = vec![];
            if v.variants.len() == 0 {
                return syn::Error::new(Span::call_site(), "There is nothing to get")
                    .to_compile_error()
                    .into();
            }
            for variant in v.variants.iter() {
                match &variant.fields {
                    syn::Fields::Named(v) => {
                        let mut has_loc = false;
                        let mut loc_id: Option<&Ident> = None;
                        for f in v.named.iter() {
                            #[allow(unused_assignments)]
                            if f.ident.as_ref().map(|id| id == "pos").unwrap_or(false) {
                                has_loc = true;
                                loc_id = f.ident.as_ref();
                                break;
                            }
                            for attr in f.attrs.iter() {
                                let path = &attr.path;
                                if path.get_ident().map(|id| id == "pos").unwrap_or(false) {
                                    if !has_loc && loc_id.is_some() {
                                        return syn::Error::new(
                                            variant.ident.span(),
                                            "[GetPos] Cannot have multiple pos",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                    loc_id = f.ident.as_ref();
                                    if loc_id.is_none() {
                                        return syn::Error::new(
                                            variant.ident.span(),
                                            "[GetPos] Field has no name",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                }
                            }
                        }
                        if loc_id.is_none() {
                            return syn::Error::new(
                                variant.ident.span(),
                                "[GetPos] Not found field pos",
                            )
                            .to_compile_error()
                            .into();
                        }
                        let loc_id = loc_id.unwrap();
                        let vname = &variant.ident;
                        let imp = quote_spanned! { variant.ident.span() => Self::#vname { #loc_id, .. } => ::srcpos_get::GetPos::pos(#loc_id) };
                        vimps.push(imp);
                    }
                    syn::Fields::Unnamed(v) => {
                        let mut loc_id: Option<usize> = None;
                        for (i, f) in v.unnamed.iter().enumerate() {
                            for attr in f.attrs.iter() {
                                let path = &attr.path;
                                if path.get_ident().map(|id| id == "pos").unwrap_or(false) {
                                    if loc_id.is_some() {
                                        return syn::Error::new(
                                            variant.ident.span(),
                                            "[GetPos] Cannot have multiple pos",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                    loc_id = Some(i);
                                }
                            }
                        }
                        if v.unnamed.len() == 0 {
                            return syn::Error::new(
                                variant.ident.span(),
                                "[GetPos] There is nothing to get",
                            )
                            .to_compile_error()
                            .into();
                        }
                        if v.unnamed.len() == 1 && loc_id.is_none() {
                            loc_id = Some(0);
                        }
                        if loc_id.is_none() {
                            return syn::Error::new(
                                variant.ident.span(),
                                "[GetPos] Not found field pos",
                            )
                            .to_compile_error()
                            .into();
                        }
                        let loc_id = loc_id.unwrap();
                        let ids = (0..(variant.fields.len())).into_iter().map(|i| {
                            if i != loc_id {
                                format_ident!("_")
                            } else {
                                format_ident!("v{}", i)
                            }
                        });
                        let loc_id = format_ident!("v{}", loc_id);
                        let vname = &variant.ident;
                        let imp = quote_spanned! { variant.ident.span() => Self::#vname(#(#ids),*) => ::srcpos_get::GetPos::pos(#loc_id) };
                        vimps.push(imp);
                    }
                    syn::Fields::Unit => {
                        return syn::Error::new(
                            variant.ident.span(),
                            "[GetPos] There is nothing to get",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }
            let imp = quote! {
                impl #impl_generics ::srcpos_get::GetPos for #name #ty_generics #where_clause {
                    fn pos(&self) -> ::srcpos_get::Pos {
                        match self {
                            #(#vimps),*
                        }
                    }
                }
            };
            return imp.into();
        }
        syn::Data::Union(_) => {
            return syn::Error::new(Span::call_site(), "Does not support union")
                .to_compile_error()
                .into();
        }
    }
}
