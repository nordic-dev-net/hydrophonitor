{
  config,
  pkgs,
  ...
}: let
  system = "aarch64-linux";
in {
  imports = [
    ./i2c.nix
  ];
  system.stateVersion = "23.05";
  sdImage.compressImage = false;

  boot = {
    # disable internal sound card and vc4 gpu
    # to set USB sound card as default
    blacklistedKernelModules = ["snd_bcm2835" "vc4"];
    # enable i2c and rtc modules
    kernelModules = ["i2c-dev" "i2c_bcm2708" "rtc_ds1307"];
    kernelPackages = pkgs.linuxKernel.packages.linux_rpi3;
    initrd.availableKernelModules = [
      "usbhid"
      "usb_storage"
      "pcie_brcmstb" # required for the pcie bus to work
      "reset-raspberrypi" # required for vl805 firmware to load
    ];
    loader = {
      grub.enable = false;
      generic-extlinux-compatible.enable = true;
    };
  };

  hardware = {
    raspberry-pi."3".i2c1.enable = true;
    deviceTree.filter = "bcm2711-rpi-*.dtb";
    # Required for the wireless firmware
    enableRedistributableFirmware = true;
  };

  environment.systemPackages = with pkgs; [i2c-tools libgpiod];

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
    output-folder = "audio";
    sample-rate = 192000;
    sample-format = "S32_LE";
    channels = 4;
    max-file-time-secs = 300;
  };

  services.gps-recorder = {
    enable = true;
    output-folder = "gps";
    interval-secs = 10;
  };

  services.i2c-rtc-start = {
    enable = true;
    i2c-bus = 1;
  };
}
