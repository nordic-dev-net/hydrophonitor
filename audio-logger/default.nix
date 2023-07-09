{
  lib,
  rustPlatform,
  pkg-config,
  pkgs,
}:
rustPlatform.buildRustPackage {
  pname = "audio-logger";
  version = "0.1.0";

  nativeBuildInputs = [pkg-config];

  buildInputs = with pkgs; [ jack1 jack2 alsaLib alsaUtils ];

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
