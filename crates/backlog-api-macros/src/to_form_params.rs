use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, Ident, Lit, Meta, Result, Type,
    spanned::Spanned,
};

/// Configuration for how a field should be serialized to form parameters
#[derive(Debug, Default)]
struct FieldConfig {
    /// Skip this field during serialization
    skip: bool,
    /// Custom API field name (defaults to camelCase conversion of field name)
    name: Option<String>,
    /// Treat as array parameter (adds [] suffix)
    is_array: bool,
    /// Date format specification for DateTime<Utc> fields
    date_format: Option<String>,
}

/// Main entry point for the ToFormParams derive macro
pub fn expand_derive_to_form_params(input: DeriveInput) -> Result<TokenStream> {
    let struct_name = &input.ident;

    let data_struct = match &input.data {
        Data::Struct(data_struct) => data_struct,
        _ => {
            return Err(Error::new(
                input.span(),
                "ToFormParams can only be derived for structs",
            ));
        }
    };

    let field_serializations = generate_field_serializations(data_struct)?;

    Ok(quote! {
        impl From<&#struct_name> for Vec<(String, String)> {
            fn from(params: &#struct_name) -> Self {
                let mut __form_params = Vec::new();
                #(#field_serializations)*
                __form_params
            }
        }

        impl #struct_name {
            pub fn to_form(&self) -> Vec<(String, String)> {
                self.into()
            }
        }
    })
}

/// Generate serialization code for all fields in the struct
fn generate_field_serializations(data_struct: &DataStruct) -> Result<Vec<TokenStream>> {
    match &data_struct.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(generate_field_serialization)
            .collect(),
        Fields::Unnamed(_) => Err(Error::new(
            data_struct.fields.span(),
            "ToFormParams does not support tuple structs",
        )),
        Fields::Unit => Ok(vec![]),
    }
}

/// Generate serialization code for a single field
fn generate_field_serialization(field: &Field) -> Result<TokenStream> {
    let field_name = field
        .ident
        .as_ref()
        .ok_or_else(|| Error::new(field.span(), "Field must have a name"))?;

    let config = parse_field_attributes(&field.attrs)?;

    // Skip this field if configured
    if config.skip {
        return Ok(quote! {});
    }

    let api_field_name = config
        .name
        .clone()
        .unwrap_or_else(|| snake_to_camel_case(&field_name.to_string()));

    let field_type = &field.ty;

    if config.is_array {
        generate_array_field_serialization(field_name, &api_field_name, field_type, &config)
    } else if is_option_type(field_type) {
        generate_optional_field_serialization(field_name, &api_field_name, field_type, &config)
    } else {
        generate_required_field_serialization(field_name, &api_field_name, field_type, &config)
    }
}

/// Generate serialization for required fields
fn generate_required_field_serialization(
    field_name: &Ident,
    api_name: &str,
    _field_type: &Type,
    config: &FieldConfig,
) -> Result<TokenStream> {
    if let Some(date_format) = &config.date_format {
        // Special handling for DateTime<Utc> fields with custom format
        Ok(quote! {
            __form_params.push((#api_name.to_string(), params.#field_name.format(#date_format).to_string()));
        })
    } else {
        Ok(quote! {
            __form_params.push((#api_name.to_string(), params.#field_name.to_string()));
        })
    }
}

/// Generate serialization for optional fields
fn generate_optional_field_serialization(
    field_name: &Ident,
    api_name: &str,
    _field_type: &Type,
    config: &FieldConfig,
) -> Result<TokenStream> {
    if let Some(date_format) = &config.date_format {
        // Special handling for Option<DateTime<Utc>> fields with custom format
        Ok(quote! {
            if let Some(ref __value) = params.#field_name {
                __form_params.push((#api_name.to_string(), __value.format(#date_format).to_string()));
            }
        })
    } else {
        Ok(quote! {
            if let Some(ref __value) = params.#field_name {
                __form_params.push((#api_name.to_string(), __value.to_string()));
            }
        })
    }
}

/// Generate serialization for array fields
fn generate_array_field_serialization(
    field_name: &Ident,
    api_name: &str,
    field_type: &Type,
    _config: &FieldConfig,
) -> Result<TokenStream> {
    let array_key = format!("{api_name}[]");

    if is_option_type(field_type) {
        // Optional array: Option<Vec<T>>
        Ok(quote! {
            if let Some(ref __values) = params.#field_name {
                for __value in __values {
                    __form_params.push((#array_key.to_string(), __value.to_string()));
                }
            }
        })
    } else {
        // Required array: Vec<T>
        Ok(quote! {
            for __value in &params.#field_name {
                __form_params.push((#array_key.to_string(), __value.to_string()));
            }
        })
    }
}

/// Parse form attributes from a field
fn parse_field_attributes(attrs: &[Attribute]) -> Result<FieldConfig> {
    let mut config = FieldConfig::default();

    for attr in attrs {
        if !attr.path().is_ident("form") {
            continue;
        }

        match &attr.meta {
            Meta::Path(_) => {
                // #[form] without arguments - return error
                return Err(Error::new(
                    attr.span(),
                    "#[form] attribute requires arguments. Use #[form(skip)], #[form(name = \"...\")], #[form(array)], or #[form(date_format = \"...\")]",
                ));
            }
            Meta::List(meta_list) => {
                // #[form(skip, name = "customName", array)]
                meta_list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        config.skip = true;
                        Ok(())
                    } else if meta.path.is_ident("array") {
                        config.is_array = true;
                        Ok(())
                    } else if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        match value.parse::<Lit>()? {
                            Lit::Str(lit_str) => {
                                config.name = Some(lit_str.value());
                                Ok(())
                            }
                            _ => Err(meta.error("name attribute must be a string literal")),
                        }
                    } else if meta.path.is_ident("date_format") {
                        let value = meta.value()?;
                        match value.parse::<Lit>()? {
                            Lit::Str(lit_str) => {
                                config.date_format = Some(lit_str.value());
                                Ok(())
                            }
                            _ => Err(meta.error("date_format attribute must be a string literal")),
                        }
                    } else {
                        Err(meta.error("unsupported form attribute"))
                    }
                })?;
            }
            Meta::NameValue(meta_name_value) => {
                // #[form = "value"] - not supported
                return Err(Error::new(
                    meta_name_value.span(),
                    "form attribute does not support this syntax",
                ));
            }
        }
    }

    Ok(config)
}

/// Check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident == "Option")
            .unwrap_or(false),
        _ => false,
    }
}

/// Convert snake_case to camelCase
fn snake_to_camel_case(snake_str: &str) -> String {
    let mut camel = String::new();
    let mut capitalize_next = false;

    for ch in snake_str.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            camel.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            camel.push(ch);
        }
    }

    camel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_to_camel_case() {
        assert_eq!(snake_to_camel_case("user_id"), "userId");
        assert_eq!(snake_to_camel_case("notified_user_ids"), "notifiedUserIds");
        assert_eq!(snake_to_camel_case("content"), "content");
        assert_eq!(snake_to_camel_case("issue_id"), "issueId");
    }
}
