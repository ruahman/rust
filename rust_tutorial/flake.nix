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
            rust-analyzer 
            rustfmt
            clippy
            vscode-extensions.vadimcn.vscode-lldb
          ]; # whatever you need
          
        shell = pkgs.bashInteractive;
        shellHook = ''
            if [ ! -L  .vadimcn.vscode-lldb ]; then
              ln -sf ${pkgs.vscode-extensions.vadimcn.vscode-lldb}/share/vscode/extensions/vadimcn.vscode-lldb .vadimcn.vscode-lldb
            fi
          '';
      };
    };
}
