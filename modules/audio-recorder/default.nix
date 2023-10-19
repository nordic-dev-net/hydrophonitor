{
  config,
  pkgs,
  lib,
  ...
}: {
  options = with lib; {
    services.audio-recorder = {
      enable = mkEnableOption "Whether to enable the audio recording service.";

      output-folder = mkOption {
        type = types.str;
        default = "/output/audio";
        description = "The folder to save recordings to.";
      };

      sample-rate = mkOption {
        type = types.int;
        default = 44100;
        description = "The sample rate to use for recording.";
      };

      sample-format = mkOption {
        type = types.str;
        default = "S32_LE";
        description = "The sample format to use for recording.";
      };

      channels = mkOption {
        type = types.int;
        default = 2;
        description = "The amount of channels to use.";
      };

      max-file-time-secs = mkOption {
        type = types.int;
        default = 600;
        description = "The maximum length of a recording in seconds.";
      };
    };
  };

  config = lib.mkIf config.services.audio-recorder.enable {
    # Configure the Behringer UMC404HD to turn capture on for all four mics and set the mic volume to max
    # Sound card idVendor and idProduct fields can be checked from /proc/asound/card1/usbid (when card number is 1, check with arecord -l)
    services.udev.extraRules = ''
      # Behringer Uphoria UMC404HD rules
      # On connect, turn capture on for all mics and set the mic volume to max
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="1397", ATTRS{idProduct}=="0509", RUN+="${pkgs.writeShellScriptBin "umc404hd-autocapture" (builtins.readFile ./umc404hd-autocapture.sh)}"
    '';

    systemd.services.audio-recorder = {
      description = "Audio Recording Service";
      wantedBy = ["multi-user.target"];
      script = ''
        #!/usr/bin/env bash
        set -x
        ${pkgs.coreutils}/bin/mkdir -p ${config.services.audio-recorder.output-folder}
        ${pkgs.alsaUtils}/bin/arecord -l
        CARD_ID=$(${pkgs.alsaUtils}/bin/arecord -l | grep "USB Audio" | grep -oP '(?<=card )\d')
        DEVICE_ID=$(${pkgs.alsaUtils}/bin/arecord -l | grep "USB Audio" | grep -oP '(?<=device )\d')
        ${pkgs.alsaUtils}/bin/arecord \
            --device hw:$CARD_ID,$DEVICE_ID \
            --format ${config.services.audio-recorder.sample-format} \
            --max-file-time ${toString config.services.audio-recorder.max-file-time-secs} \
            --rate ${toString config.services.audio-recorder.sample-rate} \
            --channels ${toString config.services.audio-recorder.channels} \
            --use-strftime \
            ${config.services.audio-recorder.output-folder}/%Y-%m-%d_%H-%M-%S.wav
      '';
      serviceConfig = {
        User = "root"; # Replace with appropriate user
        Restart = "always";
      };
      unitConfig = {
        After = "sound.target";
      };
      startLimitIntervalSec = 0;
    };
  };
}
