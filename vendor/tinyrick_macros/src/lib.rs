//! Task declaration attributes

extern crate ctor;
extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate tinyrick_models;

/// task registers tasks with signature like unboxed tinyrick_models::Task.
#[proc_macro_attribute]
pub fn task(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.sig.ident;
    let name_uppercase = name.to_string().to_uppercase();
    let setup_name = quote::format_ident!("__SETUP_FN_{}", name_uppercase);
    let name_str = &name.to_string();

    proc_macro::TokenStream::from(quote::quote! {
        #[::ctor::ctor]
        fn #setup_name () {
            let mut tasks = ::tinyrick_models::TASKS.lock().unwrap();
            tasks.insert(#name_str, Box::new(#name));
        }

        #input
    })
}

/// task registers tasks with signature like unboxed tinyrick_models::Task.
#[proc_macro_attribute]
pub fn default_task(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.sig.ident;
    let name_uppercase = name.to_string().to_uppercase();
    let setup_name = quote::format_ident!("__SETUP_FN_{}", name_uppercase);
    let name_str = &name.to_string();

    proc_macro::TokenStream::from(quote::quote! {
        #[::ctor::ctor]
        fn #setup_name () {
            let mut tasks = ::tinyrick_models::TASKS.lock().unwrap();
            tasks.insert(#name_str, Box::new(#name));
            let mut default_task = ::tinyrick_models::DEFAULT_TASK.lock().unwrap();
            *default_task = Some(#name_str);
        }

        #input
    })
}
