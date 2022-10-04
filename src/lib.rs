#[proc_macro_derive(ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas)]
pub fn derive_impl_error_with_tracing_for_struct_with_get_source_with_get_where_was(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect(
        "ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas syn::parse(input) failed",
    );
    let fields = match ast.data {
        syn::Data::Struct(struct_item) => struct_item.fields,
        _ => panic!(
            "ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas only works on structs"
        ),
    };
    let ident = &ast.ident;
    let fields_named = match fields {
        syn::Fields::Named(fields_named) => fields_named,
        _ => panic!(
            "ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas only works with named fields"
        ),
    };
    match fields_named.named.len() {
        2 => (),
        _ => {
            panic!(
                "ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas fields_named.len() != 2"
            )
        }
    }
    let source_type_ident = match &fields_named.named[0].ty {
        syn::Type::Path(type_path) => type_path,
        _ => panic!("ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas only works on structs fields with  syn::Type::Path type"),
    };
    let first_source_type_ident = source_type_ident.path.segments[0].ident.clone();
    let first_source_type_ident_as_string = format!("{}", first_source_type_ident);
    let error_and_where_was_init = if first_source_type_ident_as_string == *"Vec" {
        quote::quote! {
            match source_place_type {
                tufa_common::config::source_place_type::SourcePlaceType::Source => {
                    let mut error_handle = source
                    .iter()
                    .map(|e| e.get_source())
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !error_handle.is_empty() {
                        error_handle.pop();
                        error_handle.pop();
                    }
                    let mut where_was_vec_as_string = source
                    .iter()
                    .map(|e| e.get_bunyan_where_was(source_place_type, git_info, CONFIG.is_bunyan_separated_by_line_enabled))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !where_was_vec_as_string.is_empty() {
                        where_was_vec_as_string.pop();
                        where_was_vec_as_string.pop();
                    }
                    let where_was_handle = format!("[{} from [{}]]", where_was.file_line_column(), where_was_vec_as_string);
                    let where_was_handle = String::from("kekw");
                    tracing::error!(
                        error = error_handle,
                        where_was = where_was_handle,
                    );
                }
                tufa_common::config::source_place_type::SourcePlaceType::Github => {
                    let mut error_handle = source
                    .iter()
                    .map(|e| e.get_source())
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !error_handle.is_empty() {
                        error_handle.pop();
                        error_handle.pop();
                    }
                    let mut where_was_vec_as_string = source
                    .iter()
                    .map(|e| format!("{}, ", e.get_bunyan_where_was(source_place_type, git_info, CONFIG.is_bunyan_separated_by_line_enabled)))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !where_was_vec_as_string.is_empty() {
                        where_was_vec_as_string.pop();
                        where_was_vec_as_string.pop();
                    }
                    let where_was_handle = format!("[{} from [{}]]", where_was.file_line_column(), where_was_vec_as_string);
                    tracing::error!(
                        error = error_handle,
                        where_was = where_was_handle,
                    );
                }
                tufa_common::config::source_place_type::SourcePlaceType::None => {
                    let mut error_handle = source
                    .iter()
                    .map(|e| format!("{}, ", e.get_source()))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !error_handle.is_empty() {
                        error_handle.pop();
                        error_handle.pop();
                    }
                    tracing::error!(
                        error = error_handle
                    );
                }
            };
        }
    } else if first_source_type_ident_as_string == *"HashMap" {
        quote::quote! {
            match source_place_type {
                tufa_common::config::source_place_type::SourcePlaceType::Source => {
                    let mut error_handle = source
                    .iter()
                    .map(|(key, e)| format!("{} {}, ", key, e.get_source()))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !error_handle.is_empty() {
                        error_handle.pop();
                        error_handle.pop();
                    }
                    let mut where_was_vec_as_string = source
                    .iter()
                    .map(|(key, e)| format!("{} for [{}], ", key, e.get_bunyan_where_was(source_place_type, git_info, CONFIG.is_bunyan_separated_by_line_enabled)))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !where_was_vec_as_string.is_empty() {
                        where_was_vec_as_string.pop();
                        where_was_vec_as_string.pop();
                    }
                    let where_was_handle = format!("[{} from [{}]]", where_was.file_line_column(), where_was_vec_as_string);
                    tracing::error!(
                        error = error_handle,
                        where_was = where_was_handle,
                    );
                }
                tufa_common::config::source_place_type::SourcePlaceType::Github => {
                    let mut error_handle = source
                    .iter()
                    .map(|(key, e)| format!("{} {}, ", key, e.get_source()))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !error_handle.is_empty() {
                        error_handle.pop();
                        error_handle.pop();
                    }
                    let mut where_was_vec_as_string = source
                    .iter()
                    .map(|(key, e)| format!("{} for [{}], ", key, e.get_bunyan_where_was(source_place_type, git_info, CONFIG.is_bunyan_separated_by_line_enabled)))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !where_was_vec_as_string.is_empty() {
                        where_was_vec_as_string.pop();
                        where_was_vec_as_string.pop();
                    }
                    let where_was_handle = format!("[{} from [{}]]", where_was.file_line_column(), where_was_vec_as_string);
                    tracing::error!(
                        error = error_handle,
                        where_was = where_was_handle,
                    );
                }
                tufa_common::config::source_place_type::SourcePlaceType::None => {
                    let mut error_handle = source
                    .iter()
                    .map(|(key, e)| format!("{} {}, ", key, e.get_source()))
                    .fold(String::from(""), |mut acc, elem| {
                        acc.push_str(&elem);
                        acc
                    });
                    if !error_handle.is_empty() {
                        error_handle.pop();
                        error_handle.pop();
                    }
                    tracing::error!(error = error_handle);
                }
            };
        }
    } else {
        quote::quote! {
            match source_place_type {
                tufa_common::config::source_place_type::SourcePlaceType::Source => {
                    let error_handle = source.get_source();
                    let where_was_handle = source.get_bunyan_with_additional_where_was(
                        &where_was,
                        source_place_type,
                        git_info,
                        CONFIG.is_bunyan_separated_by_line_enabled
                    );
                    tracing::error!(
                        error = error_handle,
                        where_was = where_was_handle,
                    );
                }
                tufa_common::config::source_place_type::SourcePlaceType::Github => {
                    let error_handle = source.get_source();
                    let where_was_handle = source.get_bunyan_with_additional_where_was(
                        &where_was,
                        source_place_type,
                        git_info,
                        CONFIG.is_bunyan_separated_by_line_enabled
                    );
                    tracing::error!(
                        error = error_handle,
                        where_was = where_was_handle,
                    );
                }
                tufa_common::config::source_place_type::SourcePlaceType::None => {
                    let error_handle = source.get_source();
                    tracing::error!(error = error_handle);
                }
            };
        }
    };
    let gen = quote::quote! {
        impl tufa_common::traits::with_tracing::WithTracing<#source_type_ident> for #ident {
            fn with_tracing(
                source: #source_type_ident,
                where_was: tufa_common::where_was::WhereWas,
                source_place_type: &tufa_common::config::source_place_type::SourcePlaceType,
                git_info: &tufa_common::helpers::git::git_info::GitInformation,
            ) -> Self {
                #error_and_where_was_init
                Self { source, where_was }
            }
        }
    };
    gen.into()
}
