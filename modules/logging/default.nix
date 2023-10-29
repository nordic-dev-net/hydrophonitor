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
        default = "/output/logs";
        description = "The folder to save logs to.";
      };
    };
  };

  config = lib.mkIf config.services.journalctl-log-export.enable {
    systemd.services.journalctl-log-export-on-shutdown = {
      description = "Journalctl log export on shutdown service";
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        User = "root";
        Type = "oneshot";
        RemainAfterExit = true;
        ExecStart = "${pkgs.coreutils}/bin/mkdir -p ${config.services.journalctl-log-export.output-folder}";
        ExecStop = ''${pkgs.bash}/bin/bash -c "echo 'Exporting logs at shutdown' && journalctl --boot 0 > ${config.services.journalctl-log-export.output-folder}/$(date +\"%Y_%m_%d-%H_%M_%S\")_journalctl_on_shutdown.txt"'';
      };
    };

    systemd.services.journalctl-log-export = {
      description = "Journalctl log export service";
      script = ''
        #!/usr/bin/env bash
        set -x
        ${pkgs.coreutils}/bin/echo "Exporting journalctl logs"
        ${pkgs.coreutils}/bin/mkdir -p ${config.services.journalctl-log-export.output-folder}
        # if environment variable for output file name is not set, set it with systemctl set-environment
        if [ -z "$JOURNALCTL_LOG_EXPORT_OUTPUT_FILE" ]; then
          JOURNALCTL_LOG_EXPORT_OUTPUT_FILE=$(${pkgs.coreutils}/bin/date +"%Y_%m_%d-%H_%M_%S")_journalctl.txt
          ${pkgs.systemd}/bin/systemctl set-environment JOURNALCTL_LOG_EXPORT_OUTPUT_FILE=$JOURNALCTL_LOG_EXPORT_OUTPUT_FILE
        fi
        ${pkgs.systemd}/bin/journalctl --boot 0 > ${config.services.journalctl-log-export.output-folder}/$JOURNALCTL_LOG_EXPORT_OUTPUT_FILE
      '';
      serviceConfig = {
        User = "root";
        Type = "oneshot";
      };
    };

    systemd.timers.journalctl-log-export = {
    wantedBy = [ "timers.target" ];
    partOf = [ "journalctl-log-export.service" ];
    timerConfig = {
        OnCalendar = "*:0/5"; # every five minutes
        Unit = "journalctl-log-export.service";
    };
};
  };
}
