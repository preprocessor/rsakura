{
  description = "rsakura - an animated cherry blossom terminal screensaver";

  inputs.nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";

  outputs =
    {
      nixpkgs,
      self,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      forEachPkgs =
        f:
        lib.genAttrs [
          "aarch64-darwin"
          "aarch64-linux"
          "x86_64-darwin"
          "x86_64-linux"
        ] (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forEachPkgs (pkgs: {
        default = pkgs.callPackage ./package.nix { };
      });

      overlays.default = final: prev: {
        nsakura = self.packages.default;
      };

      devShells = forEachPkgs (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            openssl
            pkg-config
            cargo
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer
          ];

          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${pkgs.rustc}/lib/rustlib/src/rust/library";
          };
        };
      });
    };
}
