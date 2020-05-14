use proc_macro::{TokenStream};
use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput, GenericParam, TypeParam, Ident, parse_str};
use quote::quote;

#[proc_macro_derive(TQuery)]
pub fn gen_tquery_impl(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident.clone();
  let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

  let impls = input.generics.type_params().enumerate().map(|(i, ty)| {
    let tnum = Ident::new(&format!("U{}", i), Span::call_site());
    let tname = &ty.ident;
    let newt_param: TypeParam = parse_str("NewT").unwrap();

    let mut generics_newt = input.generics.clone();
    generics_newt.params.push(GenericParam::Type(newt_param.clone()));
    let (impl_generics_newt, _, _) = generics_newt.split_for_impl();

    let mut generics_newt_2 = input.generics.clone();
    for (j, param) in generics_newt_2.type_params_mut().enumerate() {
      if i == j {
        *param = newt_param.clone();
      }
    }
    let (_, ty_generics_newt, _) = generics_newt_2.split_for_impl();

    let alias = Ident::new(&format!("T{}", tname), Span::call_site());

    quote! {
      impl #impl_generics ::tquery::ComputeGetType<::typenum::#tnum> for #name #ty_generics {
        type Output = #tname;
      }

      impl #impl_generics_newt ::tquery::ComputeSetType<::typenum::#tnum, NewT> for #name #ty_generics {
        type Output = #name #ty_generics_newt;
      }

      pub type #alias = ::typenum::#tnum;
    }
  }).collect::<Vec<_>>();

  proc_macro::TokenStream::from(quote! { #(#impls)* })
}
