import difflib
import json
from prompt_toolkit import PromptSession
from prompt_toolkit.history import FileHistory
import os
import subprocess

HISTORY_FILE = 'cli_corrector_history.json'
CONFIG_FILE = "cli_corrector_config.json"
AUTO_CORRECT_THRESHOLD = 7

def load_command():
    """Load available commands from $PATH"""
    commands = set()
    for path in os.environ['PATH'].split(os.pathsep):
        try:
            for cmd in os.listdir(path):
                if os.access(os.path.join(path, cmd), os.X_OK):
                    commands.add(cmd)
        except (FileNotFoundError, PermissionError):
            continue
    return list(commands)

def load_config():
    """Load configuration from JSON file."""
    if os.path.exists(CONFIG_FILE):
        with open(CONFIG_FILE, "r") as f:
            try:
                return json.load(f)
            except json.JSONDecodeError:
                return {"auto_correct": {}}
    return {"auto_correct": {}}

def save_config(config):
    """Save configuration to JSON file."""
    with open(CONFIG_FILE, "w") as f:
        json.dump(config, f, indent=4)

def save_corrections(mistyped, suggested):
    """Save corrections to the configuration file."""
    history = []
    if os.path.exists(HISTORY_FILE):
        with open(HISTORY_FILE, 'r') as f:
            try:
                history = json.load(f)
            except json.JSONDecodeError:
                print(f"Error {HISTORY_FILE} is corrupted. Starting fresh.")
    
    history.append({"mistyped": mistyped, "suggested": suggested})
    with open(HISTORY_FILE, 'w') as f:
        json.dump(history, f, indent=4)
    return history

def count_corrections(mistyped, suggested, history):
    "Count the number of corrections made in the history."
    return sum(1 for entry in history if entry["mistyped"] == mistyped
               and entry["suggested"] == suggested)

def suggest_correction(mistyped, commands):
    """Suggest a correction for the mistyped command."""
    suggestion = difflib.get_close_matches(mistyped, commands, n=1, cutoff=0.6)

def main():
    print("Welcome to CLI Corrector!")


if __name__ == "__main__":
    main()
