{
  config,
  pkgs,
  lib,
  ...
}: let
  shutdownButton = pkgs.callPackage ./default.nix {};
in {
  options = {
    services.shutdown-button = {
      enable = lib.mkEnableOption "Whether to enable the shutdown-button service.";

      gpio-pin = lib.mkOption {
        type = lib.types.int;
        default = 21;
        description = "GPIO pin number to which button is connected";
      };

      shutdown-press-secs = lib.mkOption {
        type = lib.types.int;
        default = 1;
        description = "How many seconds button must be pushed down to trigger shutdown signal";
      };
    };
  };

  config = lib.mkIf config.services.shutdown-button.enable {
    systemd.services.shutdown-button = {
      description = "Shutdown button service";
      wantedBy = ["multi-user.target"];
      script = ''
        #!/usr/bin/env bash
        set -x
        ${shutdownButton}/bin/shutdown-button --gpio-pin ${toString config.services.shutdown-button.gpio-pin} --shutdown-press-secs ${toString config.services.shutdown-button.shutdown-press-secs}
      '';
      serviceConfig = {
        User = "root"; # Replace with appropriate user
        Restart = "always";
      };
      startLimitIntervalSec = 0;
    };
  };
}
