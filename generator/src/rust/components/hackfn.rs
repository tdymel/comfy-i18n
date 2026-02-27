use quote::quote;

use crate::shared::{NamePascalCase, NameSnakeCase};

pub fn hackfn(
    strct_name: &NamePascalCase,
    method_name: &NameSnakeCase,
    arg_types: &Vec<proc_macro2::TokenStream>,
    arg_names: &Vec<proc_macro2::TokenStream>,
    ret_type: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        impl core::ops::Deref for #strct_name {
            type Target = dyn Fn(#(#arg_types),*) -> #ret_type;

            fn deref(&self) -> &Self::Target {
                let __this = ::std::mem::MaybeUninit::<Self>::uninit();
                let __closure = move |#(#arg_names: #arg_types),*| -> #ret_type {
                    Self::#method_name(
                        unsafe { &*__this.as_ptr() }
                        #(, #arg_names)*
                    )
                };
                let __layout_of_closure = ::std::alloc::Layout::for_value(&__closure);
                fn __second<'__a, __T>(__first: &__T, __second: &'__a __T) -> &'__a __T {
                    __second
                }
                let __ret = __second(&__closure, unsafe { &*(self as *const Self as *const _) });
                ::std::mem::forget(__closure);
                assert_eq!(__layout_of_closure, ::std::alloc::Layout::new::<Self>());
                unsafe { ::std::mem::transmute(__ret as &Self::Target) }
            }
        }
    }
}
