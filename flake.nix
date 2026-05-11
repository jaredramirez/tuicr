{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk/master";
    naersk.inputs.nixpkgs.follows= "nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          # NB: `pkgs.jj` is a JSON stream editor; the Jujutsu VCS is `pkgs.jujutsu`.
          # Both expose a `jj` binary, so picking the wrong one causes tuicr to
          # fail at startup and silently skips the jj backend tests.
          buildInputs = [ cargo rustc rustfmt rustPackages.clippy jujutsu git ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
