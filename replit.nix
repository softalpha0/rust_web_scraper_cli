{ pkgs }: {
  deps = [
    pkgs.pkg-config
    pkgs.openssl
    pkgs.gcc
    pkgs.clang
    pkgs.libiconv
  ];
}