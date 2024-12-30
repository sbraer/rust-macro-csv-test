extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Csv)]
pub fn to_csv_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            Fields::Unnamed(_) | Fields::Unit => {
                panic!("Expected a struct with named fields")
            }
        },
        _ => panic!("This macro only supports structs"),
    };

    let fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = &field.ident;
            let ty = &field.ty;
            (name, ty)
        })
        .collect();

    let self_field_access_input = fields.iter().map(|(name, _)| {
        let field_name = name.as_ref().unwrap();
        quote! { #field_name }
    });

    let format_string = fields.iter().map(|_| "{}").collect::<Vec<_>>().join(",");

    let field_names: Vec<_> = fields.iter().map(|(name, _)| *name).collect();
    let expanded = quote! {
        impl #struct_name {
            pub fn to_csv_string(&self) -> String {
                format!(#format_string,#(&self.#self_field_access_input),*)
            }

            pub fn new_from_string(txt: &str) -> #struct_name {
                let mut columns = txt.split(',');
                #struct_name {
                    #(
                        #field_names: columns.next().unwrap_or_default().parse().unwrap_or_default(),
                    )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
