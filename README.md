# yarner-block-links

A [Yarner](https://github.com/mlange-42/yarner) pre-processor that adds to each code block a list of links to all referenced blocks.

Example:

<table><tr><td>

A list of links is placed under each code block that references other blocks:

```rust
fn main() {
    // ==> Block A.
    // ==> Block B.
}
```

> Macros: [`Block A`](#block-block-a), [`Block B`](#block-block-b)

Other blocks:

<a id="block-block-a"></a>
```rust
//- Block A
print!("Hello");
```

<a id="block-block-b"></a>
```rust
//- Block B
println!(" World!");
```
</td></tr></table>

## Installation

**Binaries**

Pre-compiled binaries will be available as soon as this project is in a usable state.

**Using `cargo`**

```
> cargo install --git https://github.com/mlange-42/yarner-block-links.git --branch main
```

## Usage

Add a section `preprocessor.block-links` to your `Yarner.toml`:

```toml
[preprocessor.block-links]
```

The pre-processor provides optional configuration for link formatting. Defaults are as follows (but all options can be left out):

```toml
[preprocessor.block-links]
template = "{{#if usage}}> Usage: {{usage}}  \n{{/if}}{{#if macros}}> Macros: {{macros}}{{/if}}"
join = " "
label = "`{{label}}`"
```
