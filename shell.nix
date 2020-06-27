{ stdenv, fetchFromGitHub, rustPlatform, pkgconfig, openssl, libsodium }:

rustPlatform.buildRustPackage rec {
  pname = "shadowsocks-rust";
  version = "1.8.12";

  src = ./.;

  cargoSha256 = "1vip2wqbpqrwcan9aqzrd4xzm7n501zkjw1pmh6715zp445f8c5m";

  SODIUM_USE_PKG_CONFIG = 1;

  buildInputs = [ openssl libsodium ];
  nativeBuildInputs = [ pkgconfig ];

  meta = with stdenv.lib; {
    homepage = "https://github.com/shadowsocks/shadowsocks-rust";
    description = "A Rust port of shadowsocks";
    license = licenses.mit;
    maintainers = [ maintainers.marsam ];
  };
}

