{
  description = "NixOS Raspberry Pi configuration flake";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    nixos-hardware.url = "github:NixOS/nixos-hardware/master";
    flake-utils.url = "github:numtide/flake-utils";
    deploy-rs.url = "github:serokell/deploy-rs";
    hydrophonitor-gps.url = "github:nordic-dev-net/hydrophonitor-gps";
    depth-recorder.url = "github:nordic-dev-net/depth-recorder";
  };
  outputs = {
    self,
    nixpkgs,
    nixos-hardware,
    flake-utils,
    deploy-rs,
    hydrophonitor-gps,
    depth-recorder,
    ...
  }: let
    forEachSystem = nixpkgs.lib.genAttrs ["x86_64-linux" "aarch64-linux"];
    forEachPkgs = f: forEachSystem (sys: f (nixpkgs.legacyPackages.${sys}));
    system = "aarch64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config = {allowUnfree = true;};
      overlays = [
        (final: super: {
          makeModulesClosure = x:
            super.makeModulesClosure (x // {allowMissing = true;});
        })
      ];
    };
  in {
    systems = {
      raspberry-pi-4 = nixpkgs.lib.nixosSystem {
        system = "aarch64-linux";
        specialArgs = {inherit pkgs;};
        modules = [
          ./targets/raspberry-pi-4
          ./modules/audio-recorder
          ./modules/real-time-clock/i2c-rtc.nix
          ./modules/shutdown-button/service.nix
          nixos-hardware.nixosModules.raspberry-pi-4
          hydrophonitor-gps.nixosModules.hydrophonitor-gps
          depth-recorder.nixosModules.depth-recorder
          "${nixpkgs}/nixos/modules/installer/sd-card/sd-image-aarch64.nix"
        ];
      };

      raspberry-pi-3 = nixpkgs.lib.nixosSystem {
        system = "aarch64-linux";
        specialArgs = {inherit pkgs;};
        modules = [
          ./targets/raspberry-pi-3
          ./modules/audio-recorder
          ./modules/real-time-clock/i2c-rtc.nix
          hydrophonitor-gps.nixosModules.hydrophonitor-gps
          "${nixpkgs}/nixos/modules/installer/sd-card/sd-image-aarch64.nix"
        ];
      };
    };

    deploy.nodes = {
      raspberry-pi-4 = {
        hostname = "192.168.1.76";
        profiles.system = {
          sshUser = "kaskelotti";
          sshOpts = ["-t"];
          magicRollback = false;
          path =
            deploy-rs.lib.aarch64-linux.activate.nixos
            self.systems.raspberry-pi-4;
          user = "root";
        };
      };

      raspberry-pi-3 = {
        hostname = "192.168.1.117";
        profiles.system = {
          sshUser = "kaskelotti";
          sshOpts = ["-t"];
          magicRollback = false;
          path =
            deploy-rs.lib.aarch64-linux.activate.nixos
            self.systems.raspberry-pi-3;
          user = "root";
        };
      };
    };

    formatter = forEachPkgs (pkgs: pkgs.alejandra);
  };
}
