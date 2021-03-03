# Exercise 1

## Setup

Change into the `helloworld` directory in this folder. For example, from the project root:

```sh
cd ex1/helloworld
```

### A fresh development environment

Create a new Python3 [virtual environment](https://docs.python.org/3/tutorial/venv.html), activate it, and install some tooling.

On Linux/macOS:

```sh
python3 -m venv venv
source venv/bin/activate
python3 -m pip install --upgrade pip setuptools
```

On Windows, from PowerShell:

```ps
python3 -m venv venv
venv\Scripts\activate
python3 -m pip install --upgrade pip setuptools
```

### An installed app

Install helloworld in your new Python virtual environment - using pip in editable/development mode.

```sh
pip3 install -e .
```

## Test run

Make sure this simple application works by giving it a run!

On Linux/macOS:

```sh
export FLASK_APP=helloworld.app
export FLASK_ENV=development
flask run
```

On Windows, from PowerShell:

```ps
$env:FLASK_APP=helloworld.app
$env:FLASK_ENV=development
flask run
```

In another window/terminal, you can use curl or just python to HTTP GET port 5000.

curl, if you have it:

```sh
curl localhost:5000
```

Pure Python, if you don't:

```sh
python -c 'from urllib.request import urlopen; print(urlopen("http://localhost:5000").read().decode().strip())'
```

## Cleanup

You should use `Ctrl+C` to terminate the simple flask webserver. Then (optionally) remove our local development artifacts.

Deactivating the Python virtual environment:

```sh
deactivate
```

### Optional removal of development artifacts

Remove the virtual environment (because we won't need it any more):

On Linux/macOS:

```sh
rm -rf venv
```

On Windows, with PowerShell:

```ps
Remove-Item -Recurse venv
```

## The point

1. I had to give separate instructions for which platforms you were on, and in some cases different tooling that might be present.
1. Some of you probably struggled with compatibility of this or that, the version of python, etc, even with those instructions.
1. This could have been a better experience for you, even if we got it to work in the end.
1. Would you ship this code to someone and tell them "Yeah, it just works - you should be good to go."? No, you would need to document the environment expectations and capabilities on supported platforms.
1. This is just for Hello, World. This is why containers exist.
