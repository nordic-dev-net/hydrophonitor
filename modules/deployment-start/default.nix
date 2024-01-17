{
  config,
  pkgs,
  lib,
  ...
}: {
  options = with lib; {
    services.deployment-start = {
      enable = mkEnableOption "Create deployment directory and export path as environment variable for services to write their output data to.";

      output-folder = mkOption {
        type = types.str;
        default = "/output";
        description = "The folder where deployment directory will be created.";
      };
    };
  };

  config = lib.mkIf config.services.deployment-start.enable {
    systemd.services.deployment-start = {
      description = "Deployment directory creation service";
      wantedBy = ["multi-user.target"];
      script = ''
        #!/usr/bin/env bash
        set -x
        DEPLOYMENT_DIRECTORY=${config.services.deployment-start.output-folder}/$(${pkgs.coreutils}/bin/date +"%Y-%m-%dT%H_%M_%S%z")
        ${pkgs.systemd}/bin/systemctl set-environment DEPLOYMENT_DIRECTORY=$DEPLOYMENT_DIRECTORY
        ${pkgs.coreutils}/bin/mkdir -p $DEPLOYMENT_DIRECTORY
      '';
      serviceConfig = {
        User = "root";
        Type = "oneshot";
      };
      unitConfig = {
        Before = ["audio-recorder.service"];
      };
    };
  };
}
