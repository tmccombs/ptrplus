# 2.1.0

- GAT features no longer require "nightly" feature
- Increase minimum supported rust version to 1.65

# 2.0.0

- Increase rust edition to 2021
- Add "nightly" feature to enable features that require GATs
    - Allows you to use a type "family" for FromRaw
- Relax Sized constraint on several traits

# 1.1.0

- Added support for Rc and Arc
- Added no_std support (including supporting no_std with alloc crate)
- Added AsPtr implementations for Box and CStr
- Added Unit tests
- Use 2018 edition
- Add safety documentation
