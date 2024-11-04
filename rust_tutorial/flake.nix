{
  description = "Rust flake";
  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # or whatever vers
    };
  
  outputs = { self, nixpkgs, ... }@inputs:
    let
     system = "x86_64-linux"; # your version
     pkgs = nixpkgs.legacyPackages.${system};    
    in
    {
      devShells.${system}.default = pkgs.mkShell
      {
        packages = with pkgs; [ 
            zsh 
            rustc 
            cargo 
            cargo-watch 
            rust-analyzer 
            cargo-tarpaulin 
          ]; # whatever you need

        # Use zsh as the default shell
        shellHook = ''
          export SHELL=$(which zsh)
          exec $SHELL
        '';
      };
    };
}
