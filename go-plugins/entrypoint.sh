#!/usr/bin/env bash
set -ex

case $1 in
	"cleanup")
		# cleanup all plugins
		rm /go-plugins/*
		;;
	"bash")
		bash
		;;
	"")
		# build all plugins
		for mod_path in $(find -L -maxdepth 2 -name "go.mod"); do
			mod_dir=$(dirname ${mod_path})
			mod=$(basename ${mod_dir});

			echo ${mod}...
			(
				cd ${mod_dir};
				go get -v
				# go test -v -race
				go build -o /go-plugins/
			)
		done
		;;
esac

