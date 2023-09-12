{
  config,
  pkgs,
  lib,
  ...
}: {
  options = with lib; {
    services.i2c-rtc-start = {
      enable = mkEnableOption "Whether to enable the audio recording service.";

      i2c-bus = mkOption {
        type = types.int;
        default = 1;
        description = "The I2C bus to use.";
      };
    };
  };

  config = lib.mkIf config.services.i2c-rtc-start.enable {
    systemd.services.i2c-rtc-start = {
      description = "RTC start service; reads the RTC and sets the system time from it";
      wantedBy = ["multi-user.target"];
      requires = ["systemd-modules-load.service"];
      after = ["systemd-modules-load.service"];
      script = ''
        #!/usr/bin/env bash
        set -x
        # Remove the i2c device if it already exists
        if [ -e /sys/class/i2c-adapter/i2c-${toString config.services.i2c-rtc-start.i2c-bus}/new_device ]; then
            echo "Try deleting existing i2c device"
            echo 0x68 | tee /sys/class/i2c-adapter/i2c-${toString config.services.i2c-rtc-start.i2c-bus}/delete_device || true
        fi
        echo "Adding i2c device"
        echo ds1307 0x68 | tee /sys/class/i2c-adapter/i2c-${toString config.services.i2c-rtc-start.i2c-bus}/new_device
        echo "Current hwclock time:"
        /run/current-system/sw/bin/hwclock -r
        echo "Current system time:"
        date
        echo "Setting system time from hwclock"
        /run/current-system/sw/bin/hwclock -s
      '';
      serviceConfig = {
        User = "root";
        Type = "oneshot";
      };
      startLimitIntervalSec = 0;
    };
  };
}
