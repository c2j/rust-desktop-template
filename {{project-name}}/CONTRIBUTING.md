# Contributing to {{project-name}}

Thank you for your interest in contributing to {{project-name}}! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.75+ (latest stable recommended)
- Git
- Basic familiarity with Rust and GUI development

### Setting Up Your Development Environment

1. **Fork and Clone**
   ```bash
   # Fork the repository on GitHub, then clone your fork
   git clone https://github.com/your-username/{{project-name}}.git
   cd {{project-name}}
   ```

2. **Set Up Development Tools**
   ```bash
   # Install Rust toolchain components
   rustup component add rustfmt clippy

   # Install useful tools
   cargo install cargo-watch cargo-expand
   ```

3. **Run the Application**
   ```bash
   cargo run
   ```

## Development Workflow

### Making Changes

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**
   - Write clean, well-commented code
   - Follow Rust conventions and best practices
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Your Changes**
   ```bash
   # Run all tests
   cargo test

   # Check for compilation errors
   cargo check

   # Run Clippy for additional checks
   cargo clippy -- -D warnings

   # Format your code
   cargo fmt
   ```

### Code Style Guidelines

#### Rust Conventions

- Use `cargo fmt` for code formatting
- Follow the official Rust style guide
- Keep functions focused and small
- Use meaningful variable and function names
- Add doc comments for public APIs

#### Example Module

```rust
//! Brief description of the module

use crate::prelude::*;

/// Brief description of the struct
///
/// # Examples
/// ```
/// use {{project-name}}::MyStruct;
/// let instance = MyStruct::new();
/// ```
pub struct MyStruct {
    /// Description of the field
    field: Type,
}

impl MyStruct {
    /// Creates a new instance of MyStruct
    pub fn new() -> Self {
        Self {
            field: default_value(),
        }
    }
}

impl Default for MyStruct {
    fn default() -> Self {
        Self::new()
    }
}
```

#### UI Component Guidelines

- Separate logic from UI rendering where possible
- Use descriptive variable names for UI elements
- Keep UI rendering functions focused
- Handle user interactions cleanly

```rust
impl MyComponent {
    /// Renders the component UI
    pub fn render(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.horizontal(|ui| {
            ui.label("Component Label:");
            if ui.button("Action").clicked() {
                self.handle_action();
            }
        });
    }

    /// Handles user interactions
    fn handle_action(&mut self) {
        // Handle the action
    }
}
```

### Testing

#### Unit Tests

- Write tests for all public functions
- Use descriptive test names
- Test edge cases and error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function_success_case() {
        let input = "test";
        let expected = "expected_output";
        let result = my_function(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_my_function_error_case() {
        let input = "";
        let result = my_function(input);
        assert!(result.is_err());
    }
}
```

#### UI Testing

- Test UI component creation
- Test user interactions
- Test theme applications

```rust
#[test]
fn test_component_creation() {
    let component = MyComponent::new();
    assert!(component.is_valid());
}
```

## Documentation

### Updating Documentation

- Update README.md for user-facing changes
- Update code comments for API changes
- Add examples for new features
- Keep changelog updated

### Documentation Comments

Use proper documentation comments for public APIs:

```rust
/// Description of the function
///
/// # Arguments
///
/// * `arg1` - Description of first argument
/// * `arg2` - Description of second argument
///
/// # Returns
///
/// Description of the return value
///
/// # Examples
///
/// ```
/// let result = my_function(arg1, arg2);
/// assert_eq!(result, expected_value);
/// ```
pub fn my_function(arg1: Type1, arg2: Type2) -> ReturnType {
    // Implementation
}
```

## Submitting Changes

### Pull Request Process

1. **Update Your Branch**
   ```bash
   git add .
   git commit -m "feat: add new feature description"
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request**
   - Use descriptive title and description
   - Reference any related issues
   - Include screenshots for UI changes
   - Add testing instructions

3. **Code Review**
   - Respond to review comments promptly
   - Make requested changes
   - Keep the PR updated

### Commit Message Guidelines

Use conventional commit messages:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code formatting changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
- `feat(navigation): add keyboard shortcuts`
- `fix(theme): resolve color palette issue`
- `docs(readme): update installation instructions`

## Reporting Issues

### Bug Reports

When reporting bugs, please include:
- Rust and OS version
- Steps to reproduce
- Expected vs actual behavior
- Any error messages or stack traces
- Screenshots if applicable

### Feature Requests

When requesting features, please include:
- Clear description of the feature
- Use case and motivation
- Possible implementation ideas
- Examples from other applications

## Code Review Guidelines

### For Reviewers

- Check for correctness and performance
- Ensure code follows project conventions
- Verify tests are comprehensive
- Check documentation accuracy
- Be constructive and helpful in feedback

### For Contributors

- Respond to all review comments
- Explain complex code if needed
- Be open to suggestions
- Update documentation for changes

## Performance Considerations

- Profile the application when adding new features
- Be mindful of memory usage in UI components
- Optimize rendering loops
- Consider async operations for long-running tasks

## Security

- Follow secure coding practices
- Validate all user inputs
- Use safe Rust features
- Keep dependencies updated
- Report security vulnerabilities responsibly

## Getting Help

- Check the documentation first
- Search existing issues
- Ask questions in discussions
- Join our community channels

Thank you for contributing to {{project-name}}! 🚀