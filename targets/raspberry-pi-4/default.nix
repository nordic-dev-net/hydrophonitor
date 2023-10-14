{
  config,
  pkgs,
  ...
}: let
  system = "aarch64-linux";
in {
  system.stateVersion = "23.05";
  sdImage.compressImage = false;

  # disable internal sound card and vc4 gpu
  # to set USB sound card as default
  boot.blacklistedKernelModules = ["snd_bcm2835" "vc4"];
  # enable i2c and rtc modules
  boot.kernelModules = ["i2c-dev" "i2c_bcm2708" "rtc_ds1307"];

  hardware = {
    raspberry-pi."4".i2c1.enable = true;
    raspberry-pi."4".apply-overlays-dtmerge.enable = true;
    deviceTree.filter = "bcm2711-rpi-4*.dtb";
  };

  environment.systemPackages = with pkgs; [i2c-tools];

  users.users.kaskelotti = {
    isNormalUser = true;
    initialHashedPassword = "$6$ySDQdXbGH/qDvjpe$Jp5icbEFRSBLsxB2XGxFz.dACxOS/.KYHENxVSUzFED0UYi9R64858JevedVB06sTsFvlKOPSlzBvbACbxNZr1";
    extraGroups = ["wheel" "networkmanager"];
  };
  nix.settings.trusted-users = ["kaskelotti"];

  sound.enable = true;

  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = true;
  };

  services.audio-recorder = {
    enable = true;
    output-folder = "/output/audio";
    sample-rate = 192000;
    sample-format = "S32_LE";
    channels = 4;
    max-file-time-secs = 60;
  };

  services.gps-recorder = {
    enable = true;
    output-folder = "/output/gps";
    interval-secs = 10;
  };


  # disabled for now, with current config
  # RTC resets after shutdown
  services.i2c-rtc-start = {
    enable = false;
    i2c-bus = 1;
  };

  services.shutdown-button = {
    enable = true;
    gpio-pin = 21; # option not implemented yet
    shutdown-press-secs = 1; # option not implemented yet
  };
}
