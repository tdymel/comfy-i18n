/*
    # Concepts
    ## Architecture
      - We need i18n! for flexability. Not sure if i18n!() can handle custom syntax.
        => It has to be flexible enough to support features like: File Loading, Remote Loading, Inline etc.
      - Recurvie mod for every translation
      - Use Impls, statics, consts for comfortable API
      - Make everything a11ble under the I18n target enum
      - Generate by creating first a recurive tree like struct then generate the solution.
        => Decouble generation and interpretation of the AST and makes us more flexible in the translation
        Mod {
          impls,
          consts,
          statics,
          struct_decl,
          nested -> Other Mods (Nested structs, tuple types etc.)
        }
      

    ## Migration from Rust-I18n
      - Provide a guide
      - Provide a t! macro
        => Default context:
           - Use static variable for it
           - comfy_i18n::set_default_context! / comfy_i18n::default_context! (Or just a function)
        => Syntax
           - Reserved keywords: key, context
           - Multiple ways to specify args:
             - name: "Bla" | "name": "Bla"
             - name = "Bla" | "name" = "Bla"
             - name => "Bla" | "name" => "Bla"
        => t!("path.to.field") generates I18n::DE.path().to.field
        => Runtime access: I18n::DE.by_path<T>(path: &str) -> Option<T>
           - Available on I18n and every node
           - How it may work: match fields &str -> Any -> downcast_ref
           - How do we match from root objects?
             - Dynamic Dispatch via static Map => Dynamic + Locking during compilation
               => Dont think there are any static alternatives?
               => Use hashbrown for no_std support => Also faster than std::HashMap

    ## Format! 
      - Performance:
        => Const fold as much as possible
        => Smart Caching: Store ShortCut templates for certain arg combinations (Enums work very well here)
        => Format! only accepts literals as template. So we can not pre compute the template.
           - As of 2023: format! was not very embedded system friendly as it couldnt be properly optimized by the compiler, blowing up the code size
           - But there is work to improve it on the way, so writing our own implementation is likely to just be worse and less flexible

      - Tuples and Arrays: 
        => Boxed FN Pointers:
        => No Access to self!
          => Pre-Process: change self to lang_self_references_to.the_component_self_references_to().path.to.self
        => Sub-Optimal Syntax

      - Structs: 
        => Create pub impl with field_name(&self) using the field for better syntax. (Field should be private)


*/

/// Generates structures like these from the AST.
///
/// # Mod: i18n.rs
/// #[derive(ComfyI18n)]
/// pub enum I18n {
///     #[comfy(fallback: true, locale: "de-DE")]
///     DE,
///     // Defaults to: Fallback: false, locale: en_US
///     EN,
///     #[comfy(locale: "es-ES")]
///     ES
/// }
///
/// impl I18n {
///   pub const fn fallback(key: Self, available: [Self]) -> Self {
///     ...
///   }
/// }
///
/// // will generate:
/// pub mod ci18n_context {
///   pub struct Context {
///     pub fallback: bool,
///     pub locale: Locale // TODO: Require or set as Default en_US?
///   }
///   
///   impl Context {
///     context specific helper function delegates for time etc. using ICU ...
///
///     TODO: Or implement it on I18n Enum directly?
///   }
///
///   // TODO: This makes it std, cause Deref is std::ops
///   impl Deref<Locale> for Context { ... }
///   
///   impl super::I18n {
///     // TODO: Or define fallback here?
///
///     pub const fn context(key: Self) -> Context {
///         ...
///     }
///   }
/// }
///
/// # Any component
///
/// TODO: Find better names for "Key" and "Translations"
/// i18n!(
///     // Compile Error will be generated if this name is used multiple times!
///     name: AnyComponent,
///     key: crate::i18n::I18n,
///     // Validation will require at least one language to be set. Questionable how it will work with remote.
///     translations: {
///         DE: Inline({ format: "comfy", value: { custom fields ... } }), # TODO: Debatable how we want the syntax to be like
///         EN: File("..."),
///         ES: Remote("...")
///     },
///     
///     // Or for all translations
///     translations: Folder("..."),
///     translations: Remote("..."),
/// )
///
/// // will generate:
/// pub mod ci18n_any_component {
///   pub struct AnyComponent {
///     ci18n_key: crate::i18n::I18n,
///     custom fields...
///   }
///
///   pub mod nested {
///     Pattern repeats recursively ...
///   }
///
///   impl AnyComponent {
///     pub fn format_field_name(&self, arg1, ..., argN) -> String {
///         match Self::fallback(self.ci18n_key, [crate::i18n::I18n::DE]) {
///             crate::i18n::I18n::DE => {
///               format!("Optimized template: {arg1:..} ... {argN:..}", arg1 = arg1, ..., argN = argN)
///             },
///             _ => panic!()
///         }
///     }
///
///     ...
///   }
///
///   pub const DE: AnyComponent { ... };
///
///   impl crate::i18n::I18n {
///     pub const fn any_component(key: Self) -> AnyComponent {
///         match Self::fallback(key, [crate::i18n::I18n::DE]) {
///             crate::i18n::I18n::DE => DE,
///             _ => panic!()
///         }
///     }
///   }
/// }
///
/// # In your component
///
/// fn your_component(i18n: crate::i18n::I18n) {
///     println("{}", i18n.any_component().nested_field.tuple_field.2.format_field("Hello", "World"));
/// }
///
pub struct RustGenerator;

impl RustGenerator {
    pub fn gen_decl() {
        todo!()
    }

    pub fn gen_init() {
        todo!()
    }

    pub fn gen_impl() {
        todo!()
    }
}
