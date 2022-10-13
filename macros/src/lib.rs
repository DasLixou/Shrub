use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ItemData)]
pub fn item_data_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics shrub::ItemData for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
    gen.into()
}
