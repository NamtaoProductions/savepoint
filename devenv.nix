{
  pkgs,
  lib,
  config,
  ...
}:

{
  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };

  packages = with pkgs; [
    bacon
    openssl
  ];

  scripts.watcher = {
    exec = ''
      RUSTFLAGS=-Awarnings watchexec -r --clear=reset -e rs --wrap-process=none "cargo run -q"
    '';
    packages = [ pkgs.watchexec ];
    description = "Rebuilds and runs app with supressed warnings";
  };

  #
  # C LIBRARIES
  #
  # env.LD_LIBRARY_PATH = lib.makeLibraryPath [
  #   pkgs.zlib
  # ];
}
