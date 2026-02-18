# ☕ Comfy I18n
```rust
#[derive(ComfyI18n)]
pub enum Language {
    #[fallback]
    PASSIVE_AGGRESSIVE,
    NORMAL,
}

i18n!(
    name: SomeComponent,
    key: crate::Language,
    translations: {
        PASSIVE_AGGRESSIVE: {
            reason: "Whats your lame excuse, this time? 🤨"
        },
        NORMAL: {
            reason: "Reason"
        }
    }
)

fn some_component(lang: Language) {
    println!("{}", lang.some_component().reason());
}
```

## Concept
![Architecture](./architecture.png)

Comfy I18n aims to be a source format and target language agnostic transpiler. Currently, only the **comfy**-format and **rust** is implemented.

It translates your i18n sources to usable structures in your source code at compile time.  
These structures are **component based** by design and are as such **nested** and **composable**.

The API design is very opinionated and aims to use the language specific features to provide the most ergonomic API possible, while being feature rich and blazingly fast. Compared to the common dictionary based approach, it comes with an overhead in compile time and binary size.

## Core Features
TODO: For each make a headline and add examples in a spoiler tag (if it gets too long)

✅ **Ergonomic and intuitive API**  
✅ **Blazingly fast**  
✅ **Type safe and const:** Benefit from real structures with literals. No run-time shenanigans! **Enjoy the full benefits of static code analysis.**  
✅ **Compile time validation:** Be warned if there are missing translations.  
✅ **std::fmt like syntax and const folding:** `"{self.hello} {world}"` -> `(world) -> "Hello {world}"`  
✅ **Arbitrary nested composites:** Mix and match literals and composites.  
✅ **Arbitrary functions:** Simply provide an `impl` for any localization.  
✅ **Component based:** Keep your localizations close to your components.  
✅ **Composable:** Reuse localizations in a type safe manner.  
✅ **Extendable:** Build your own I18n-Library on top of comfy-i18n.  
✅ **Fallback:** If a translation is missing it tries to use the next best one.

## Contribute! - Planned features
Please feel free to implement one of these features. I don't have a lot of time, and it would help the community tremendously.

* Compile time validation (TODO: Ticket)
* Comfy: Function decleration support (TODO: Ticket)
* Using source files (using the comfy-format) (TODO: Ticket)
* Remote source (TODO: Ticket)
* TOML support (TODO: Ticket)
* JSON support (TODO: Ticket)
* Fluent support (TODO: Ticket)
* Gettext files support (TODO: Ticket)
* ICU/CLDR integration (TODO: Ticket)
* Const folding for templates (TODO: Ticket)
* Smart template caching and optimization (TODO: Ticket)
* Code generation for Javascript (TODO: Ticket)
* No-std support (TODO: Ticket)
* Hot language file reloading (TODO: Ticket)

## But is it blazingly fast?!
TODO: Redo Benchmarks!

YES! - Probably? - Maybe?  

Comfy I18n trades speed for binary size and compile time.  
It creates specialized structures and functions, which may or may not be optimized away by the compiler.  
Nevertheless, the binary size and compile time would be bigger compared to a dictionary based approach.

| Crate      | Literal  | Interpolation (2 args) | Interpolation (7 args) |
| ---------- | -------- | ---------------------- | ---------------------- |
| comfy-i18n | 0.26 ns  | < 38 - 60 ns ¹         | < 173 ns ¹             |
| rust-i18n  | 32.63 ns | 128.70 ns              | 370.28 ns              |

> ¹ We use [dfmt](https://github.com/tdymel/dfmt). If we can use a literal template, it uses `format!` under the hood. In addition to this, we do const folding during transpilation and once statically during runtime, to reduce the amount of arguments.

## License
This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.