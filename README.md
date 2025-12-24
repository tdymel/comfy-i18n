# ☕ Comfy I18n

> 🚀 Let's get ~~shit~~ i18n done!

## Goals
✅ Ergonomical and intuitive API  
✅ Easy to use  
✅ Easy to extend  
✅ Blazingly fast

## Concept
Comfy I18n aims to be a [source format and target language agnostic transpiler](CONTRIBUTING.md). Currently, only **rust** is implemented.

It translates your i18n sources to usable structures in your source code. These structures are **component based** by design and are as such **nested** and **composable**.

## Core Features
✅ **Type safe and const:** Benefit from real structures with literals. No run-time shenanigans! **Enjoy the full benefits of static code analysis.**  
✅ **Compile time validation:** Be warned if there are missing translations.  
✅ **std::fmt like syntax and const folding:** `"{self.hello} {world}"` -> `(world) -> "Hello {world}"`  
✅ **Arbitrary nested composites:** Mix and match literals and composites.  
✅ **Arbitrary functions:** Simply provide an `impl` for any localization.  
✅ **Component based:** Keep your localizations close to your components.  
✅ **Composable:** Reuse localizations in a type safe manner.  
✅ **Extendable:** Build your own I18n-Library on top of comfy-i18n.  
✅ **Fallback:** If a translation is missing it tries to use the next best one.  

## Planned Features
🚧 Other source formats: This 20y old format, JSON, YAML, TOML etc.  
🚧 Remote translations  
🚧 Integrate ICU for helper functions: time, pluralizations etc.  
🚧 t! Macro and runtime access by path support  
🚧 Map support  
🚧 Validation  
🚧 Const folding for self.a and self.a() and self.b(123, self.a, ...)  
🚧 Generation  
🚧 CI/CD  
🚧 Deployment process  
🚧 Badges in Readme  
🚧 Documentation  
🚧 no_std feature  
🚧 Smart Cache feature  

## Example
```toml
[dependencies]
comfy-i18n = "0.1"
```

```rust,ignore
TODO
```
> TODO: Example: En vs. Passive aggrassive En => "Reason" vs. "Whats your lame excuse, this time? 🤨"

> TODO: Examples project. Refer to it. Plus put it into the documentation with playgrounds.

> TODO EXAMPLE and Playgrounds!

## But is it blazingly fast?!
TODO: Redo Benchmarks!

YES! - Probably? - Maybe?  

Comfy I18n trades speed for binary size and compile time.  
It creates specialized structures and functions, which may or may not be optimized away by the compiler.  
Nevertheless, the binary size and compile time would be bigger compared to a dictionary based approach.

| Crate      | Parsing          | Literal  | Interpolation (2 args) | Interpolation (7 args) |
| ---------- |----------------- | -------- | ---------------------- | ---------------------- |
| comfy-i18n | 1.3 ms ¹         | 0.26 ns  | 38.32 ns               | 189.08 ns              |
| rust-i18n  | Unknown          | 32.63 ns | 128.70 ns              | 370.28 ns              |

> ¹ For an average size structure

## License
MIT - By contributing to this project you accept that any contributions are licensed under MIT as well.