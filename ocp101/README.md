# OpenShift 4 101

## Prerequisites

You will need a Red Hat Individual Developer account (they're free!), a system on which you are an administrator with around ~8 vCPUs (a quad-core processor) and 16 GB of RAM, and a personal GitHub account (they're also free!)

### Red Hat Individual Developer Account

You're going to be running an OpenShift cluster locally - in a single node, on your own machine. You can spin these up completely free with a Red Hat Individual Developer account and experience the majority of OpenShift's features on a small scale, so you should sign up for one if you don't already have access to a Red Hat account with entitlements on it.

1. Head to the [Red Hat Developers portal](https://developers.redhat.com/) and click the link in the top right to log in:
    [!login](./login.png)
1. Click on the link where it says "Don't have an account? Create one now."
1. Fill out the information.
1. Welcome to the Red Hat Developers program!

### Install CodeReady Containers on your system

CodeReady Containers is available for Linux, macOS (on x86_64 Macs), and Windows. It uses the native virtualization provider for your platform to instantiate the single-node OpenShift cluster. Configuring it requires the ability to use that native virtualization provider.

1. Navigate to the [CodeReady Containers](https://cloud.redhat.com/openshift/create/local) installer section (You can also get here less directly by going to [cloud.redhat.com](https://cloud.redhat.com)'s [Cluster Manager](https://cloud.redhat.com/openshift/) link, clicking on [Create cluster](https://cloud.redhat.com/openshift/create), and choosing the Local tab).
1. Download the appropriate release for your operating system, unpack the installer, and place it somewhere you can get to it (On my Fedora system, I keep it in `~/.local/bin/crc`).
1. Copy or download your pull secret (this is unique to every account and identifies you as a Developer Subscription holder) and be ready to paste it in to the crc command line.
1. Run the setup and start commands, filling in information as asked:
    `crc setup`
    `crc start`
1. Record the login information `crc` leaves you with on the terminal - this will be necessary to access your local OpenShift cluster.
1. Optionally, review the [documentation on CodeReady Containers](https://access.redhat.com/documentation/en-us/red_hat_codeready_containers/).

#### Optional backup cluster plan

If, for whatever reason, you can't get an OpenShift cluster up and running with CodeReady Containers, you can [spin up a free one](https://learn.openshift.com/playgrounds/openshift46/) with very limited resources - but these should be enough to conduct most of these exercises. They last only about 60 minutes, so you may have to start over if you take a bit long on some of the exercises, but you can just keep reprovisioning them and learning! This free cluster, from [learn.openshift.com](https://learn.openshift.com), is designed to give you a playground to experiment and learn with. There are lots of other courses available there to better understand this tech, though, so you're welcome to explore those at will.

### A personal GitHub account

Part of the OpenShift 101 workshop involves forking a GitHub project and making modifications to it, seeing how OpenShift can help you deploy that application through changes. For this, you'll need a GitHub account.

1. Signing up with GitHub is very easy - there's just [one link](https://github.com/join?source=header-home) to click! Even if you've never made a single git commit, or written any code, this should be a pretty approachable part of the workshop.

## The labs

The lab guides for OpenShift 101 are maintained at [RedHatGov](https://redhatgov.io/workshops/openshift_4_101/). These guides walk you through how to interact with your OpenShift cluster, including using the built-in web terminal that includes the only command line client you'll need to interact with the cluster.

Remember as you walk through the linked lab guides completing exercises that your cluster, if you successfully set up CodeReady Containers, is accessible through your browser locally at `https://console-openshift-console.apps-crc.testing` - and that you are a Cluster Administrator on your local cluster.

## Closing

If you'd like more time to explore OpenShift in a guided fashion, communicate that with your Red Hat team after you finish the 101 content and we can select an appropriate workshop to go deeper into the areas of OpenShift that you're most interested in, or would be responsible for.

Thanks for sticking around as we learn containers from the ground up, and I hope you can see how they would be useful - even for embedded systems developers. Containers and Kubernetes are rapidly gaining traction as a viable platform for all kinds of embedded systems in today's world - even making it onto the [International Space Station](https://www.ibm.com/cloud/blog/ibm-develops-a-unique-custom-edge-computing-solution-in-space)! The ability to host Linux processes in isolation on a cluster of computers, acessible from anywhere and using deployment artifacts that span the hybrid cloud, is a useful upgrade no matter how it is you're developing, delivering, and maintaining software today.
