/// The `Optionalize` macro generates a new struct with optional fields.
///
/// For any struct that derives `Optionalize`, the macro will generate a new struct
/// with the same name appended by `Optional`. Each field in the original struct
/// is transformed:
///
/// - If a field is of type `Option<T>`, it remains `Option<T>`.
/// - If a field is of type `T`, it becomes `Option<T>`.
///
/// # Example
///
/// ```rust
/// use optionalize_macro::{Optionalize};
/// use optionalize_core::OptionalizeTrait;
///
/// #[derive(Optionalize)]
/// pub struct MyStruct {
///     pub id: i32,
///     pub name: String,
///     pub description: Option<String>,
/// }
///
/// // The generated struct will look like:
/// let test = MyStruct::Optional {
///     id: Some(1i32),
///     name: Some("Name".to_string()),
///     description: Some("Test Description".to_string())
/// };
/// ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Type, TypePath, Meta};

#[proc_macro_derive(Optionalize, attributes(optionalize_ignore))]
pub fn derive_optionalize(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the struct name
    let struct_name = input.ident.clone();

    // Generate a new name for the "optionalized" struct
    let optional_struct_name = syn::Ident::new(&format!("{}Optional", struct_name), struct_name.span());

    // Build the fields for the new struct
    let fields = if let Data::Struct(data_struct) = input.data {
        data_struct.fields
    } else {
        // Only work with structs
        return syn::Error::new_spanned(input, "Optionalize can only be used on structs")
            .to_compile_error()
            .into();
    };
    let fields = fields.into_iter().map(|field| {
        let mut is_optional = false;
        if let Type::Path(type_path) = &field.ty {
            is_optional = type_path.path.segments.last().map(|f| f.ident == "Option").unwrap_or(false);
        }
        for attr in &field.attrs {
            match &attr.meta {
                Meta::Path(path) if path.is_ident("optionalize_ignore") => {
                    return (field, true, is_optional);
                }
                _ => {}
            }
        }
        return (field, false, is_optional);
    });
    // Create fields with Option types
    let optional_fields = fields.clone().map(|(field, is_ignored, is_optional)| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        match (is_ignored, is_optional) {
            (false, false) => quote! { #field_name: Option<#field_type> }, // Option<T>
            (false, true) => quote! { #field_name: #field_type }, // Option<T>
            (true, false) => quote! { #field_name: #field_type }, // T
            (true, true) => quote! { #field_name: #field_type}, // Option<T>
        }
    });

    let to_active_model_fields = fields.map(|(field, is_ignored, is_optional)| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        match (is_ignored, is_optional) {
            (true, false) => {
                quote! {
                    #field_name: sea_orm::ActiveValue::Unchanged(self.#field_name)
                }
            },
            (false, false) => {
                quote! {
                    #field_name: match self.#field_name {
                        Some(value) => sea_orm::ActiveValue::Set(value),
                        None => sea_orm::ActiveValue::NotSet
                    }
                }
            },
            (_, _) => {
                quote! {
                    #field_name: match self.#field_name {
                        Some(value) => sea_orm::ActiveValue::Set(Some(value)),
                        None => sea_orm::ActiveValue::NotSet
                    }
                }
            },
        }
    });

    // Generate the output tokens
    let expanded = quote! {

        #[derive(Debug, Deserialize)]
        pub struct #optional_struct_name {
            #( #optional_fields, )*
        }

        impl #optional_struct_name {
            pub fn to_active(self) -> ActiveModel {
                ActiveModel {
                    #( #to_active_model_fields, )*
                }
            }
        }

        impl OptionalizeTrait for #struct_name {
            type Optional = #optional_struct_name;
        }
    };

    TokenStream::from(expanded)
}
