use std::collections::HashSet;
use std::env;
use std::fs;

use strsim::levenshtein;

pub fn load_available_commands() -> Vec<String> {
    let mut commands = HashSet::new();

    if let Ok(path_var) = env::var("PATH") {
        for path in env::split_paths(&path_var) {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    // Performance optimization: use `entry.file_type()` instead of `entry.path().is_file()`
                    // to avoid redundant `stat` system calls. Also use `entry.file_name()` directly
                    // to avoid allocating unnecessary `PathBuf` objects.
                    let is_file = match entry.file_type() {
                        Ok(ft) => ft.is_file() || (ft.is_symlink() && entry.path().is_file()),
                        Err(_) => entry.path().is_file(),
                    };

                    if is_file {
                        if let Some(name_str) = entry.file_name().to_str() {
                            // En Rust, la vérification d'exécutabilité dépend de l'OS.
                            // Pour faire simple, on ajoute tous les fichiers du $PATH
                            // On pourrait affiner avec std::os::unix::fs::PermissionsExt
                            commands.insert(name_str.to_string());
                        }
                    }
                }
            }
        }
    }

    commands.into_iter().collect()
}

pub fn suggest_command_local(mistyped: &str, commands: &[String]) -> Option<String> {
    // Quelques corrections connues qu'on peut hardcoder ou charger depuis la config
    let known_corrections: [(&str, &str); 7] = [
        ("gti", "git"),
        ("grpe", "grep"),
        ("cd..", "cd .."),
        ("mkaedir", "mkdir"),
        ("clera", "clear"),
        ("pyhton", "python"),
        ("exiy", "exit"),
    ];

    for (k, v) in known_corrections.iter() {
        if *k == mistyped {
            return Some(v.to_string());
        }
    }

    let mut best_match = None;
    let mut best_score = usize::MAX; // distance la plus petite = meilleur score

    for cmd in commands {
        // Performance optimization: check string length differences (O(1)) before
        // calculating the expensive Levenshtein distance (O(N*M)) to prune impossible matches quickly.
        // Si la commande est beaucoup plus longue ou plus courte, on ignore
        if cmd.len().abs_diff(mistyped.len()) > 3 {
            continue;
        }

        let distance = levenshtein(mistyped, cmd);

        // 3 est un seuil arbitraire pour la distance de Levenshtein
        if distance < best_score && distance <= 3 {
            best_score = distance;
            best_match = Some(cmd.clone());
        }
    }

    best_match
}
