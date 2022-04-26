import kong_pdk.pdk.kong as kong

Schema = ({"message": {"type": "string"}},)


class Plugin:
    def __init__(self, config):
        self.config = config

    def access(self, kong: kong.kong):
        kong.service.request.set_header("x-pythonplugin", "foobar")
