# Exercise 2

## Prerequisites

For this to work, you'll need a container runtime (or the ability to install one), and to have already git cloned this repository for the previous exercise.

## Setup

### Installing a container runtime

I'm going to be using `podman` for my walkthroughs, but you're able to use `docker`. I am also using the `podman-docker` package for my distribution, which lets me use commands that call the executable `docker` so we can all copy & paste the same commands, since the interfaces for those two tools are so similar.

To install `podman` and `podman-docker` on Fedora 33, you could just run:

```sh
sudo dnf -y install podman podman-docker
```

To install Docker Desktop on your workstation, you should follow [their installation instructions](https://docs.docker.com/get-docker/).

#### A quick note about Docker ubiquity

Docker is available for installation on macOS, Windows, and many Linux distributions. Docker relies on the native features of the Linux kernel to implement its isolation mechanisms as an underlying component of the architecture, so when you run Docker on your Mac or Windows machine, you're actually running a Linux VM - it's just wrapped up and hidden from you quite a bit. Running Docker on a Linux distribution, like Fedora or Debian, uses the native features of the OS on which it's installed. Because Docker encapsulates the configuration of this Linux VM for you, you may prefer using that on platforms other than Linux. Docker and Podman both conform to the Open Container Initiative specifications for images and runtimes - and Podman intentionally keeps its command line interface similar to make migration from Docker easier, changing how those kernel features are used but not how they're exposed to you.

## 
