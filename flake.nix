{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

	buildInputs = with pkgs; [
	  
	];

	nativeBuildInputs = with pkgs; [
          pkg-config
	] ++ buildInputs;
      in {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
	  buildInputs = buildInputs;
	  nativeBuildInputs = nativeBuildInputs;
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
	  buildInputs = buildInputs;
          nativeBuildInputs = nativeBuildInputs;
        };
      }
    );
}
