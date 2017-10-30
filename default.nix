with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "tri-0.1.0";
  buildInputs = [ openssl pkgconfig sqlite ];
}
