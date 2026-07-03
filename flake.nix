{
  description = "NixCI demo for the ociman crate";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

  };

  outputs = { nixpkgs, rust-overlay, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain
          (pkgs.rust-bin.stable."1.93.0".default.override {
            extensions = [ "clippy" "rustfmt" ];
          });

        src = ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "workspace";
          version = "0.0.0";
        });

        buildPackage = args: craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        } // args);

        ociman = buildPackage {
          pname = "ociman";
          version = "0.3.1";
          cargoExtraArgs = "-p ociman";
          cargoTestExtraArgs = "--lib -- --skip backend::tests";
        };

        stratosphere = buildPackage {
          pname = "stratosphere";
          version = "0.0.5";
          cargoExtraArgs = "-p stratosphere";
        };

        stratosphere-core = buildPackage {
          pname = "stratosphere-core";
          version = "0.0.5";
          cargoExtraArgs = "-p stratosphere-core";
        };

        stratosphere-generator = buildPackage {
          pname = "stratosphere-generator";
          version = "0.0.5";
          cargoExtraArgs = "-p stratosphere-generator";
        };

        # Turn a derivation into a fixed-output derivation so the Nix
        # sandbox grants network access. The output is a deterministic
        # magic string whose hash we pre-compute; the real signal is
        # whether the build succeeds or fails.
        makePureImpure = drv:
          let
            magicString = builtins.unsafeDiscardStringContext
              (builtins.substring 0 12 (baseNameOf drv.drvPath));
            outputHash = builtins.hashString "sha256" magicString;
          in
          drv.overrideAttrs (old: {
            outputHashAlgo = "sha256";
            outputHashMode = "flat";
            inherit outputHash;
            buildCommand = ''
              ${old.buildCommand or ""}
              rm -rf $out
              echo -n "${magicString}" > $out
            '';
          });
      in
      {
        packages = {
          inherit ociman stratosphere stratosphere-core stratosphere-generator;
          default = ociman;
        };

        checks = {
          inherit ociman stratosphere stratosphere-core stratosphere-generator;

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            pname = "workspace";
            version = "0.0.0";
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          });

          fmt = craneLib.cargoFmt {
            inherit src;
            pname = "workspace";
            version = "0.0.0";
          };

          ociman-integration-test =
            let
              ocimanTestArchive = craneLib.mkCargoDerivation (commonArgs // {
                inherit cargoArtifacts;
                pname = "ociman";
                version = "0.3.1";
                nativeBuildInputs = [ pkgs.cargo-nextest ];
                buildPhaseCargoCommand = ''
                  cargo nextest archive \
                    --all-features \
                    --release \
                    --package ociman \
                    --archive-file archive.tar.zst
                '';
                installPhaseCommand = ''
                  mkdir -p $out
                  cp archive.tar.zst $out/
                '';
              });

              integrationTest = pkgs.testers.runNixOSTest {
                name = "ociman-integration-test";
                nodes.machine = { pkgs, lib, ... }: {
                  virtualisation.podman.enable = true;
                  virtualisation.memorySize = 4096;
                  virtualisation.cores = 2;
                  virtualisation.diskSize = 8192;
                  networking.useDHCP = lib.mkForce true;
                  environment.systemPackages = [ pkgs.cargo-nextest ];
                };
                testScript = ''
                  machine.wait_for_unit("default.target")
                  machine.wait_for_unit("dhcpcd.service")
                  machine.succeed(
                    "cp -r ${src}/. /tmp/source"
                    " && chmod -R u+w /tmp/source"
                    " && OCIMAN_BACKEND=podman"
                    " cargo-nextest nextest run"
                    " --archive-file ${ocimanTestArchive}/archive.tar.zst"
                    " --workspace-remap /tmp/source"
                    " --test-threads=1"
                  )
                '';
              };
            in
            makePureImpure (pkgs.stdenv.mkDerivation {
              name = "ociman-integration-test";
              requiredSystemFeatures = [ "nixos-test" "kvm" ];
              buildCommand = ''
                mkdir -p $out
                export LOGFILE=/dev/null
                ${integrationTest.driver}/bin/nixos-test-driver -o $out
              '';
            });
        };

        devShells.default = craneLib.devShell {
          checks = {
            inherit ociman;
          };
          packages = [
            pkgs.cargo-nextest
          ];
        };
      }
    );
}
