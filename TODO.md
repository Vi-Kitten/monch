# Future Refactors and Abstractions

- [ ] Refactor to or Create additional trait that can handle dynamic parsers `impl`
- [ ] Refactor to allow for handling of incrimental parsing
- [ ] Refactor to use explicit types instead of `impl Parser` from `Fn` types, this will allow certain traits to be preserved through combinators. (this will probably have to be done anyways at some point)
- [ ] Create a specialised trait for Errors we support to help writing parsers with nice error messages
- [ ] Create primitives.rs for leaf parsers, maybe move Wrap and Fail into there
- [ ] Create errors.rs for handling errors and providing a specific error type
