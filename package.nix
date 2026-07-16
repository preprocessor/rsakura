{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "nsakura";
  version = "1.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "an animated cherry blossom terminal screensaver";
    homepage = "https://github.com/preprocessor/nsakura";
    license = lib.licenses.mpl20;
    maintainers = with lib.maintainers; [ wyspr ];
    mainProgram = "nsakura";
  };
})
