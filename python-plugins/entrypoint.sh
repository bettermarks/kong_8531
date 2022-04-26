#!/usr/bin/env bash
set -ex

case $1 in
	"cleanup")
		# cleanup all plugins
		rm -rf /python-plugins/*
		;;
	"bash")
		bash
		;;
	"")
		# build all plugins
		for mod_path in $(find -L -maxdepth 2 -name "setup.py"); do
			mod_dir=$(dirname ${mod_path})
			mod=$(basename ${mod_dir});

			echo ${mod}...
			(
				cd ${mod_dir};
				shiv . -c pythonplugin -o /python-plugins/${mod}
			)
		done
		;;
esac

