# shellcheck shell=bash
# Build the PHP CLI and extension development tools without Homebrew.
# Invoked explicitly with bash by the publication workflows.
set -euo pipefail

php_series="${1:?PHP minor version is required}"
install_prefix="${2:?An absolute installation prefix is required}"
case "$php_series" in
8.2)
	php_version=8.2.33
	archive_sha256=fbdeace9b38220436a4c8fd79b900df92878151db145e641750743a283b514c1
	;;
8.3)
	php_version=8.3.33
	archive_sha256=e293ed620cec74651bb4a071317892a478aa6840fab22db45c72d77cd42f9676
	;;
8.4)
	php_version=8.4.25
	archive_sha256=dc1ad8b4109898d9db49744450403874858c23efc685b1032a50bd1e83906848
	;;
*)
	echo "Unsupported PHP series: $php_series" >&2
	exit 1
	;;
esac
[[ "$install_prefix" == /* ]] || { echo 'Installation prefix must be absolute' >&2; exit 1; }

# Digests above are from https://www.php.net/releases/index.php?json&version=8.x.
build_directory="${install_prefix}-build"
mkdir -p "$build_directory"
archive="${build_directory}/php-${php_version}.tar.xz"
curl --fail --location --retry 3 "https://www.php.net/distributions/php-${php_version}.tar.xz" -o "$archive"
printf '%s  %s\n' "$archive_sha256" "$archive" | shasum -a 256 --check
tar -xJf "$archive" -C "$build_directory"
cd "${build_directory}/php-${php_version}"
./configure --prefix="$install_prefix" --disable-all --enable-cli --disable-cgi --disable-phpdbg --without-pear
make -j "$(sysctl -n hw.logicalcpu)"
make install
"$install_prefix/bin/php" -n -r 'if (PHP_ZTS || PHP_DEBUG) { exit(1); } echo PHP_VERSION, PHP_EOL;'
"$install_prefix/bin/php-config" --version
"$install_prefix/bin/phpize" --version
if [[ -n "${GITHUB_PATH:-}" ]]; then
	printf '%s\n' "$install_prefix/bin" >> "$GITHUB_PATH"
	printf 'PHP_CONFIG=%s/bin/php-config\n' "$install_prefix" >> "$GITHUB_ENV"
fi
