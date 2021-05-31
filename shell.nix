{ pkgs ? import <unstable> {} }:
pkgs.mkShell {
	nativeBuildInputs = with pkgs; [
		cargo
		pkg-config
	];

	buildInputs = with pkgs; [
		openssl
	];
}
