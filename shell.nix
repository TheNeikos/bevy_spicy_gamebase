with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "bevy-spicy-gamebase";

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
    (
      let
        neuronRev = "e3f3349e42dab59384a705001209e0e8741c4af4";
        neuronSrc = builtins.fetchTarball "https://github.com/srid/neuron/archive/${neuronRev}.tar.gz";
        neuronPkg = import neuronSrc;
      in
      neuronPkg.default
    )
  ];



  shellHook =
    let
      dlopen-libs = [
        vulkan-loader
        alsaLib
        udev
      ];
    in
    "export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${lib.makeLibraryPath dlopen-libs}";
}
