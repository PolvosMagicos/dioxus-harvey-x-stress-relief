{ pkgs ? import <nixpkgs> {} }:

let
  gst = pkgs.gst_all_1;
in
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    gcc
    pkg-config
    openssl
    cargo-binstall

    gtk3
    webkitgtk_4_1
    libsoup_3
    xdotool

    alsa-lib

    gst.gstreamer
    gst.gst-plugins-base
    gst.gst-plugins-good
    gst.gst-plugins-bad
    gst.gst-plugins-ugly
    gst.gst-libav
  ];

  shellHook = ''
    export PATH="$HOME/.cargo/bin:$PATH"

    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.alsa-lib.dev}/lib/pkgconfig:${pkgs.webkitgtk_4_1.dev}/lib/pkgconfig:${pkgs.gtk3.dev}/lib/pkgconfig:${pkgs.libsoup_3.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"

    export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"

    export LIBRARY_PATH="${pkgs.xdotool}/lib:${pkgs.alsa-lib}/lib:$LIBRARY_PATH"

    export LD_LIBRARY_PATH="${pkgs.xdotool}/lib:${pkgs.alsa-lib}/lib:${pkgs.webkitgtk_4_1}/lib:${pkgs.gtk3}/lib:${pkgs.libsoup_3}/lib:$LD_LIBRARY_PATH"

    export GST_PLUGIN_SYSTEM_PATH_1_0="${gst.gstreamer.out}/lib/gstreamer-1.0:${gst.gst-plugins-base}/lib/gstreamer-1.0:${gst.gst-plugins-good}/lib/gstreamer-1.0:${gst.gst-plugins-bad}/lib/gstreamer-1.0:${gst.gst-plugins-ugly}/lib/gstreamer-1.0:${gst.gst-libav}/lib/gstreamer-1.0"
  '';
}
