import os
import sys
import time
import json
import getpass
import requests
import datetime
import subprocess
from dotenv import load_dotenv
from rapidfuzz import process, fuzz
from prompt_toolkit import PromptSession
from prompt_toolkit.history import FileHistory
from prompt_toolkit.formatted_text import HTML

load_dotenv()

HISTORY_FILE = "cli_corrector_history.json"
CONFIG_FILE = "cli_corrector_config.json"
AUTO_CORRECT_THRESHOLD = 3

GEMINI_API_KEY = os.environ.get("GEMINI_API_KEY")
GEMINI_API_URL = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent"

KNOWN_CORRECTIONS = {
    "gti": "git",
    "grpe": "grep",
    "cd..": "cd ..",
    "mkaedir": "mkdir",
    "clera": "clear",
    "pyhton": "python",
    "exiy": "exit",
}

BUILTIN_COMMANDS = {"cd", "exit", "clear", "alias", "unalias"}

def load_available_commands():
    commands = set()
    for path in os.environ.get("PATH", "").split(os.pathsep):
        try:
            for cmd in os.listdir(path):
                full_path = os.path.join(path, cmd)
                if os.access(full_path, os.X_OK):
                    commands.add(cmd)
        except (FileNotFoundError, PermissionError):
            continue
    return list(commands)


def load_config():
    if os.path.exists(CONFIG_FILE):
        with open(CONFIG_FILE, "r") as f:
            try:
                return json.load(f)
            except json.JSONDecodeError:
                print(f"Warning: {CONFIG_FILE} is corrupted. Resetting.")
                return {"auto_correct": {}}
    return {"auto_correct": {}}


def save_config(config):
    with open(CONFIG_FILE, "w") as f:
        json.dump(config, f, indent=2)


def load_history():
    if os.path.exists(HISTORY_FILE):
        try:
            with open(HISTORY_FILE, "r") as f:
                return json.load(f)
        except json.JSONDecodeError:
            print(f"Warning: {HISTORY_FILE} is corrupted. Resetting.")
    return {}


def save_history(history):
    with open(HISTORY_FILE, "w") as f:
        json.dump(history, f, indent=2)


def update_history(history, mistyped, suggested):
    key = f"{mistyped} -> {suggested}"
    history[key] = history.get(key, 0) + 1
    save_history(history)
    return history[key]


def suggest_command(mistyped, commands):
    if mistyped in KNOWN_CORRECTIONS:
        return KNOWN_CORRECTIONS[mistyped]

    if GEMINI_API_KEY:
        prompt = f"""
        The user typed the incorrect command '{mistyped}'.
        Suggest the correct command from the available commands: {', '.join(commands)}.
        Provide only the correct command without explanation.
        """
        headers = {"Content-Type": "application/json"}
        data = {"contents": [{"parts": [{"text": prompt.strip()}]}]}

        try:
            response = requests.post(
                f"{GEMINI_API_URL}?key={GEMINI_API_KEY}",
                headers=headers,
                json=data,
                timeout=5
            )
            response.raise_for_status()
            result = response.json()
            suggestion = result.get("candidates", [{}])[0].get("content", {}).get("parts", [{}])[0].get("text", "").strip()
            if suggestion in commands:
                return suggestion
        except requests.RequestException:
            pass
        except Exception:
            pass

    result = process.extractOne(mistyped, commands, scorer=fuzz.ratio, score_cutoff=70)
    return result[0] if result else None


def execute_command(command):
    try:
        subprocess.run(command, shell=True)
    except subprocess.SubprocessError as e:
        print(f"Execution error: {e}")


def smart_clear():
    print("Auto-correcting to 'clear'... [press Enter or wait 1s]")
    time.sleep(1)
    os.system('clear')


def propose_alias(mistyped, suggested):
    print(f"Do you want to create alias '{mistyped}={suggested}' in your shell? [y/n]")
    if input().strip().lower() == "y":
        shell_config = os.path.expanduser("~/.bashrc")
        alias_line = f"alias {mistyped}='{suggested}'\n"
        try:
            with open(shell_config, "a") as f:
                f.write(alias_line)
            print(f"Alias added to {shell_config}. Run 'source {shell_config}' to apply it.")
        except Exception as e:
            print(f"Alias creation error: {e}")
    else:
        print("Alias not created.")


def add_manual_correction(mistyped, corrected, config, history):
    if corrected not in load_available_commands() and corrected.split()[0] not in BUILTIN_COMMANDS:
        print(f"Error: '{corrected}' is not a valid command.")
        return False
    KNOWN_CORRECTIONS[mistyped] = corrected
    auto_correct = config.get("auto_correct", {})
    auto_correct[mistyped] = corrected
    config["auto_correct"] = auto_correct
    save_config(config)
    update_history(history, mistyped, corrected)
    print(f"Correction '{mistyped} -> {corrected}' added and enabled for auto-correction.")
    return True


def print_stats(history, config):
    print("\n📊 CLI Correction Stats\n")
    reverse_map = {}

    for key, count in history.items():
        if "->" not in key:
            continue
        mistyped, corrected = [x.strip() for x in key.split("->")]
        reverse_map.setdefault(corrected, {})[mistyped] = count

    if not reverse_map:
        print("No correction history found.")
    else:
        print("🔁 Frequent corrected commands:\n")
        for corrected_cmd, mistyped_map in reverse_map.items():
            sorted_mistyped = sorted(mistyped_map.items(), key=lambda x: x[1], reverse=True)
            formatted = ", ".join(f"{mistyped} ({count})" for mistyped, count in sorted_mistyped)
            print(f"{corrected_cmd} ← {formatted}")
        print()

    auto_correct = config.get("auto_correct", {})
    if auto_correct:
        print("⚙️  Auto-corrections enabled:\n")
        for mistyped, corrected in auto_correct.items():
            print(f"{mistyped} → {corrected}")
        print()

    if KNOWN_CORRECTIONS:
        print("📚 Known corrections:\n")
        for mistyped, corrected in KNOWN_CORRECTIONS.items():
            print(f"{mistyped} → {corrected}")
        print()


def get_custom_prompt():
    user = getpass.getuser()
    cwd = os.getcwd()
    time_now = datetime.datetime.now().strftime("%H:%M:%S")
    return HTML(f"<ansicyan>{user}</ansicyan> <ansigreen>{cwd}</ansigreen> <ansiblue>{time_now}</ansiblue> > ")


def main():
    available_commands = load_available_commands()
    config = load_config()
    history = load_history()
    session = PromptSession(history=FileHistory(".cli_corrector_prompt_history"))

    print("""
CLI Corrector - Command Line Interface Correction Tool
----------------------------------------------------
- Enter a command to execute it or get correction suggestions.
- Type 'correct <mistyped> <corrected>' to manually add a correction (e.g., 'correct sl ls').
- Type 'stats' to see correction statistics.
- Type 'quit' or 'exit' to leave the program.
----------------------------------------------------
    """)

    while True:
        try:
            user_input = session.prompt(get_custom_prompt()).strip()
            if user_input.lower() in {"quit", "exit"}:
                break

            if user_input.lower() == "stats":
                print_stats(history, config)
                continue

            parts = user_input.split(maxsplit=2)
            cmd = parts[0] if parts else ""

            if cmd == "correct" and len(parts) == 3:
                mistyped, corrected = parts[1], parts[2]
                add_manual_correction(mistyped, corrected, config, history)
                continue

            args = parts[1] if len(parts) > 1 else ""

            if not cmd:
                continue

            # === GESTION DES COMMANDES INTERNES ===
            if cmd == "cd":
                path = args if args else os.path.expanduser("~")
                try:
                    os.chdir(path)
                except FileNotFoundError:
                    print(f"No such directory: {path}")
                except Exception as e:
                    print(f"cd error: {e}")
                continue

            if cmd == "exit":
                print("Goodbye !")
                break

            if cmd == "clear":
                smart_clear()
                continue

            if cmd in available_commands:
                if cmd == "clear":
                    smart_clear()
                else:
                    execute_command(user_input)
                continue

            auto_correct = config.get("auto_correct", {})
            if cmd in auto_correct:
                corrected = f"{auto_correct[cmd]} {args}".strip()
                print(f"Auto-correcting '{cmd}' -> '{corrected}'")
                if corrected.strip() == "clear":
                    smart_clear()
                else:
                    execute_command(corrected)
                continue

            suggestion = suggest_command(cmd, available_commands)
            if suggestion:
                corrected = f"{suggestion} {args}".strip()

                print(f"Did you mean: '{corrected}' ? [y/n]", end=" ")
                resp = input().strip().lower()

                if resp == "y":
                    if corrected.strip() == "clear":
                        smart_clear()
                    else:
                        execute_command(corrected)
                    correction_count = update_history(history, cmd, suggestion)

                    if correction_count >= AUTO_CORRECT_THRESHOLD and cmd not in auto_correct:
                        print(f"'{cmd}' has been corrected to '{suggestion}' {correction_count} times.")
                        print("Enable auto-correction for this? [y/n]")
                        if input().strip().lower() == "y":
                            auto_correct[cmd] = suggestion
                            config["auto_correct"] = auto_correct
                            save_config(config)
                            print(f"Auto-correction enabled: '{cmd}' -> '{suggestion}'")
                        else:
                            propose_alias(cmd, suggestion)
                else:
                    print("Command canceled.")
            else:
                print(f"Unknown command: {cmd}")

        except KeyboardInterrupt:
            print("\nInterrupted. Type 'quit' to exit.")
        except Exception as e:
            print(f"Error: {e}")
    print("Goodbye !")


if __name__ == "__main__":
    main()
