{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  } @ inputs: let
      fenix = inputs.fenix.packages;
    in
    # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    (flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        crane = inputs.crane.mkLib pkgs;
        # Toolchain
        toolchain = fenix.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
        };
        craneLib = crane.overrideToolchain toolchain;

        buildInputs = with pkgs; [
          openssl.dev
          pkg-config
          wayland
        ];

        commonArgs = {
          src = ./.;
          doCheck = false;
          inherit buildInputs;
        };

        libraries = with pkgs; [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
        # Compile all artifacts
        appDeps = craneLib.buildDepsOnly commonArgs;

        # Compile
        app = craneLib.buildPackage (commonArgs // {
          cargoArtifacts = appDeps;
        });
      in {
        # nix build
        packages.default = app;

        # nix run
        apps.default = flake-utils.lib.mkApp {
          drv = app;
        };

        # nix develop
        devShells.default = craneLib.devShell {
          inherit buildInputs;

          packages = with pkgs; [
            toolchain
            heaptrack
          ];

          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libraries}";
        };
      }
    ));
}
