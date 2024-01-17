{
  config,
  pkgs,
  lib,
  ...
}: {
  options = with lib; {
    services.journalctl-log-export = {
      enable = mkEnableOption "Enable exporting journalctl logs to a file.";

      output-folder = mkOption {
        type = types.str;
        default = "logs";
        description = "The folder to save logs to within the deployment folder.";
      };
    };
  };

  config = lib.mkIf config.services.journalctl-log-export.enable {
    systemd.services.journalctl-log-export-on-shutdown = {
      description = "Journalctl log export on shutdown service";
      wantedBy = ["multi-user.target"];
      serviceConfig = {
        User = "root";
        Type = "oneshot";
        RemainAfterExit = true;
        ExecStop = ''${pkgs.bash}/bin/bash -c "echo \"Exporting logs at shutdown\" && journalctl --boot 0 > $DEPLOYMENT_DIRECTORY/${config.services.journalctl-log-export.output-folder}/journalctl_on_shutdown.txt"'';
      };
      unitConfig = {
        Before = ["audio-recorder.service"];
      };
    };

    systemd.services.journalctl-log-export = {
      description = "Journalctl log export service";
      script = ''
        #!/usr/bin/env bash
        set -x
        ${pkgs.coreutils}/bin/echo "Exporting journalctl logs"
        OUTPUT_PATH=$DEPLOYMENT_DIRECTORY/${config.services.journalctl-log-export.output-folder}
        ${pkgs.coreutils}/bin/mkdir -p $OUTPUT_PATH
        # if environment variable for output file name is not set, set it with systemctl set-environment
        if [ -z "$JOURNALCTL_LOG_EXPORT_OUTPUT_FILE" ]; then
          JOURNALCTL_LOG_EXPORT_OUTPUT_FILE=$(${pkgs.coreutils}/bin/date +"%Y-%m-%dT%H_%M_%S%z")_journalctl.txt
          ${pkgs.systemd}/bin/systemctl set-environment JOURNALCTL_LOG_EXPORT_OUTPUT_FILE=$JOURNALCTL_LOG_EXPORT_OUTPUT_FILE
        fi
        ${pkgs.systemd}/bin/journalctl --boot 0 > $OUTPUT_PATH/$JOURNALCTL_LOG_EXPORT_OUTPUT_FILE
      '';
      serviceConfig = {
        User = "root";
        Type = "oneshot";
      };
    };

    systemd.timers.journalctl-log-export = {
      wantedBy = ["timers.target"];
      partOf = ["journalctl-log-export.service"];
      timerConfig = {
        OnCalendar = "*:0/5"; # every five minutes
        Unit = "journalctl-log-export.service";
      };
      unitConfig = {
        After = ["multi-user.target" "deployment-start.service"];
      };
    };
  };
}
