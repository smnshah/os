{
  description = "Rust OS development environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, rust-overlay }:
  let systems = [ 
    "x86_64-linux" 
    "aarch64-darwin" 
  ];

  forAllSystems = f:
    nixpkgs.lib.genAttrs systems (system: f system);
  
  in
  {
    devShells = forAllSystems (system:
      let pkgs = import nixpkgs { 
	inherit system;
	overlays = [ rust-overlay.overlays.default ];
      };
      in {
        default = pkgs.mkShell {
	  packages = with pkgs; [ 
	    git
	    llvmPackages_latest.clang
	    llvmPackages_latest.llvm
	    llvmPackages_latest.lld
	    qemu
	    xorriso
	    mtools
	    autoconf
	    automake
	    nasm
	    (rust-bin.nightly.latest.default.override {
  	      targets = [ "x86_64-unknown-none" ];
 	    })
	  ];

	  shellHook = ''
	    export PATH=${pkgs.git}/bin:$PATH
	  '';
        };
      }
    );
  }; 
}
