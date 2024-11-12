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

    // Create fields with Option types
    let optional_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        let mut ignore_field = false;

        for attr in &field.attrs {
            match &attr.meta {
                Meta::Path(path) if path.is_ident("optionalize_ignore") => {
                    ignore_field = true; // Mark this field to be ignored
                },
                _ => {}
            }
        }

        // Check if the field is already an Option<T>
        if let Type::Path(TypePath { path, .. }) = field_type {
            if ignore_field || path.segments.last().map(|s| s.ident == "Option").unwrap_or(false) {
                // Field is already an Option<T>, keep it as is
                quote! { #field_name: #field_type }
            } else if type_implements_optionalize(field_type) {
                // If the field type implements OptionalizeTrait, use Option<FieldType::Optional>
                quote! { #field_name: Option<<#field_type as OptionalizeTrait>::Optional> }
            } else {
                // Wrap the field type in Option<T>
                quote! { #field_name: Option<#field_type> }
            }
        } else {
            // Wrap non-path types (like tuples) in Option<T>
            quote! { #field_name: Option<#field_type> }
        }
    });

    // Generate the output tokens
    let expanded = quote! {

        #[derive(Debug, Deserialize)]
        pub struct #optional_struct_name {
            #( #optional_fields, )*
        }
        impl OptionalizeTrait for #struct_name {
            type Optional = #optional_struct_name;
        }
    };

    TokenStream::from(expanded)
}

fn type_implements_optionalize(field_type: &Type) -> bool {
    match field_type {
        Type::Path(TypePath { path, .. }) => {
            // Primitive types and standard library types that we assume don't implement `OptionalizeTrait`
            let segment = path.segments.last().unwrap().ident.to_string();
            matches!(
                segment.as_str(),
                "i32" | "i64" | "u32" | "u64" | "f32" | "f64" | "String" | "bool"
            ) == false
        }
        _ => false,
    }
}

