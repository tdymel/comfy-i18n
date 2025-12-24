# ☕ Comfy I18n
```rust
// TODO Add example here!
// TODO: Example: En vs. Passive aggrassive En => "Reason" vs. "Whats your lame excuse, this time? 🤨"
```

## Concept
> TODO: Add diagram to explain it

Comfy I18n aims to be a source format and target language agnostic transpiler. Currently, only **rust** is implemented.

It translates your i18n sources to usable structures in your source code. These structures are **component based** by design and are as such **nested** and **composable**.

## Core Features
✅ **Ergonomical and intuitive API**  
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

## But is it blazingly fast?!
TODO: Redo Benchmarks!

YES! - Probably? - Maybe?  

Comfy I18n trades speed for binary size and compile time.  
It creates specialized structures and functions, which may or may not be optimized away by the compiler.  
Nevertheless, the binary size and compile time would be bigger compared to a dictionary based approach.

| Crate      | Parsing          | Literal  | Interpolation (2 args) | Interpolation (7 args) |
| ---------- |----------------- | -------- | ---------------------- | ---------------------- |
| comfy-i18n | 1.3 ms ¹         | 0.26 ns  | < 38 - 60 ns ²         | < 173 ns ²             |
| rust-i18n  | Unknown          | 32.63 ns | 128.70 ns              | 370.28 ns              |

> ¹ For an average size structure  
> ² We use [dfmt](https://github.com/tdymel/dfmt). If we can use a literal template, it uses `format!` under the hood. In addition to this, we do const folding during transpilation and once statically during runtime, to reduce the amount of arguments.

## License
This project is dual licensed under the Apache 2.0 license and the MIT license.