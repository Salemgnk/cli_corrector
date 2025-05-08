import difflib
import json
from prompt_toolkit import PromptSession
from prompt_toolkit.completion import FileHistory
import os
import subprocess

HISTORY_FILE = 'cli_corrector_history.json'
CONFIG_FILE = "cli_corrector_config.json"

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