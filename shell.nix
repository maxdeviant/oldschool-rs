with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "oldschool";

  buildInputs = [
    stdenv
    pkg-config
  ];
}
