{
  rustPlatform,
  pkg-config,
  pkgs,
}:
rustPlatform.buildRustPackage {
  pname = "shutdown-button";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };
}
