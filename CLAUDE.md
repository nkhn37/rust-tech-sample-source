# Rust blog project instructions

This repository contains educational Rust examples for blog posts.

## Principles

- Prioritize clarity and readability over optimization
- Keep code simple and easy to understand for beginners
- Prefer idiomatic Rust
- Prefer examples that focus on a single concept

## Style

- Follow Rust naming conventions (snake_case, CamelCase, etc.)
- Keep functions and examples small and focused
- Avoid unnecessary complexity
- Use modern Rust idioms (Rust 1.58.0+):
  - Use named arguments in `println!` and other formatting macros
  - Display trait: `println!("{variable}")` instead of `println!("{}", variable)`
  - Debug trait: `println!("{variable:?}")` instead of `println!("{:?}", variable)`
  - This improves readability, especially with multiple variables or debug output

## Flexibility

- Using `unwrap()` is acceptable when it helps clarity
- Do not over-engineer error handling for simple examples

## When generating code

- Match the style and level of complexity of surrounding examples
- Keep explanations in mind (code should be easy to explain)
