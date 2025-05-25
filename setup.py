from setuptools import setup, find_packages

setup(
    name="cli-corrector",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[
        "prompt_toolkit",
    ],
    entry_points={
        "console_scripts": [
            "cli-corrector = cli_corrector.cli_corrector:main"
        ]
    },
    author="Salem",
    description="A CLI tool that suggests and auto-corrects mistyped terminal commands.",
    classifiers=[
        "Programming Language :: Python :: 3",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.6",
)
