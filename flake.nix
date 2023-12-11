{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageOverrides = pkgs: pkgs.rustBuilder.overrides.all ++ [
            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "system-deps";
              overrideAttrs = drv: {
                propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [
                  pkgs.pkg-config
                ];
                shellHook = drv.shellHook or "" + ''
                  export PGK_CONFIG_ALLOW_CROSS=1
                '';
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "gobject-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.gobject-introspection
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "cairo-sys-rs";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.cairo
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "gio-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.glib
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "gdk-pixbuf-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.gdk-pixbuf
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "gdk4-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.gtk4
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "gsk4-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.gtk4
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "gtk4-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.gtk4
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "pango-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.pango
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "graphene-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.gobject-introspection
                  pkgs.graphene
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "glib-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.glib
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "libadwaita-sys";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.libadwaita
                ];
              };
            })

            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "dust-mail-gtk";
              overrideAttrs = drv: {
                buildInputs = drv.buildInputs or [ ] ++ [
                  pkgs.glib
                ];
              };
            })

          ];

          rustVersion = "1.70.0";
          extraRustComponents = [ "rustfmt" ];
          packageFun = import ./Cargo.nix;
        };

        workspaceShell = rustPkgs.workspaceShell {
          buildInputs = with pkgs; [ pkg-config ];
        };

      in
      rec {
        devShell = workspaceShell;

        packages = {
          dust-mail-gtk = (rustPkgs.workspace.dust-mail-gtk { });
          default = packages.dust-mail-gtk;
        };
      }
    );
}
