from kong_pdk.cli import start_dedicated_server
from .plugin import Plugin

start_dedicated_server("pythonplugin", Plugin, "0.1.0", 0)
