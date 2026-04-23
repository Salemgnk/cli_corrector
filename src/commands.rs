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
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name_str) = path.file_name().and_then(|f| f.to_str()) {
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
        // Optimization: checking string length difference is O(1).
        // We do this before calculating the O(N*M) Levenshtein distance
        // to prune impossible matches quickly.
        // Si la commande est beaucoup plus longue ou plus courte, on ignore
        if (cmd.len() as isize - mistyped.len() as isize).abs() > 3 {
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
