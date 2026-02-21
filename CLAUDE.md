# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`refined_type` is a Rust library for creating refined types — types that enforce validation rules through the type system with runtime validation during construction. Values wrapped in `Refined<RULE>` are guaranteed to satisfy their rule's constraints.

## Commands

```bash
cargo build --release          # Build
cargo test --release           # Run all tests
cargo test --lib               # Unit tests only
cargo test --test read_me      # Integration tests from README examples
cargo fmt -- --check           # Check formatting
cargo clippy -- -D warnings    # Lint (CI treats warnings as errors)
```

## Architecture

### Core Types (`src/`)

- **`Refined<RULE>`** (`src/refined.rs`): The central wrapper type. Parameterized by a `Rule` implementor. Construction via `new()` validates the value; `unsafe_new()` panics on failure. Implements `Serialize`/`Deserialize` transparently (serializes as the inner value, validates on deserialization). Also implements `TryFrom` for common types, `Display`, `Clone`, `Eq`, `Ord`.

- **`Rule` trait** (`src/rule.rs`): `trait Rule { type Item; fn validate(target: Self::Item) -> Result<Self::Item>; }` — the single trait all validation rules implement. Also defines `Valid<T>` (always passes) and `Invalid<T>` (always fails) stub rules.

- **`Error<T>`** (`src/result.rs`): Validation error that preserves the original value for recovery.

### Rule Modules (`src/rule/`)

Rules are organized into categories, all re-exported from `src/rule.rs`:

- **`composer/`**: Logical combinators — `And`, `Or`, `Not`, `If`, `IfElse`, `Nand`, `Nor`, `Xor`, `Equiv`, `Imply`. The `And![]` and `Or![]` macros flatten nested compositions.
- **`number/`**: `Equal<N>`, `Greater<N>`, `Less<N>`, `GreaterEqual<N>`, `LessEqual<N>`, `MinMax<MIN, MAX>`, `Range<MIN, MAX>`, `Even`, `Odd`. Uses macros to generate type aliases for all numeric types (e.g., `MinMaxU8`, `EqualI32`).
- **`string/`**: `Alphabet`, `Digit`, `AlphaDigit`, `Email`, `IPv4`, `IPv6`. Custom regex rules via `declare_regex_rule!` macro.
- **`non_empty/`**: `NonEmptyString`, `NonEmptyVec<T>`, `NonEmptyVecDeque<T>`, `NonEmptyHashSet<T>`, `NonEmptyHashMap<K, V>`. Implemented as `Not<EmptyRule<T>>`.
- **`empty/`**: Empty checks extensible via `EmptyDefinition` trait.
- **`length/`**: `LengthEqual<N>`, `LengthGreater<N>`, `LengthLess<N>`, `LengthMinMax<MIN, MAX>`. Extensible via `LengthDefinition` trait.
- **`collection/`**: Rules applied to iterables — `ForAll<RULE>`, `Exists<RULE>`, `Head<RULE>`, `Last<RULE>`, `Tail<RULE>`, `Init<RULE>`, `Index<N, RULE>`, `Reverse<RULE>`, `Skip<RULE, SkipOption>`, and count-based rules (`CountEqual`, `CountGreater`, etc.).

### Naming Convention

- Rule structs use `*Rule` suffix: `NonEmptyStringRule`, `MinMaxRuleU8<MIN, MAX>`
- Type aliases drop the suffix: `NonEmptyString = Refined<NonEmptyStringRule>`, `MinMaxU8<MIN, MAX> = Refined<MinMaxRuleU8<MIN, MAX>>`

### Key Patterns

- Heavy use of const generics for numeric bounds (e.g., `MinMax<const MIN: u8, const MAX: u8>`)
- `PhantomData` used throughout for zero-cost rule abstractions
- Macros (`paste!`, internal macros) generate repetitive impls across numeric types
- `EmptyDefinition` and `LengthDefinition` traits serve as extension points for custom types

## Testing

Unit tests are co-located in modules via `#[cfg(test)]`. Integration tests in `tests/read_me.rs` mirror README examples. The project has ~170 tests.

## Rust Edition

Edition 2024. Stable toolchain.
