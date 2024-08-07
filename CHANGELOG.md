# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2024-07-10
### 🚀 Features

- [_a457655_](https://github.com/Froloket64/lifers/commit/a4576555d60632430586ca8b501ff318946b8dc0) _(api)_ [**breaking**] Use owned value for data _()_

````text
While previously `StepFn` took the cell data by reference, now it takes
  it by value.
````

- [_d840974_](https://github.com/Froloket64/lifers/commit/d8409747f30d8f23e1a1f022628bbf69ad9b36fc) _(engine)_ [**breaking**] Share methods between builders; Remove `to_mapped` _()_

- [_7e4c79c_](https://github.com/Froloket64/lifers/commit/7e4c79cab85632008a6d1cc631dea7cd2df875de) _(engine)_ Add `is_finished()` and `is_infinite()` _()_

- [_1eaa62f_](https://github.com/Froloket64/lifers/commit/1eaa62f605687e77b55614845f012ece1039535f) _(engine)_ [**breaking**] Add `life_like` automata; Restructure `engine` _()_

### 🚜 Refactoring

- [_0f8ae28_](https://github.com/Froloket64/lifers/commit/0f8ae28d9364eae5a03acc5a4833f44487addc0d) _(lint)_ Apply clippy suggestions _()_

<!-- generated by git-cliff -->
