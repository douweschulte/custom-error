# Custom error format

A library to easily create nice end user facing errors, especially for custom parsing work.

- [x] Colour
- [x] Fixed identifier (or code) (=> path to type)
- [x] position code calling error? (=> only if called with macro)
- [x] Context lines with context
- [x] Add help to the error
- [x] Test when using multiple enums as type source, how does it combine?
- [x] Linking to docs with extra help, automatically create link to docs on the enum type (only works on published crates, only using the macro, figure out a nice way to call)
- [x] Simple unwrap like addition? Option&lt;T&gt;.unwrap_or_error(CustomError&lt;E&gt;)
- [x] Add the option to load multiple context lines at once and specify multiple highlights
- [x] Add notes to highlights, especially useful for multiple ones
- [x] Possibility of adding related code spans, or multiple highlights within a context
- [ ] Implement From on CustomError for common and generic Errors?

# Features
* Builder style error messages with many optional elements
    * Longer messages
    * Help notices
    * Urls
    * Location in the source file (were the error was defined)
* Builder style context for the error messages, like lines in a source file
    * Line numbers
    * Context lines before and after the given line
    * Highlights
* Colour output (behind the optional `ansi_term` feature)
* Unique meaningful identifiers for all errors, by using your own enums
* Combine different error types into hierarchies of errors (using `.convert()`)
* Create links to docs.rs if the used type is an enum and the crate is published

# License
MIT