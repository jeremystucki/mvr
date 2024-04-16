{
  description = "A replacement for zsh's zmv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/21.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        rustPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "mvr";
          version = "0.1.0";
          src = pkgs.lib.cleanSource ./.;
          cargoSha256 = "lTLaJcnM5bgwfOnqwKqdTJUdtFq6YjB5/Lu5ECdLMuI=";
        };
      in
      {
        packages.default = rustPackage;
        defaultPackage = rustPackage;
        devShell = pkgs.mkShell {
          buildInputs = [ pkgs.rustc pkgs.cargo ];
        };
      }
    );
}

