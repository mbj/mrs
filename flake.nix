{
  description = "mrs development environment (ociman Apple integration)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/7a1a64774a5fd0b0cd39ac95d0e170ace8b266a0";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.aarch64-darwin;
    in
    {
      devShells.aarch64-darwin.default = pkgs.mkShell {
        packages = with pkgs; [
          rustup
          cargo-nextest
        ];

        RUST_BACKTRACE = "full";
      };
    };
}
