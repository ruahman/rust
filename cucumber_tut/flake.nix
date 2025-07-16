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
            bashInteractive
            bash-completion
            rustc 
            cargo 
            cargo-watch 
            cargo-nextest
            bacon
            rust-analyzer 
            rustfmt
            clippy
            vscode-extensions.vadimcn.vscode-lldb
          ]; # whatever you need
          
        shell = pkgs.bashInteractive;
        shellHook = ''
          export CODELLDB=${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb
        '';
      };
    };
}
