{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs =
    { nixpkgs, self, ... }:
    let
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs (nixpkgs.lib.systems.flakeExposed) (
          system:
          f {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
          }
        );

      derivation =
        {
          lib,
          rustPlatform,
        }:

        rustPlatform.buildRustPackage {
          pname = "cloudflare-speed-cli";
          version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = {
            description = "CLI for internet speed test via cloudflare";
            homepage = "https://github.com/kavehtehrani/cloudflare-speed-cli";
            license = lib.licenses.gpl3Only;
            sourceProvenance = with lib.sourceTypes; [ fromSource ];
            meta.platforms = lib.platforms.all;
            # maintainers = with lib.maintainers; [ ];
            mainProgram = "cloudflare-speed-cli";
          };
        };
    in
    {
      packages = forAllSystems (
        { pkgs, ... }:
        {
          default = pkgs.callPackage derivation { };
        }
      );

      overlays.default = final: prev: {
        cloudflare-speed-cli = self.packages.${prev.stdenv.hostPlatform.system}.default;
      };

      devShell = forAllSystems (
        { pkgs, ... }:
        pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
          ];
        }
      );
    };
}
