# Exercise 1 - Why do containers exist?

## Prerequisites

For this to work, you'll need Python 3.6 or newer and `git`. Some other maybe helpful things are `curl`, and a little bit of knowledge about Python could help.

## Setup

Clone this project to your local workstation, no matter what the OS is:

```sh
git clone https://github.com/jharmison-redhat/containers-for-embedded-ws
```

Change into the `helloworld` directory in this folder, beneath the project root.

```sh
cd containers-for-embedded-ws/ex1/helloworld
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

### A quick peek at our helloworld app

You should open or `cat` the file located at `src/helloworld/app.py`. It's a pretty simple "Hello, World!" example using the Flask web framework in Python. You shouldn't have to know python to be able to puzzle out what it does. Flask, as a web framework, enables us to use Python functions as callables for path-based requests from a web client and we've defined for the `/` route (meaning the bare address of the web server) to call a function that returns a simple string that includes our system's hostname.

### An installed app

Install helloworld in your new Python virtual environment - using pip in editable/development mode.

```sh
pip3 install -e .
```

## Test run

Make sure this simple application works by giving it a run!

On Linux/macOS:

```sh
export FLASK_APP=helloworld.app  # This tells Flask which app to run, we're using the one we installed
flask run  # This starts up a very simple webserver that routes requests according to our app definition
```

On Windows, from PowerShell:

```ps
$env:FLASK_APP=helloworld.app
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

You should use `Ctrl+C` to terminate the simple Flask webserver. Then deactivate the virtual envrionment and (optionally) remove our local development artifacts.

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

#### To REALLY reset everything

This will reset everything in this folder to exactly match the contents of the git HEAD:

```sh
git clean -dxf
```

## The point

1. I had to give separate instructions for which platforms you were on, and in some cases different tooling that might be present.
1. Some of you probably struggled with compatibility of this or that, the version of python, etc, even with those instructions.
1. Would you ship this code to someone and tell them "Yeah, it just works - you should be good to go."? No, you would need to document the environment expectations and capabilities on supported platforms.
1. This is just for Hello, World. This is why containers exist.
