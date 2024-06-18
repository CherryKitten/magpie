{
  description = "Magpie Music Server";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ { self, flake-parts, nixpkgs, naersk, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {

      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = { config, pkgs, system, ... }: {
        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo rustfmt rustPackages.clippy cargo-watch cargo-mommy diesel-cli ];
          shellHook = "exec $SHELL";
        };

        packages =
          let
            naersk' = pkgs.callPackage naersk { };
          in
          {
            magpie = naersk'.buildPackage {
              src = ./.;
            };

            default = self.packages.${system}.magpie;
          };
      };

      flake = { };
    };
}
