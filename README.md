# rsakura

`rsakura` is a rust port of [`nsakura`](https://github.com/KornelHajto/nsakura)

> Terminal cherry blossom screensaver.
> It draws a static ASCII tree and animates falling leaves with a lightweight physics loop.


## Installation
### Cargo

`cargo install --git https://github.com/preprocessor/rsakura`

### Nix

Note: Flakes must be enabled.

#### Nix shell
`nix run github:preprocessor/rsakura`
#### Nix flake input

`flake.nix`
```nix
{
  inputs.rsakura.url = "github:preprocessor/rsakura";

  outputs = {nixpkgs, ...} @ inputs: {
    nixosConfigurations.HOSTNAME = nixpkgs.lib.nixosSystem {
      specialArgs = { inherit inputs; }; # this is the important part
      modules = [
        ./configuration.nix
      ];
    };
  };
}
```
`configuration.nix`
```nix
{inputs, pkgs, ...}: {
     # Use the overlay
    nixpkgs.overlays = [ rsakura.overlays.default ];

    # Add the package to your systemPackages
    environment.systemPackages = [
        pkgs.rsakura
    ];
}
```


## Usage

Optional flags:
- `--speed=<float>`: scales fall speed (default 1.0)
- `--sway=<float>`: attached leaf sway amplitude (0.0 to 1.0, default 0.0)
- `--art=<path>`: use a custom art file instead of the built-in tree
- `--art-color=COLOR`: use a custom color (hex or rgb) for the art
- `--leaf-color=COLOR`: use a custom color (hex or rgb) for the leaves

Keys:
- `Q` or `Esc`: quit


