# Exercise 3 - Containers for generalized workloads

## Prerequisites

For this to work, you'll need a container runtime (or the ability to install one), and to have already git cloned this repository for the previous exercises.

These examples assume you are in this directory subordinate to the root of the project, meaning your are in a folder named `ex3` inside `containers-for-embedded-ws`.

## Setup

We just need to make our target directory for our compilation.

```sh
mkdir src/dist
```

## Basic exploration

Unlike our previous exercise, we're not going to be using a Dockerfile to build this program. We're going to use one that's already published. If you'd like to take a look at the Dockerfile (it's quite complex!), it's publicly available [here](https://github.com/multiarch/crossbuild/blob/master/Dockerfile).

This image is published to the world's largest public container registry, [Docker Hub](https://hub.docker.com/r/multiarch/crossbuild).

The first program we're going to compile is a pretty simple Hello World binary on MIPS. The documentation on that image tells you how to use it, and you don't need to do anything to download the image first. You should be able to simply run it, and your runtime will download it for you before executing it:

```sh
docker run --rm -v "$(pwd)/src:/workdir" -e CROSS_TRIPLE=mipsel-linux-gnu docker.io/multiarch/crossbuild:latest gcc -static helloworld.c -o dist/helloworld-mipsel
```

Note that in my case, because I'm running on an SELinux enabled system and trying to pass a directory into the container (which runs with a different SELinux label), I get the following error message:

```console
$ docker run --rm -v "$(pwd)/src:/workdir" -e CROSS_TRIPLE=mipsel-linux-gnu docker.io/multiarch/crossbuild:latest gcc -static helloworld.c -o dist/helloworld-mipsel
gcc: error: helloworld.c: Permission denied
gcc: fatal error: no input files
compilation terminated.
```

That's easy enough to fix by appending a `:Z` to the end of the volume specification, causing `podman` (which is pretending to be Docker) to relabel the directory on the fly for mounting. Now, running with `-v "$(pwd)/src:/workdir:Z`," I got.... well, no output. But a file named `helloworld-mipsel` should show up in your `src/dist` directory!

```console
$ ls -halF src/dist/helloworld-mipsel
-rwxr-xr-x. 1 james james 680K Mar  4 19:36 src/dist/helloworld-mipsel*
$ file src/dist/helloworld-mipsel
src/dist/helloworld-mipsel: ELF 32-bit LSB executable, MIPS, MIPS-II version 1 (SYSV), statically linked, for GNU/Linux 2.6.32, BuildID[sha1]=61e22dd95e3c7213568118fea3eb5fd1267cf6cb, not stripped
```

This is a pretty cool thing happening - I didn't compile my application inside a container image, I simply used a container image as a build environment to let me compile it right here. This happens to be compiled using a crossbuild toolchain to enable me to create a Linux binary for an alternative architecture - but importantly, we should be able to _run_ this application inside a container as well.

Let's build a container image for running our MIPS binary:

```sh
docker build . -f Dockerfile.mips -t mips-runner
```

Something you may have picked up on - as we've been using a common base for all of the images we've built today, you haven't had to pull that base image since the first time you inspected it or built an image with it. Your runtime sees that the image is already pulled and simply reuses it. If you wanted to update that base image, you could explicitly pull it again and rebuild any images that rely on it. Or, as shorthand, you can run `docker build --pull . -t mips-runner` instead to force your runtime to pull any images specified in `FROM` instructions.

Let's use our mips-runner to run `helloworld`:

```console
$ docker run --rm -v "$(pwd)/src:/project" mips-runner dist/helloworld-mipsel
Hello, World!
```

Note that again, I had to use `:Z` after my volume specification personally - those of you on SELinux-enabled systems may have to do the same.

Okay, now, consider the case where we may be targeting multiple generations of some system that are on slightly different architectures. To build `helloworld` for both ARM and AARCH64, run the following:

```sh
docker run --rm -v "$(pwd)/src:/workdir" -e CROSS_TRIPLE=arm-linux-gnueabi docker.io/multiarch/crossbuild:latest gcc -static helloworld.c -o dist/helloworld-arm
docker run --rm -v "$(pwd)/src:/workdir" -e CROSS_TRIPLE=aarch64-linux-gnu docker.io/multiarch/crossbuild:latest gcc -static helloworld.c -o dist/helloworld-aarch64
```

Double check all your binaries to make sure they're the file types you expect them to be:

```console
$ file src/dist/*
src/dist/helloworld-aarch64: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV), statically linked, for GNU/Linux 3.7.0, BuildID[sha1]=5f501838d598779b3ce6bfabf9e7ad181f1959fc, not stripped
src/dist/helloworld-arm:     ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, for GNU/Linux 2.6.32, BuildID[sha1]=0aa474a84eb51a54c0848dcc2878e391bf97653d, not stripped
src/dist/helloworld-mipsel:  ELF 32-bit LSB executable, MIPS, MIPS-II version 1 (SYSV), statically linked, for GNU/Linux 2.6.32, BuildID[sha1]=61e22dd95e3c7213568118fea3eb5fd1267cf6cb, not stripped
```


Now, to build runners for both of those:

```sh
docker build . -f Dockerfile.arm -t arm-runner
docker build . -f Dockerfile.aarch64 -t aarch64-runner
```

And finally, to run them both to make sure they're working like we expect:

```sh
docker run --rm -v "$(pwd)/src:/project" arm-runner dist/helloworld-arm
docker run --rm -v "$(pwd)/src:/project" aarch64-runner dist/helloworld-aarch64
```

## The point

You're able to cross-compile applications for multiple architectures typical of embedded systems. If you already have an x86_64 emulator, or Linux available on some alternate architecture, you can use containers to bring you the same benefits you saw demonstrated for traditional applications.
