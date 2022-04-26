import setuptools

setuptools.setup(
    name="pythonplugin",
    version="0.1.0",
    author="Oliver Berger",
    author_email="oliver.berger@bettermarks.com",
    description="Python Kong PDK plugin.",
    zip_safe=False,
    include_package_data=True,
    package_dir={"": "src"},
    # https://setuptools.readthedocs.io/en/latest/setuptools.html#find-namespace-packages
    packages=setuptools.find_namespace_packages(where="src"),
    entry_points={"console_scripts": ["pythonplugin = pythonplugin.cli:main"]},
    # we locked use req txt for final dist
    install_requires=["kong-pdk"],
)
