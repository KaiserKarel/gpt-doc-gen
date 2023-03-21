`gpt-doc-gen` is a command-line interface (CLI) tool for generating Rust doc comments using OpenAI's [`Dali`] language model. It will document all public items, including adding examples, `# Errors` sections and cross linking types from other crates.

# Installation

```bash
cargo install gpt-doc-gen
```

Or build the binary locally:

```bash
git clone git@github.com/kaiserkarel/gpt-doc-gen
cd gpt-doc-gen
cargo install --path .
```

# Usage

The CLI parameters are still unstable and may change.


```
cargo gpt-doc-gen --help

--- 
gpt-doc-gen 0.1.0
A CLI to document Rust code

USAGE:
    gpt-doc-gen [FLAGS] --api-key <api-key> <SUBCOMMAND>

FLAGS:
    -d, --dryrun     Print the output to stdout instead of writing it to a file
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --api-key <api-key>     [env: GPT_DOC_GEN_API_KEY=$API_KEY]

SUBCOMMANDS:
    document-crate    Add doc comments to all Rust files in a crate
    document-file     Add doc comments to a single file
    help              Prints this message or the help of the given subcommand(s)
```

`gpt-doc-gen` can add documentation to a specific file, 