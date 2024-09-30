{
  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override {
          extensions = [ "rust-analysis" ];
        };
      in
      {
        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            pkg-config
            binutils
            gcc
            rust-analyzer
            # using a hardcoded rustfmt version to support nightly rustfmt features.
            rust-bin.nightly."2024-09-25".rustfmt
            rust-toolchain

          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
            # NOTE: Deps copied from a Bevy project, not sure if they're all needed for Iced.
            alsaLib
            udev
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            vulkan-headers
            vulkan-loader
            vulkan-tools
            vulkan-validation-layers

          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.Foundation
            darwin.apple_sdk.frameworks.Cocoa
            darwin.apple_sdk.frameworks.Carbon
            darwin.apple_sdk.frameworks.WebKit
          ];

          # necessary for some libs. Not even sure which ones,
          # i just copy and pasted from a Bevy repo of mine, hah.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
