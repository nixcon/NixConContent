{ pkgs, config, ... }: {
  programs.fish.enable = true;

  fileSystems."/" = {
    device = "/dev/sda1";
  };
  boot.loader.grub.enable = false;
  system.stateVersion = "23.05";
}
