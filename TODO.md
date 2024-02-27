# Future Refactors, Abstractions and Overall Objectives

- [ ] Test ParserInfo for parser combinators

- [ ] Refactor to or Create additional trait that can handle dynamic parsers `impl`
- [ ] Refactor to allow for handling of incrimental parsing
- [ ] Refactor to use explicit types instead of `impl Parser` from `Fn` types, this will allow certain traits to be preserved through combinators. (this will probably have to be done anyways at some point)
- [ ] Create a specialised trait for Errors we support to help writing parsers with nice error messages
- [x] Create primitives.rs for leaf parsers, maybe move Wrap and Fail into there
    - [ ] Populate primitives.rs with elementary parsers
- [x] Create errors.rs for handling errors and providing a specific error type
    - [ ] Populate errors.rs with error utilities
- [ ] Test the until parsers EXTENSIVELY
- [ ] Make eligible combinators presere MemoHandler status
