{
  description = "Basic rust project flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {inherit system;};
    in
      pkgs.mkShell {
        packages = with pkgs; [
          cargo # Rust package manager
          rust-analyzer # Rust LSP
					rustfmt # Rust formattr
        ];

        shellHook = ''
          echo "Rust development!"
        '';
      };
  };
}
