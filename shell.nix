{ }:

let
  rustOverlay = import (builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/f600ea449c7b5bb596fa1cf21c871cc5b9e31316.tar.gz";
    sha256 = "0x70l5b4v5zljnwkzbv7yi5ld04vb75zb6wk620sfm6vd406166c";
  });

  nixpkgs = import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/a07d4ce6bee67d7c838a8a5796e75dff9caa21ef.tar.gz";
    sha256 = "0f6zni3jn6ji5icwbidbpmcgxdal2qnjszp7ragdcy0857hvq3c5";
  }) {
    overlays = [ rustOverlay ];
  };

  rustNightly = nixpkgs.rust-bin.nightly."2026-02-26".minimal.override {
    targets = [
      "thumbv8m.main-none-eabihf"  
    ];
    extensions = [ "rust-src" "clippy" "rustfmt" "llvm-tools-preview"];
  };

  rustSrc = nixpkgs.rust-bin.nightly."2026-02-26".rust-src;
in
with nixpkgs;
mkShell {
  buildInputs = [
    rustNightly
    cargo-binutils
    pkgsCross.arm-embedded.buildPackages.binutils
    llvmPackages.clang
    llvmPackages.llvm
    python3
    uv
    protobuf
    pyright
    pkg-config
    openssl
    zlib
    libffi
    libusb1
    libjpeg
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath [
    libffi
    libjpeg
    libusb1
    libressl
  ];
  DYLD_LIBRARY_PATH = "${libffi}/lib:${libjpeg.out}/lib:${libusb1}/lib:${libressl.out}/lib";
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  RUST_SRC_PATH = "${rustSrc}/lib/rustlib/src/rust/library";
}