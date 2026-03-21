# ☕ Comfy I18n
Comfy I18n aims to be a source format and target language agnostic transpiler. Currently, only the **comfy**-format and **rust** is implemented.

It translates your i18n sources to usable structures in your source code at compile time.  
These structures are **component based** by design and are as such **nestable** and **composable**.

The API design is very opinionated and aims to use the language specific features to provide the most ergonomic API possible, while being feature rich and blazingly fast. Compared to the common dictionary based approach, it comes with an overhead in compile time and binary size.

## Core features

### ✅ Ergonomic, intuitive and extendable API
Comfy I18n aims to have the most ergnomical and intuitive API design possible.
This is a small showcase of the API. Please refer to the examples in the documentation for a more detailed overview.

```rust
// TODO: Improve Readme. Make it easy and concise. Add playgrounds where examples are.

// Must be used in the crate root
// Define the available contexts
i18n_init!(
    EN,
    #[fallback]
    DE,
    RU
);

i18n!(
    animals,
    EN: {
        cat: {
            one: "Cat",
            many: "Cats"
        },
    },
    DE: "path/relative/to/root.comfy"
)

i18n!(
    some_component,
    EN: {
        // Fully supports core::fmt specifiers
        fmt_like_syntax: "Today is the {num_days:03}th day of the year!",
        // Reference the localization trees
        i18n_references: "Hello {i18n.EN.some_component.person.name:^20}!",
        context_references: "Hello {context.some_component.person.name:^20}!",
        root_references: "Hello {root.person.name:^20}!",
        person: {
            self_references: "Hello {self.name:^20}!",
            name: "Peter",
            age: 42,
            pronouns: ["he", "him"],
            // You can also use any constant, as long as you hint the type
            favorite_animal: I18n::DE.animals().cat().one() as &'static str,
        },
        arbitary_nesting: (1, (2, [{a: 42}; 5])),
        // 🚧 More utility functions and ICU support will be added in the future
        complicated_localization: |&self, amount: usize| -> String {
            let cat = self.context().animals().cat();
            match amount {
                0 => format!("No {}", cat.many()),
                1 => format!("A {}", cat.one()),
                num_cats => format!("{} {}", num_cats, cat.many())
            }
        }
    },
    DE: {
        person: {
            name: "Anna",
            age: 21,
        },
        complicated_localization: |&self, amount: usize| -> String {
            let cat = self.context().animals().cat();
            match amount {
                0 => format!("Keine {}", cat.many()),
                1 => format!("Eine {}", cat.one()),
                num_cats => format!("{} {}", num_cats, cat.many())
            }
        }
    }
)

fn some_component() {
    // Set the current global context within the application 
    // By default the next best context is chosen (fallback > first)
    I18n::DE.as_current_context();

    assert_eq!(
        I18n::EN.some_component().fmt_like_syntax(&83)
        "Hello, Peter (42)! Today is the 083th day of the year!"
    );

    // Falls back to the english format string, using the german values for person
    assert_eq!(
        I18n::current().some_component().fmt_like_syntax(&83),
        "Hello, Anna (21)! Today is the 083th day of the year!"
    );

    // Falls back to english, as its not specified in the fallback language
    assert_eq!(
        I18n::RU.some_component().person().pronouns(),
        ["he", "him"]
    );

    // Then call your custom function like this
    assert_eq!(
        I18n::DE.some_component().person().complicated_localization(42),
        "42 Katzen".to_string()
    );

    // Access the localization by path or using the usual t! macro
    // The macro defaults to the first fallback context by default, unless another is specified
    // For dynamic paths, the macro assumes the type &'static str if no other type is specified
    // The macro can resolve the type of static paths itself
    let path = "some_component.person.age";
    assert_eq!(
        I18n::EN::by_path::<i32>(path),
        t!(path, ty = i32)
    );
    assert_eq!(
        t!(path, ty = i32),
        t!("some_component.person.age")
    );
    assert_eq!(
        I18n::EN.some_component().fmt_like_syntax(&83),
        t!("some_component.fmt_like_syntax", context = I18n::EN, num_days = 83)
    );
}
```

### ✅ Blazingly fast
Comfy I18n trades speed for binary size and compile time.  
It creates specialized structures and functions, which may or may not be optimized away by the compiler.  
Nevertheless, the binary size and compile time would be bigger compared to a dictionary based approach.

For formatted strings [dfmt](https://github.com/tdymel/dfmt) is used under the hood, which performance is very close the the original `core::fmt` machinery, as it uses this machinery under the hood. This means for very simple cases it is slightly slower, as it does not take shortcuts (yet). However, this library will benefit from any future performance improvements of the `core::fmt` machinery.

| Benchmark                                                  | Comfy-i18n            | Rust-i18n        |
| ---------------------------------------------------------- | --------------------- | ---------------- | 
| Lookup of constant                                         | **0.4 ns**            | 18.96 ns         |
| Interpolation 2 args (strings only)                        | 50.18 ns              | **47.15 ns**     |
| Interpolation 2 args (f64 only)                            | **129.49 ns**         | 149.10 ns        |
| Interpolation 2 args (f64 only) + dynamic args             | **128.72 ns**         | 148.25 ns        |
| Interpolation 2 args (f64 only) + dynamic args + specifier | **130.03 ns**         | 336.88 ns        |
| Interpolation 7 args (strings only)                        | 136.20 ns             | **85.55 ns**     |
| Interpolation 7 args (f64 only)                            | **413.65 ns**         | 457.87 ns        |
| Interpolation 7 args (f64 only) + dynamic args             | **412.05 ns**         | 449.03 ns        |
| Interpolation 7 args (f64 only) + dynamic args + specifier | **412.40 ns**         | 1,084.64 ns      |

### 🚧 [#16](https://github.com/tdymel/comfy-i18n/issues/16): Compile time validation
Be warned if there are missing translations. Warnings during development and errors during production builds.

### 🚧 [#5](https://github.com/tdymel/comfy-i18n/issues/5): Different source file formats
Load localizations from different kind of source file formats other than the comfy format, e.g. TOML, JSON, Fluent or Gettext

### 🚧 [#18](https://github.com/tdymel/comfy-i18n/issues/18): Hot reload translations from source files
Source files are watched for changes during development and hot reloaded if changes occur.

### 🚧 [#2](https://github.com/tdymel/comfy-i18n/issues/2): Remote source files support
Load source files from a remote source on the fly. During compilation these source files are loaded to create the specification for the structure.

### 🚧 [#8](https://github.com/tdymel/comfy-i18n/issues/8): No-std support
This library already largely supports no-std environments. However, this will not be a hard requirement for the releases until all core features have been implemented.

## License
This project is licensed under either

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.