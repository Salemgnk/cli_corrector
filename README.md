# CLI Corrector

CLI Corrector is an intelligent command-line interface (CLI) rewritten in **Rust** to help developers automatically fix mistyped commands (e.g., `gti` → `git`). It offers lightning-fast suggestions, records a history of corrections, and can automatically execute corrected commands.

This new Rust version replaces the old Python REPL, acting as a native CLI utility that you can easily integrate into your shell.

## Features

- **Blazing Fast:** Written in Rust, it launches and executes in milliseconds.
- **Fuzzy Matching:** Detects typos using Levenshtein distance.
- **Smart AI Suggestions:** Can use Gemini API for advanced, context-aware corrections.
- **Auto-Learning:** If you make the same typo 3 times, it offers to auto-correct it permanently.
- **Native Integration:** Instead of a REPL, pass your failed command directly to the tool.

## Prerequisites

- [Rust & Cargo](https://rustup.rs/)

## Installation

1. Clone the repository:
```bash
git clone git@github.com:Salemgnk/cli_corrector.git
cd cli_corrector
```

2. Build and install the binary globally using Cargo:
```bash
cargo install --path .
```

*Note: Ensure your `~/.cargo/bin` is in your `$PATH`.*

## Usage

Instead of launching a separate prompt, you pass the mistyped command directly to `cli_corrector`:

```bash
$ cli_corrector gti status
Did you mean: 'git status' ? [y/N] y
... (git status output) ...
```

If you type the same error 3 times, the tool will ask to enable auto-correction:
```bash
'gti' has been corrected to 'git' 3 times. Enable auto-correction? [y/N]
```

### Manual Corrections

You can manually add a permanent correction:
```bash
$ cli_corrector correct sl ls
Manual correction added: sl -> ls
```

Now, running `cli_corrector sl` will automatically execute `ls`.

## Shell Integration (Alias)

To make it even faster, you can alias it in your `~/.bashrc` or `~/.zshrc`:
```bash
alias c="cli_corrector"
```
So you can just type:
```bash
$ c gti status
```

## AI Suggestions (Gemini)

To enable smart suggestions via Google's Gemini, simply export your API key before running:

```bash
export GEMINI_API_KEY="your_api_key_here"
```

The tool will query the LLM first. If it fails or no key is provided, it instantly falls back to local fuzzy matching.

## Configuration & History

Your configuration (auto-corrections) and history are now stored safely in your user folder:
- Linux/macOS: `~/.config/cli_corrector/`
- Windows: `C:\Users\Username\AppData\Roaming\cli_corrector\`

## Contribute
We welcome contributions!

1. Fork the repository.
2. Create a branch (`git checkout -b feature/new-feature`).
3. Commit your changes.
4. Push your branch.
5. Open a Pull Request.

## License
MIT License.
