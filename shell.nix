# shell.nix
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    packages = with pkgs;
    [
        # Rust Dependencies
        rustup
        gcc
        openssl
        pkg-config
    ];
    
    # Environment variables
    # RUST_BACKTRACE = 1;
}
