use std::fs;
use std::path::Path;

pub fn create_build_debug_nix_file(project_dir: &str, project_name: &str) -> Result<(), String> {
    let content = format!(r#"{{
  pkgs ? import <nixpkgs> {{}}:

  let
    # Define the project directory
    projectDir = "{project_dir}";

    # Define the linked Rust and Cargo toolchains
    linkedRust = "$projectDir/.rustup";
    linkedCargo = "$projectDir/.cargo";

    # Define the destination directory for the installed program
    installDir = "/bin";  # Installing to a different directory within the Nix store
  in

  pkgs.stdenv.mkDerivation {{
    name = "{project_name}";

    # Specify build inputs
    buildInputs = [
      pkgs.rustc
      pkgs.cargo
    ];

    # Set the project directory as the build directory
    src = projectDir;

    # Set environment variables to use the linked toolchains
    shellHook = ''
      export RUSTUP_HOME="${{linkedRust}}"
      export CARGO_HOME="${{linkedCargo}}"
    '';

    # Build script
    buildPhase = ''
      # Build the project
      cargo build
    '';

    # Install script
    installPhase = ''
      mkdir -p $out$installDir
      cp target/debug/{project_name} $out$installDir
    '';
  }}
}}"#, project_dir = project_dir, project_name = project_name);

    let file_path = Path::new(project_dir).join("buildDebug.nix");
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to create file '{}': {}", file_path.display(), e))?;

    Ok(())
}

