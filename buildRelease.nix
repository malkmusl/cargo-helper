{ pkgs ? import <nixpkgs> {} }:

let
  # Define the project directory
  projectDir = "/home/malkmusl/Development/rust/cargo-helper/";

  # Define the linked Rust and Cargo toolchains
  linkedRust = "${projectDir}.rustup";
  linkedCargo = "${projectDir}.cargo";

  # Define the destination directory for the installed program
  installDir = "/bin";  # Installing to a different directory within the Nix store
  executablePath = "${installDir}/cargo-helper";
in

pkgs.stdenv.mkDerivation {
  name = "cargo-helper";

  # Specify build inputs
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
  ];

  # Set the project directory as the build directory
  src = ./.;

  # Set environment variables to use the linked toolchains
  shellHook = ''
    export RUSTUP_HOME="${linkedRust}"
    export CARGO_HOME="${linkedCargo}"
  '';

  # Build script
  buildPhase = ''
    # Build the project
    cargo build --release
  '';

  installPhase = ''
    mkdir -p $out/bin  # Ensure the target directory exists
    cp -v target/release/cargo-helper $out/bin/  # Copy the executable
    # Add the export statement to modify the PATH in the user's .bashrc
    export PATH="/nix/store/il3q6l8riizn1pijmay2m47idgdsx1y9-cargo-helper/usr/bin:$PATH"

  '';


}


