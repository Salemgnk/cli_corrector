# CLI Corrector
CLI Corrector is an intelligent command-line interface (CLI) developed in Python to help developers automatically fix mistyped commands (e.g. `gti` → `git`). It offers quick suggestions, records a history of corrections, and can suggest aliases for common errors. This project is open source and designed to be simple, fast, and extensible.

## Features

- **Autocorrect:** Detects typos in CLI commands using Levenshtein distance.
- **Smart Suggestions:** Offers similar commands based on installed tools ($PATH).
- **History:** Stores corrections in a JSON file for future customization.
- **Aliases :** Recommends shell aliases for repeated errors (e.g. alias `gti`=`git`).
- **Light and fast :** Optimized for sub-0.5 second runtime.

## Prerequisites

- Python 3.8 or higher
- Supported systems: Linux, macOS, Windows (WSL recommended)
- Python libraries: prompt_toolkit, difflib (included in Python)

## Installation

1. Clone the :
```bash
git clone git@github.com:Salemgnk/cli_corrector.git
cd cli-corrector
```

2. Create a virtual environment (optional, recommended):
```bash
python -m venv venv
source venv/bin/activate # Linux/macOS
venv\Scripts\activate # Windows
```

3. Install the dependencies:
```bash
pip install -r requirements.txt
```

4. Launch the CLI:
```bash
python cli_corrector.py
```

## Usage
Type a command in the Corrector CLI, and if it's badly written, a suggestion will be displayed.
**Example:**
```bash
$ gti status
Did you mean 'git status'?
```
To activate a suggestion, follow the instructions displayed. Correction history is saved in `cli_corrector_history.json`.

## Contribute
We welcome contributions! Here's how to get started:

1. Fork the repository.

2. Create a branch for your feature (`git checkout -b feature/new-feature`).

3. Commit your changes (`git commit -m “Add new feature”`).

4. Push your branch (`git push origin feature/new-function`).

5. Open a Pull Request.

## Roadmap

- [] Integration of a lightweight AI model for contextual suggestions.
- [] Support for parsing command arguments (e.g. gti status → git status).
- [] Automatic alias generation in .bashrc or .zshrc.

## License
This project is licensed under the MIT License. You are free to use, modify and distribute it.

## Contact
For questions or suggestions, open an issue on GitHub or contact `gnandisalem@gmail.com`.

⭐ If you like this project, give it a star on github.

