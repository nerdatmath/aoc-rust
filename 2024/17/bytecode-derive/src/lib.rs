use syn::spanned::Spanned;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(bytecode))]
struct ByteCode {
    opcode: u8,
}

fn bytecode_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // parse
    let mut input = syn::parse2::<syn::DeriveInput>(item)?;

    // extract field attributes
    let mut new_matchers = Vec::new();
    let mut opcode_matchers = Vec::new();
    let mut operand_matchers = Vec::new();
    if let syn::Data::Enum(e) = &mut input.data {
        for variant in e.variants.iter_mut() {
            if variant.fields.len() > 1 {
                panic!("too many fields in bytecode variant {}", variant.ident);
            }
            let ByteCode { opcode } = deluxe::extract_attributes(variant)?;
            let span = variant.span();
            let ident = &variant.ident;
            let members = variant.fields.members();
            new_matchers.push(quote::quote_spanned! {span=>
                #opcode => Self::#ident { #( #members: operand.into() ),* }
            });
            let members = variant.fields.members();
            opcode_matchers.push(quote::quote_spanned! {span=>
                Self::#ident { #( #members: _ ),* } => #opcode
            });
            let members = variant.fields.members();
            if variant.fields.len() == 1 {
                operand_matchers.push(quote::quote_spanned! {span=>
                    Self::#ident { #( #members: operand ),* } => operand.into()
                });
            }
        }
    }

    // define impl variables
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    // generate
    Ok(quote::quote! {
        impl #impl_generics bytecode::ByteCode for #ident #type_generics #where_clause {
            fn new(opcode: u8, operand: u8) -> Self {
                match opcode {
                    #( #new_matchers ),*,
                    _ => panic!("invalid opcode"),
                }
            }
            fn opcode(&self) -> u8 {
                match *self {
                    #( #opcode_matchers ),*,
                }
            }
            fn operand(&self) -> u8 {
                match *self {
                    #( #operand_matchers ),*,
                    _ => 0,
                }
            }
        }

        impl #impl_generics From<(u8, u8)> for #ident #type_generics #where_clause {
            fn from((opcode, operand): (u8, u8)) -> Self {
                return Self::new(opcode, operand)
            }
        }

        impl #impl_generics From<#ident> for (u8, u8) #type_generics #where_clause {
            fn from(this: #ident) -> (u8, u8) {
                return (this.opcode(), this.operand())
            }
        }
    })
}

#[proc_macro_derive(ByteCode, attributes(bytecode))]
pub fn bytecode_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bytecode_derive_macro2(item.into()).unwrap().into()
}
