use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};

/// Função para percorrer diretórios e executar `cargo clean` se `Cargo.toml` for encontrado
fn clean_cargo_projects(start_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !start_dir.is_dir() {
        return Err(format!("O diretório '{}' não existe ou não é um diretório válido.", start_dir.display()).into());
    }

    // Iterar recursivamente pelos diretórios
    for entry in fs::read_dir(start_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            clean_cargo_projects(&path)?; // Recursão para subdiretórios
        } else if path.is_file() && path.file_name().unwrap() == "Cargo.toml" {
            let project_dir = path.parent().unwrap();
            println!("Encontrado Cargo.toml em {}. Executando 'cargo clean'...", project_dir.display());
            let status = Command::new("cargo")
                .arg("clean")
                .current_dir(project_dir)
                .status()?;

            if !status.success() {
                eprintln!("Erro ao executar 'cargo clean' em {}", project_dir.display());
            }
        }
    }

    Ok(())
}

fn main() {
    // Obter argumento da linha de comando
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: {} <diretório_inicial>", args[0]);
        exit(1);
    }

    let start_dir = Path::new(&args[1]);

    // Executar a função principal e lidar com erros
    if let Err(e) = clean_cargo_projects(start_dir) {
        eprintln!("Erro: {}", e);
        exit(1);
    }
}
