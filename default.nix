let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "bruhmania";
	version = "0.1.0";

	src = ./.;

	nativeBuildInputs = with pkgs; [
		cargo
		pkg-config
	];

	buildInputs = with pkgs; [
		openssl
	];

	cargoSha256 = "1pws0g0hz0h2kxjinz2rgqwmw4cyr4wkj3ajgbbc3qackdlsa7pm";
}
