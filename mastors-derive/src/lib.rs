#[macro_use] extern crate quote;
use proc_macro::TokenStream;

const IDENT_MASTORS: &str = "mastors";
const IDENT_CONNECTION: &str = "connection";
const IDENT_PATH_PARAM: &str = "path_param";
const IDENT_AUTHORIZATION: &str = "authorization";
const IDENT_METHOD_PARAMS: &str = "method_params";

#[proc_macro_derive(Method, attributes(mastors, method_params))]
pub fn derive_method(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let (http_method, entity, endpoint, response_header) = get_method_params(&input.attrs)
        .expect("An attribute `method_params` is required for deriving Method");

    let connection_field = get_field_name_with_attribute(&input.data, IDENT_CONNECTION)
        .expect("An attribute `mastors(connection)` is required for deriving Method");

    let fn_path = match get_field_name_with_attribute(&input.data, IDENT_PATH_PARAM) {
        Some(path_param_field) => quote! {
            fn path(&self) -> String {
                Self::ENDPOINT.replace("_PATH_PARAM_", self.#path_param_field.as_str())
            }
        },
        None => quote! {
            fn path(&self) -> String {
                Self::ENDPOINT.to_owned()
            }
        },
    };

    let fn_authorization = match get_field_name_with_attribute(&input.data, IDENT_AUTHORIZATION) {
        Some(auth_field) => quote! {
            fn authorization(&'a self) -> Option<&'a str> {
                if self.#auth_field {
                    Some(self.#connection_field.access_token())
                } else {
                    None
                }
            }
        },
        None => quote! {
            fn authorization(&'a self) -> Option<&'a str> {
                None
            }
        }
    };

    let fn_send_internal_impl = if http_method == "GET" {
        quote! { Ok(self.get()?) }
    } else if http_method == "POST" {
        quote! { Ok(self.post()?) }
    } else if http_method == "DELETE" {
        quote! { Ok(self.delete()?) }
    } else if http_method == "PUT" {
        quote! { Ok(self.put()?) }
    } else {
        panic!("Unexpected HTTP method");
    };

    let trait_impl = match response_header {
        Some(response_header) => { quote! { 
            impl<'a> crate::syncronous::methods::private::MethodInternalWithRespHeader<'a, #entity> for #name<'a> {
                const RESPONSE_HEADER_NAME: &'a str = #response_header;

                fn send_internal(&self) -> crate::Result<(crate::entities::PageNavigation, #entity)> {
                    #fn_send_internal_impl
                }
            }
        }},
        None => { quote! { 
            impl<'a> crate::syncronous::methods::private::MethodInternalWithoutRespHeader<'a, #entity> for #name<'a> {
                fn send_internal(&self) -> crate::Result<#entity> {
                    #fn_send_internal_impl
                }
            }
        }},
    };

    TokenStream::from(quote! {
        impl<'a> crate::syncronous::methods::private::MethodInternal<'a, #entity> for #name<'a> {
            const ENDPOINT: &'a str = #endpoint;

            fn connection(&self) -> &Connection {
                self.#connection_field
            }

            #fn_path

            #fn_authorization
        }

        #trait_impl
    })
}

fn get_field_name_with_attribute<'a>(data: &'a syn::Data, attr: &str) -> Option<&'a syn::Ident> {
    if let Some(field) = get_field_with_attribute(data, attr) {
        field.ident.as_ref()
    } else {
        None
    }
}

fn get_field_with_attribute<'a>(data: &'a syn::Data, attr: &str) -> Option<&'a syn::Field> {
    let mut result: Option<&'a syn::Field> = None;

    let data_struct = match data {
        syn::Data::Struct(ref data_struct) => data_struct,
        _ => return None,
    };
    let fields_named = match data_struct.fields {
        syn::Fields::Named(ref fields_named) => fields_named,
        _ => return None,
    };

    for field in fields_named.named.iter() {
        for attribute in field.attrs.iter() {
            let meta = match attribute.parse_meta() {
                Ok(meta) => meta,
                Err(_) => continue,
            };
            let meta_list = match meta {
                syn::Meta::List(meta_list) => meta_list,
                _ => continue,
            };
            if let Some(ident) = meta_list.path.get_ident() {
                if ident == IDENT_MASTORS {
                    if let Some(syn::NestedMeta::Meta(nested)) = &meta_list.nested.first() {
                        if let Some(ident) = nested.path().get_ident() {
                            if ident == attr {
                                if result.is_some() {
                                    /* Need the nightly build.
                                    ident.span().unwrap().error("Attribute `".to_owned() + attr + "` duplicated").emit();
                                    */
                                    panic!("Attribute `".to_owned() + attr + "` duplicated")
                                } else {
                                    result = Some(field);
                                }
                            }
                        }
                    } else {
                        /* Need the nightly build.
                        ident.span().unwrap().error("An attribute mastors requires 1 argument").emit();
                        */
                        panic!("An attribute mastors requires 1 argument");
                    }
                }
            }
        }
    }
    result
}

type MethodParams = (syn::Ident, syn::Ident, syn::Lit, Option<syn::Lit>);

fn get_method_params(attrs: &[syn::Attribute]) -> Option<MethodParams> {
    let mut result: Option<MethodParams> = None;
    let (mut http_method, mut entity, mut endpoint, mut response_header);

    for attr in attrs.iter() {
        let meta = match attr.parse_meta() {
            Ok(meta) => meta,
            Err(_) => continue,
        };
        let meta_list = match meta {
            syn::Meta::List(meta_list) => meta_list,
            _ => continue,
        };

        if let Some(ident) = meta_list.path.get_ident() {
            if ident == IDENT_METHOD_PARAMS {
                let mut params = meta_list.nested.iter();

                // First arg is a HTTP Method of this method.
                if let Some(syn::NestedMeta::Meta(meta)) = params.next() {
                    if let Some(ident) = meta.path().get_ident() {
                        if is_http_method(ident) {
                            http_method = ident.clone();
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }

                // Second arg is the entity that is returned by this method.
                if let Some(syn::NestedMeta::Meta(meta)) = params.next() {
                    if let Some(ident) = meta.path().get_ident() {
                        entity = ident.clone();
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }

                // Third arg is api endpoint of this method.
                if let Some(syn::NestedMeta::Lit(lit)) = params.next() {
                    endpoint = lit.clone();
                } else {
                    return None;
                }

                // Fourth arg is the name of HTTP Response header to capture.
                if let Some(syn::NestedMeta::Lit(lit)) = params.next() {
                    response_header = Some(lit.clone());
                } else {
                    response_header = None;
                }

                result = Some((http_method, entity, endpoint, response_header));
            }
        };
    }

    result
}

fn is_http_method(ident: &syn::Ident) -> bool {
    ident == "GET" ||
    ident == "POST" ||
    ident == "PUT" ||
    ident == "DELETE"
}
