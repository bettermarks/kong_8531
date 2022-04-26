#!/usr/bin/env bash
set -ex

case $1 in
	"cleanup")
		# cleanup all plugins
		rm -rf /rust-plugins/*
		;;
	"bash")
		bash
		;;
	"")
		# build all plugins
		for mod_path in $(find -L -maxdepth 2 -name "Cargo.toml"); do
			mod_dir=$(dirname ${mod_path})
			mod=$(basename ${mod_dir});

			


			echo ${mod}...
			(
				export CARGO_TARGET_DIR=/rust-targets/${mod}
				cd ${mod_dir};
				cargo build --release
				cp ${CARGO_TARGET_DIR}/release/${mod} /rust-plugins/
			)
		done
		;;
esac

