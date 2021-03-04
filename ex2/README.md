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

To install Docker Desktop on your workstation, you should follow [their installation instructions](https://docs.docker.com/get-docker/). You should probably follow the [optional post-installation steps](https://docs.docker.com/engine/install/linux-postinstall/) if you expect your user to be able to call `docker` without `sudo`. Docker usually requires `root` to run on Linux, unlike `podman`.

#### A quick note about Docker ubiquity

Docker is available for installation on macOS, Windows, and many Linux distributions. Docker relies on the native features of the Linux kernel to implement its isolation mechanisms as an underlying component of the architecture, so when you run Docker on your Mac or Windows machine, you're actually running a Linux VM - it's just wrapped up and hidden from you quite a bit. Running Docker on a Linux distribution, like Fedora or Debian, uses the native features of the OS on which it's installed.

Because Docker encapsulates the configuration of this Linux VM for you, you may prefer using that on platforms other than Linux. Docker and Podman both conform to the Open Container Initiative specifications for images and runtimes - and Podman intentionally keeps its command line interface similar to make migration from Docker easier, changing how those kernel features are used but not how they're exposed to you.

### If you need to be able to curl and can't

Supposing you did exercise 1 without `curl` and used the pure-python implementation, and are using a shell like `sh`, `bash`, or `zsh`, you could define this little function to help out:

```sh
function curl { python -c "from urllib.request import urlopen; print(urlopen('http://${1}').read().decode().strip())" ; }
```

This isn't great for general-purpose replacement of `curl`, but will work for our controlled examples here.

## Anatomy of a Dockerfile

### Dockerfile vs Containerfile

Dockerfile is a convention that, since popularized by Docker, has stuck around. A Containerfile would more appropriately describe the collection of steps that build OCI container images, but almost all tooling looks for a file named simply "Dockerfile" by default.

### What's in the box

Open or `cat` the file located at `ex2/Dockerfile.py`, relative to the project root.

```console
$ pwd
/home/james/Projects/containers-for-embedded-ws
$ cat ex2/Dockerfile.py
FROM registry.fedoraproject.org/fedora:33

COPY helloworld-py /app

RUN dnf -y install python3-pip \
 && pip3 install /app

ENV FLASK_APP=helloworld.app

CMD ["flask", "run", "--host=0.0.0.0"]
```

Let's unpack that a bit.

```dockerfile
FROM registry.fedoraproject.org/fedora:33
```

This means that we're starting from a base image running Fedora 33. Let's look into that image's metadata:

```sh
docker inspect registry.fedoraproject.org/fedora:33
```

Note that `docker` will pull the image before inspecting it, but you could also use `skopeo inspect docker://registry.fedoraproject.org/fedora:33` to inspect the image directly from the registry - without downloading the layers first.

In particular, notice this section of the `docker inspect` output:

```json
[
    {
        "RootFS": {
            "Type": "layers",
            "Layers": [
                "sha256:d9e1d1e08de26f234a83c6c737827884dd15c68c80714a5a973d245ed456f7a1"
            ]
        }
    }
]
```

There is a single layer in this image (yours may not match the above if they updated the base image), and we are presented with the hash of that layer. Any other images that build on top of this image will have this same layer, and others defined by the rest of our Dockerfile. What makes a layer, though, and why is this image a single layer?

The next line in this Dockerfile says this:

```dockerfile
COPY helloworld-py /app
```

This will copy something from the path at `helloworld-py` and place it at `/app`. If we executed this from the project root, `helloworld-py` would be a folder that contains the Python package we worked with in exercise 1.

This line, that starts with `COPY`, adds a new layer all by itself. Every instruction in the Dockerfile results in a completely empty copy-on-write (COW) layer being populated with the results of that step, then the layer is hashed, and follow-on layers continue to execute.

This means that for the Fedora image to be a single layer, it had to be created in a single step. This single step could have been simply a `COPY` line after calling `FROM scratch`, which has been a [special case](https://github.com/moby/moby/pull/8827) in Docker for a long time, and made it into the OCI spec [almost immediately](https://github.com/opencontainers/umoci/blob/05c30365a67487176d42b4bf0fb5db9459dd54ec/CHANGELOG.md#000-rc2---2016-12-12). If you install a Linux distribution into a folder (you can just `dnf install --installroot=/some/folder <some package>` to install a full Fedora system into a folder, for example), and then `COPY` the whole folder into `/`, you have a container image with most of a distribution in it!<sup>1</sup>

Knowing that every line you put in a Dockerfile adds a new layer makes the next line make a bit more sense:

```dockerfile
RUN dnf -y install python3-pip \
 && pip3 install /app
```

This is a single Dockerfile command, split onto two lines for readability. The `\` character is an escape that ignores the newline following it, so this line is equivalent to `RUN dnf -y install python3-pip && pip3 install /app`. This line makes sure we have `pip3` in our container's environment, which is the same Python package manager we used in exercise 1, and then installs our application - but not in editable development mode, or in a virtual environment. We don't need to worry about editable mode, because our container image is immutable (every layer is hashed!). We don't need to worry about a virtual environment, because we're not dirtying up our own systems inside this container.

This COW layer would contain only the differences inside the container file system that are different from the previous step, where our source code was copied in, and this step, where the python package is installed. In this case that mostly means that the files are now in /usr/local/lib/python3.9/site-packages, because that's where `pip3` installs packages on Fedora 33 by default. Note that yes, we just modified a system directory with a `pip3 install` - and we didn't even call `sudo` or anything. Inside the container we appear to be running as `root`, so you may have even caught a warning message from pip flying by about using `pip` as root on your system. Mine said:

```console
$ podman build . -f ex2/Dockerfile.py -t helloworld-py
STEP 1: FROM registry.fedoraproject.org/fedora:33
STEP 2: COPY helloworld-py /app
--> aaae2f69dd5
STEP 3: RUN dnf -y install python3-pip  && pip3 install /app

# trimmed for brevity

WARNING: Running pip install with root privileges is generally not a good idea. Try `pip3 install --user` instead.
```

Our next Dockerfile command is an interesting one:

```dockerfile
ENV FLASK_APP=helloworld.app
```

This command doesn't add anything new to the filesystem - but it does add new metadata to the container image. This metadata tells your runtime to export the environment variable, in the same way that we manually did in exercise 1, before continuing any further. As the image continues to build, any new `RUN` commands would have it exported as well. `ENV` declarations are pretty useful, considering the popularity of OS environment variables as a means of parameterizing applications or changing their behavior.

The last line in this Dockerfile is where the rubber meets the road for us:

```dockerfile
CMD ["flask", "run", "--host=0.0.0.0"]
```

This line adds another metadata layer to let our runtime know what command to run when we instantiate a container from this container image. Note that there's another directive, `ENTRYPOINT`, that can be used for certain things as well. `ENTRYPOINT` and `CMD` behave a little differently, which you can read a bit about from the [Docker documentation on the subject](https://docs.docker.com/engine/reference/builder/#entrypoint). Note that the `--host` argument to `flask run` will be obviously important soon...

### Building this image

Ensure that you're in the project root (that is, `ls` returns the `ex` directories as well as the `helloworld-js` and `helloworld-py` directories), and run the following to build this image:

```sh
docker build . -f ex2/Dockerfile.py -t helloworld-py
```

This means that we're building within the context of the project root (so our `helloworld-py` folder lines up), using the Dockerfile we've been exploring, and we're tagging this image as `helloworld-py`. Really, `docker` and `podman` are nice enough to give this image a fully qualified image name of `localhost/helloworld-py:latest`, but we're going to keep using the shorthand.

You should see some things scroll by and, no matter what platform you're on, you should end up on a line that says `STEP 6: COMMIT helloworld-py`. This means that we have executed all of these steps serially, and packaged all of the layers into a single manifest - which we hashed and named. This manifest, whether by hash or name, now refers to the collection of layers that we've been talking about - including filesystem changes on those lines that change the filesystem, and metadata on those layers that didn't.

### Exploring our image

Because we just used `CMD` to define how to run our containers, we can provide arbitrary other commands to run inside the container image when we instantiate a container from them. Let's use a shell to poke around inside our container:

```sh
docker run --rm -it helloworld-py bash
```

You should get a shell prompt that ends in `#` indicating that you are root. Let's see what's in this image:

```console
[root@15f0e4ebcc9b /]# ls -l /app  # You can see the artifacts we COPY'd in here
total 4
-rw-r--r--. 1 root root 211 May 22  2020 setup.py
drwxr-xr-x. 4 root root  51 Mar  3 15:58 src
[root@15f0e4ebcc9b /]# ls -l /usr/local/lib/python3.9/site-packages/helloworld  # You can see where pip installed our package to here
total 4
-rw-r--r--. 1 root root   0 May 22  2020 __init__.py
drwxr-xr-x. 2 root root  63 Mar  3 23:28 __pycache__
-rw-r--r--. 1 root root 167 Mar  3 16:01 app.py
[root@15f0e4ebcc9b /]# cat /usr/local/lib/python3.9/site-packages/helloworld/app.py  # You can see that this is just our app.py, in one of the system's python site-packages directories
from socket import gethostname

from flask import Flask
app = Flask(__name__)


@app.route('/')
def hello_world():
    return f'Hello, world, from {gethostname()}!\n'
[root@15f0e4ebcc9b /]# echo $FLASK_APP  # And here you can see that our environment variable is set
helloworld.app
[root@15f0e4ebcc9b /]# exit
exit
```

The name of my container is not the same as the name of your container. When a container is instantiated, it usually gets a randomized container ID from the runtime, which sets that ID as the hostname inside the container. You can explicitly set the hostname if you need to, but it shouldn't matter too bad for the most part.

```console
$ docker run --rm -it --hostname helloworld-py helloworld-py bash
[root@helloworld-py /]# echo $HOSTNAME
helloworld-py
[root@helloworld-py /]# exit
exit
```

Every time you instantiate a new container from this image, the image is unpacked layer-by-layer. It's a fresh copy of the environment, which we defined in the Dockerfile. Let's run this container image without extra arguments and see what we see:

```sh
docker run --rm helloworld-py
```

You should see Flask start up very similarly to how it did for us earlier. In another terminal or window, again curl the port we know it's listening on:

```console
$ curl localhost:5000
curl: (7) Failed to connect to localhost port 5000: Connection refused
```

The reason this didn't work is important to understand. Inside the container, our Flask application is listening on port 5000/tcp. Outside of the container, nothing is listening on that port. By namespacing networks down from the kernel's root namespace, we have isolated the container networking from our host - and we haven't instructed the runtime to allow anything through. Most runtimes will allow the container to egress through your host automatically (usually using some form of NAT), but not allow ingress by default. We can specify individual ports to forward, though. `Ctrl+C` your flask server and it should quit gracefully, then try running it again like this:

```sh
docker run --rm -p 5000:5000 helloworld-py
```

And rerun your `curl`:

```console
$ curl localhost:5000
Hello, world, from 36abd89b4292!
```

This is why the `flask run` call in the Dockerfile `CMD` had the host parameter - it needed to ensure that the webserver would be able to accept requests that didn't originate from inside the container. Docker, depending on your network configuration, may not need this setting; you should set it anyways, for those environments that do.

You can `Ctrl+C` the server again - we're done with that for now.

## What did that even get us?

There's a lot of complex tooling involved in running Python applications locally. There's a lot of complex tooling involved in running containers locally, too. Why did we just swap out all of our tooling complexity?

The answer, of course, is that we got a lot more than a consistent Python environment. Build the other Dockerfile in that directory:

```sh
docker build . -f ex2/Dockerfile.rs -t helloworld-rs
```

Feel free to explore that image in the same way, or look over the Dockerfile to see how different it is. You'll probably have a bit of time to do that during the initial build. We performed the same kind of steps - installing language-specific tooling, moving our application code into the container image, compiling the application, and specifying how to run the application - but in a pretty different order and manner.

The Rust example isn't much more complicated than our Python example, but it may seem that way. We used a convention called [multistage builds](https://docs.docker.com/develop/develop-images/multistage-build/) to enable us to cache layers that go through the process of downloading and building dependencies for our helloworld example in different stages. These intermediate stages enable us to save a lot of time as we compile not only our simple application, but also the complex dependencies of it, because of the way that image layer cacheing works for container image builds.

The best part of these multistage builds is that the final image that's produced just has Fedora and our compiled binary laid on top. This is sound production image building practice, in general, though there are other ways to accomplish similar goals with less Dockerfile complexity - for example [S2I](https://codeburst.io/source-to-image-s2i-by-example-9635c80b6108). Multistage builds, when well structured, give us that as well as a better local development experience.

Let's make sure this example works like we expect:

```sh
docker run --rm -it -p 8000:8000 helloworld-rs
```

And in another window, give it a `curl`:

```console
$ curl localhost:8000
Hello, world, from 67bfd19c410f!
```

If that works like you expect, exit the web app with `Ctrl+C` again and make a small change to `helloworld-rs/src/main.rs` in a way that won't affect the dependencies, like changing line 9 to have a slightly different output string (maybe all caps?), then rerun the build. It should be pretty fast, despite recompiling an entire web application! My initial build took 48 seconds (my computer's pretty quick) and my follow-on builds after changing the program only took around 8 seconds. You should see a speedup no matter what, though, as you take advantage of the cached layers.

## The point

You've built and packaged two applications, in totally different programming languages, with exactly one set of tooling installed on your machine. Both should work for all of us with a functional container runtime. The fact that both of these applications do the same thing isn't the point, and the fact that they're simple web applications doesn't mean that's all we can do. We've gained process portability across any Linux system - even a VM for the Windows and macOS users - with consistent environments for installing and running our images. **And**, for those of us using `podman`, they're running isolated even from other processes running as our own user, let alone the rest of the system we're running on. **And**, because of the `USER` line in the multistage build for our Rust application, even inside the container the process has no privilege! **And** these images are _very_ easy to distribute.

### A note on distribution

Container Registries are another thing specified in the OCI spec, under the dedicated [Distribution Spec](https://github.com/opencontainers/distribution-spec). We won't cover how to interact with registries here - but you've been downloading images from them this whole time. Uploading your own images for others to use is easy once you have an account on one (or you can instantiate your own registry!).

---

<sup>1</sup>: The Fedora project is actually not doing a single `COPY` and releasing that - they're actually squashing images, but this gets a bit more complex than we should cover here :)
