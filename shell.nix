let
  sources = import ./npins;
   pkgs = import sources.nixpkgs {};
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rust-analyzer
    pkg-config
    libxcb
    libxcursor
    libx11
    libGL
    egl-wayland
    wayland
    freetype
    fontconfig
    alsa-lib
    jack2
  ];
}
