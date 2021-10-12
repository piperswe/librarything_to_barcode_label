{
  inputs.nixpkgs.url = github:nixos/nixpkgs;
  inputs.flake-utils.url = github:numtide/flake-utils;
  inputs.naersk.url = github:nmattia/naersk;
  inputs.flake-compat.url = github:edolstra/flake-compat;
  inputs.flake-compat.flake = false;

  outputs = { self, nixpkgs, flake-utils, naersk, flake-compat }:
  flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
      rec {
        # `nix build`
        packages.librarything_to_barcode_label = naersk-lib.buildPackage {
          pname = "librarything_to_barcode_label";
          root = ./.;
        };
        defaultPackage = packages.librarything_to_barcode_label;

        # `nix run`
        apps.librarything_to_barcode_label = flake-utils.lib.mkApp {
          drv = packages.librarything_to_barcode_label;
        };
        defaultApp = apps.librarything_to_barcode_label;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo rust-analyzer pkgconfig nixpkgs-fmt ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };

        hydraJobs = {
          inherit packages;
        };
      });
}
