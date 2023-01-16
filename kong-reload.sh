#!/bin/sh

# This goes inside the container

# Switch the symlink

if readlink /etc/kong/kong.conf | grep -q blue; then
    echo ln -s /etc/kong/kong.conf.green /etc/kong/kong.conf
    ln -s -f /etc/kong/kong.conf.green /etc/kong/kong.conf
else
    echo ln -s /etc/kong/kong.conf.blue /etc/kong/kong.conf
    ln -s -f /etc/kong/kong.conf.blue /etc/kong/kong.conf
fi

# If we don't do this, kong ignores us :(
# https://discuss.konghq.com/t/kong-reload-usage/6275
kong prepare
# Reload the new config
kong reload
