with import <nixpkgs>{};

stdenv.mkDerivation {
    name = "bevy-spicy-data";

    buildInputs = [
        gcc
        pkgconfig 
        x11 
        xorg.libXcursor 
        xorg.libXrandr 
        xorg.libXi 
        vulkan-tools
        vulkan-headers 
        vulkan-loader 
        vulkan-validation-layers 
        alsaLib
        udev
        python3
    ];

    

    shellHook = let 
        dlopen-libs = [
            vulkan-loader
            alsaLib
            udev
        ];
        in "export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${lib.makeLibraryPath dlopen-libs}";
}