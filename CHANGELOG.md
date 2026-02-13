## [2.0.0](https://github.com/FAZuH/contribution-grid/compare/v1.0.2...v2.0.0) (2026-02-13)


### ⚠ BREAKING CHANGES

* calculate_date_range, calculate_dimensions, and
get_coordinates are no longer part of the public API
* Theme, Strategy, and strategy implementations are no longer
re-exported at the crate root. They must be accessed via the builtins module:

Before:
  use contribution_grid::{Theme, LinearStrategy};

After:
  use contribution_grid::builtins::{Theme, LinearStrategy};
  // or with the new Strategy factory
  use contribution_grid::builtins::{Theme, Strategy};

- Created src/builtins.rs containing Theme factory, Strategy factory, and strategy impls
- Moved LinearStrategy, LogarithmicStrategy, ThresholdStrategy to builtins
- Added Strategy factory for creating builtint strategy instances

### Code Refactoring

* Make internal methods of ContributionGraph private ([d20ff00](https://github.com/FAZuH/contribution-grid/commit/d20ff002c1240c9b523d4e45c22710bbbdea6ecd))
* Move themes and strategies to builtins module ([e1f9c85](https://github.com/FAZuH/contribution-grid/commit/e1f9c856baa3f3ebf304a466f15bea549bafaae0))


### Continuous Integration

* Add caching to Rust builds ([2ecf649](https://github.com/FAZuH/contribution-grid/commit/2ecf649e300c8afc17217c3730cb7b8f9645ffa3))
* Allow dirty working directory in cargo publish ([d18df4a](https://github.com/FAZuH/contribution-grid/commit/d18df4a830ead872eb1c5b8d7081f54eef71cb9c))
* Checkout correct tag ref in publish-crates job ([a744f4d](https://github.com/FAZuH/contribution-grid/commit/a744f4d4c7480738a86ed0684f942a814394006c))

## [1.0.2](https://github.com/FAZuH/contribution-grid/compare/v1.0.1...v1.0.2) (2026-02-13)


### Miscellaneous Chores

* **release:** v1.0.2 ([499a7df](https://github.com/FAZuH/contribution-grid/commit/499a7df0729b9670050fa150fa1b12dcb9c32583))

## [1.0.1](https://github.com/FAZuH/contribution-grid/compare/v0.1.0...v1.0.1) (2026-02-13)

## [0.1.0](https://github.com/FAZuH/contribution-grid/compare/d1b8275e817148327881fb3a3e02dcdf39847b79...v0.1.0) (2026-02-13)


### Documentation

* Add missing package information to Cargo.toml ([fa33403](https://github.com/FAZuH/contribution-grid/commit/fa334039b1ba11c1ef5f94a1062a9d41b5720e5b))


### Miscellaneous Chores

* **release:** v0.1.0 ([bd253ad](https://github.com/FAZuH/contribution-grid/commit/bd253ada9a81f009e2d1e0d0470abf0451758fd3))


### Continuous Integration

* Add GitHub actions ([d1b8275](https://github.com/FAZuH/contribution-grid/commit/d1b8275e817148327881fb3a3e02dcdf39847b79))
* Modify release workflow ([2271738](https://github.com/FAZuH/contribution-grid/commit/227173859054b557bcc86d2606955215b118e293))

