# Edit this configuration file to define what should be installed on
# your system.  Help is available in the configuration.nix(5) man page
# and in the NixOS manual (accessible by running ‘nixos-help’).

{ config, pkgs, ... }:

{
  imports =
    [ # Include the results of the hardware scan.
      ./hardware-configuration.nix
    ];

  nix = {
    package = pkgs.nixUnstable;
    extraOptions = ''
      experimental-features = nix-command flakes
    '';
  };

  # Use the GRUB 2 boot loader.
  boot.loader.grub.enable = true;
  boot.loader.grub.version = 2;

  # Define on which hard drive you want to install Grub.
  boot.loader.grub.device = "/dev/sda";

  networking.hostName = "reineke";

  # Set your time zone.
  time.timeZone = "UTC";

  # The global useDHCP flag is deprecated, therefore explicitly set to false here.
  # Per-interface useDHCP will be mandatory in the future, so this generated config
  # replicates the default behaviour.
  networking.useDHCP = false;
  networking.interfaces.ens3.useDHCP = true;

  # Configure IPv6. Hetzner doesn't seem to provide DHCP for that.
  networking.interfaces.ens3.ipv6.addresses = [ {
    address = "2a01:4f8:1c1c:3385::2";
    prefixLength = 64;
  } ];
  networking.defaultGateway6 = {
    address = "fe80::1";
    interface = "ens3";
  };

  # Select internationalisation properties.
  # i18n.defaultLocale = "en_US.UTF-8";
  # console = {
  #   font = "Lat2-Terminus16";
  #   keyMap = "us";
  # };

  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQCpUT/y8rl08ZxWUVIAj2LHh1dapmG8S6DqZLDaqKlBEhEj3w/DC3991+NA/3I8O9ITvwGeox3EC/WMNb0NYq1jgLACvIc+ig14b69U86HbVMcTJqyCkc0Bf/zgbEnH+HxzKsPGFBLjlISHIInwwquoxDCa3sR8LVuhCUc2YYiRcgIbXxUcwxlMrSrJuKmsfMDBdGACTK4AvgR7q7SXVjypCvU+joPmX9d8IKZRg59AQkWnZAdulNPF/xk53wSZlkNynh6JhWjU28x/1XUSkK+JHVKUoaQgRFmf9OdqmT7YCi9KfP6/ipAJcB41N1/zDwahIy6sGxtx+TjEPGKsGu2RJMKdjwXioMcQNgoHhuQhJZTiimnJJz5Y6DzUdgNsZkRHFmoinbZ71TFGGppLijMC173sOioMSToNuyHEJKu91bDDxJfaZE9DQCh4nGxEJYNxUwlO2YIMzFVHWyYQ5IpSi8CfWmnWIJTLltVQOzPnFMt5N1ZCIN/O0NYMHzPmjE8= root@reineke.nodes.braun-odw.eu"
  ];

  # Define a user account. Don't forget to set a password with ‘passwd’.
  # users.users.jane = {
  #   isNormalUser = true;
  #   extraGroups = [ "wheel" ]; # Enable ‘sudo’ for the user.
  # };

  # List packages installed in system profile. To search, run:
  # $ nix search wget
  environment.systemPackages = with pkgs; [
    git
    vim
  ];

  # Some programs need SUID wrappers, can be configured further or are
  # started in user sessions.
  # programs.mtr.enable = true;
  # programs.gnupg.agent = {
  #   enable = true;
  #   enableSSHSupport = true;
  # };

  # List services that you want to enable:

  # Enable the OpenSSH daemon.
  services.openssh.enable = true;
  services.openssh.permitRootLogin = "yes";

  # This value determines the NixOS release from which the default
  # settings for stateful data, like file locations and database versions
  # on your system were taken. It‘s perfectly fine and recommended to leave
  # this value at the release version of the first install of this system.
  # Before changing this value read the documentation for this option
  # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
  system.stateVersion = "21.05"; # Did you read the comment?

}

