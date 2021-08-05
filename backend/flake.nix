{
  description = "made-by.braun-odw.eu custom backend";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # We can't use `crate2nix` from nixpkgs, as that only contains a binary, not
    # the Nix file we're importing below.
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
  };

  outputs = {self, nixpkgs, crate2nix}:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        system = system;
      };
      crate = import "${crate2nix}/tools.nix" { pkgs = pkgs; };
      cargoPackage = import
        (crate.generatedCargoNix { name = "backend"; src = ./.; })
        { pkgs = pkgs; };
    in
    {
      defaultPackage.${system} = cargoPackage.rootCrate.build;
    };
}
