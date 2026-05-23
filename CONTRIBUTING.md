# Contributing to oximo

Thank you for your interest in contributing to oximo! All contributions are welcome and appreciated.

There are many ways to get involved:

## Reporting Bugs

If you encounter a bug, unexpected behavior, or inconsistency, please open a GitHub issue. Many issues are discovered by users, and reports help improve the project for everyone.

When reporting a bug, try to include:

- A clear description of the problem
- Steps to reproduce it
- Expected and actual behavior
- Relevant error messages or logs
- Minimal reproducible examples when possible

## Improving Documentation

Good documentation is essential for usability.

If you find parts of the documentation confusing, incomplete, outdated, or difficult to follow, please let us know. If you got stuck while trying to do something in oximo, that usually means the documentation can be improved.

Suggestions may include:

- Clarifying existing guides
- Adding examples
- Creating tutorials
- Improving API explanations
- Fixing typos or formatting issues

You can open an issue or submit a pull request directly.

## Contributing Code

Code contributions are welcome, whether they are bug fixes, performance improvements, new features, tests, or refactoring.

To contribute code:

1. Fork the repository
2. Create a new branch for your changes
3. Make your modifications
4. Add or update tests when appropriate
5. Format your code using `cargo fmt` and ensure it passes `cargo clippy` checks before submitting.
6. Write a clear and descriptive commit message following the project's naming convention (see [Commit Naming Convention](#commit-naming-convention)).
7. Run tests to ensure your changes do not break existing functionality and that new features work as intended (see [Testing](#testing)).
8. Submit a pull request

Please keep pull requests focused and reasonably scoped. Smaller pull requests are generally easier to review and merge.

## Testing

Some tests require third-party tools, including commercial solvers.

If you are submitting a pull request, you are not expected to run tests for crates unrelated to your changes. Unrelated crates can be excluded using the `--exclude` argument with `cargo test`.

Default features do not depend on third-party tools.

Before merging, a reviewer will need to run the complete test suite manually. The current GitHub CI workflow does not execute all tests due to solver licensing restrictions.

## Commit Naming Convention

We recommend the following commit naming convention:

- Commits affecting a single crate:
  - `oximo-<crate-name>: <short description>`

- Commits affecting the workspace configuration:
  - `workspace: <short description>`

- Commits affecting GitHub workflows or repository configuration:
  - `github: <short description>`

Examples:

```text
oximo-core: improve error handling
workspace: update dependency versions
github: add CI workflow for testing
```

This convention helps maintain clarity in the commit history and makes it easier to identify the scope of changes at a glance.

## AI-Assisted Contributions

AI-assisted contributions are allowed.
However, contributors remain fully responsible for the quality, correctness, licensing, and usefulness of their submissions. All contributions, whether written entirely by humans or assisted by AI tools, must meet the project's standards.

### Accountability

Contributors are responsible for reviewing and validating any AI-assisted content before submission.

Using an AI tool does not transfer responsibility for:

- Correctness
- Code quality
- License compliance
- Security
- Documentation accuracy

### Transparency

If a significant portion of a contribution is generated or copied verbatim from an AI tool, contributors should disclose it.

Routine assistance such as grammar correction, spelling fixes, or minor phrasing improvements does not require disclosure.

If you need to indicate AI assistance, you can do so by adding it in your pull request description, or you can use a commit trailer such as:

```text
Assisted-by: generic LLM chatbot
```

This information helps the project evaluate tooling practices and improve contribution guidelines over time.

## Licensing

By contributing to oximo, you agree that your contributions will be licensed under the same dual-license terms as the project itself:

- MIT License
- Apache License 2.0

Unless explicitly stated otherwise, all submitted contributions are assumed to be provided under both licenses.
