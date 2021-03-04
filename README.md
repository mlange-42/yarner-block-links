# yarner-block-links

[![Test status](https://github.com/mlange-42/yarner-block-links/actions/workflows/tests.yml/badge.svg)](https://github.com/mlange-42/yarner-block-links/actions/workflows/tests.yml)
[![Crate](https://img.shields.io/crates/v/yarner-block-links.svg)](https://crates.io/crates/yarner-block-links)

A [Yarner](https://github.com/mlange-42/yarner) plugin that adds to each code block a list of links to all referenced and all referencing blocks.

Example:

<table><tr><td>

A list of links is placed under each code block that references other blocks:

<a name="yarner-block-main-block" id="yarner-block-main-block"></a>
```rust
//- Main block
fn main() {
    // ==> Block A.
    // ==> Block B.
}
```

> Macros: [`Block A`](#yarner-block-block-a) [`Block B`](#yarner-block-block-b)

Blocks that are referenced by other blocks get a list of usages added.

The first referenced block:

<a name="yarner-block-block-a" id="yarner-block-block-a"></a>
```rust
//- Block A
print!("Hello");
```

> Usage: [`Main block`](#yarner-block-main-block)

The second referenced block:

<a name="yarner-block-block-b" id="yarner-block-block-b"></a>
```rust
//- Block B
println!(" World!");
```

> Usage: [`Main block`](#yarner-block-main-block)
</td></tr></table>

## Installation

**Binaries**

1. Download the [latest binaries](https://github.com/mlange-42/yarner-block-links/releases) for your platform  
2. Unzip somewhere
3. Add the parent directory of the executable to your `PATH` environmental variable

**Using `cargo`**

```
> cargo install yarner-block-links
```

## Usage

Add a section `plugin.block-links` to your `Yarner.toml`:

```toml
[plugin.block-links]
```

## Options

The plugin provides optional configuration for link formatting. Defaults are as follows (but all options can be left out):

```toml
[plugin.block-links]
template = "{{#if usage}}> Usage: {{usage}}  \n{{/if}}{{#if macros}}> Macros: {{macros}}{{/if}}"
join = " "
label = "`{{label}}`"
```

| Option     | Details                                         |
|------------|-------------------------------------------------|
| `template` | Template for formatting of the links section(s) |
| `join`     | Separator between links                         |
| `label`    | Formatting of link labels                       |
