use std::{fs, path::Path};

pub fn create_build_release_nix_file(project_dir: &str, project_name: &str) -> Result<(), String> {
    let content = format!(r#"{{
  pkgs ? import <nixpkgs> {{}}:

  let
    # Define the project directory
    projectDir = "{}";

    # Define the linked Rust and Cargo toolchains
    linkedRust = "${{projectDir}}/.rustup";
    linkedCargo = "${{projectDir}}/.cargo";

    # Define the destination directory for the installed program
    installDir = "/bin";  # Installing to a different directory within the Nix store
  in

  pkgs.stdenv.mkDerivation {{
    name = "{name}";

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
      cargo build --release
    '';

    # Install script
    installPhase = ''
      mkdir -p $out$installDir
      cp target/release/{name} $out$installDir
    '';
  }}
}}"#, project_dir, name = project_name);

let file_path = Path::new(project_dir).join("buildRelease.nix");
fs::write(&file_path, content)
    .map_err(|e| format!("Failed to create file '{}': {}", file_path.display(), e))?;

Ok(())
}
