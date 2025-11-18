# Gradescope Autograder Guide

This document details how to use the autograders in this `homeworks` repository.

# Create

Do you best to follow along the code for the existing autograders, and you can probably figure out
how they work. If you have a question, your best bet is to probably just contact
[Connor Tsui](mailto:connortsui20@gmail.com) directly.

# Build

Run this command to build a specific autograder, replacing the docker name and the autograder name:

Make sure 

```sh
docker buildx build --platform=linux/amd64 -t <container-name> <path-to-autograder>
```

For example:

```sh
docker buildx build --platform=linux/amd64 -t connortsui/cardlab_autograder week3/autograder/
```

_You can also omit the `buildx` command, but that is being deprecated soon._

# Local Run

To run the autograder locally, you have to first create a local `submission` folder. This
represents the files that a student submits to Gradescope. For most of the homeworks, this just
means placing a `src` folder with relevant `.rs` files into the `submission` folder. At the time of
writing, the only homeworks that don't follow this submission protocol are `FilterLab` and
`GrepLab`.

You should also create a local `results` folder. Technically you don't need to do this, but if you
let Docker create it for you, you will have to use `sudo` permissions to delete the `results`
folder later.

```sh
mkdir submission
mkdir results
```

Run this command to run the autograder, which will store the results in your local `results` folder:

```sh
docker run \
    -v <path-to-submission-folder>:/autograder/submission \
    -v <path-to-results-folder>:/autograder/results \
    <image-name> \
    /autograder/run_autograder \
    --rm
```

The `--rm` flag removes the flag after the autograder has run, and the `-v` arguments effectively
"mirror" the directories into the container.

For example, if the `submission` and `results` folder are both in the current directory:

```sh
docker run \
    -v ./submission:/autograder/submission \
-v ./results:/autograder/results \
    connortsui/cardlab_autograder \
    /autograder/run_autograder \
    --rm
```

Once it has finished running, you can find the output in `<path-to-results-folder>/results.json`.

# Publish

To publish the container, run this command (replacing the name with the correct container name):

```sh
docker push connortsui/cardlab_autograder:latest
```
