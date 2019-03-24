with import <nixpkgs> {};

let src = fetchFromGitHub {
  owner = "mozilla";
  repo = "nixpkgs-mozilla";
  rev = "cebceca52d54c3df371c2265903f008c7a72980b";
  sha256 = "1vfib46ahbcnff0b7dmdmbpvc5yb0l3kl49y4h9191j4nix8z7af";
};
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
stdenv.mkDerivation rec {
  name = "mxhkd-env";
  version = "0.0.1";

  nativeBuildInputs = [
    latest.rustChannels.nightly.rust
    pkgconfig
  ];

  RUST_BACKTRACE = 1;

  buildInputs = [
    python3
    xorg.libxcb
    xorg.libX11
    xorg.xmodmap
  ];
}
