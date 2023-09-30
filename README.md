# rtx-plugin-template

<!-- [![Build](https://github.com/rtxvm/rtx-plugin-template/actions/workflows/build.yml/badge.svg)](https://github.com/rtxvm/rtx-plugin-template/actions/workflows/build.yml) [![Lint](https://github.com/rtxvm/rtx-plugin-template/actions/workflows/lint.yml/badge.svg)](https://github.com/rtxvm/rtx-plugin-template/actions/workflows/lint.yml)

This is an [rtx plugin](https://github.com/rtxvm/rtx-plugin-template/README.md) template with CI to run [Shellcheck](https://github.com/koalaman/shellcheck) and testing with the [rtx test GitHub Action](https://github.com/rtxvm/actions).

## Usage

1. [Generate](https://github.com/rtxvm/rtx-plugin-template/generate) a new repository based on this template.
1. Clone it and run `bash setup.bash`.
1. Force push to your repo: `git push --force-with-lease`.
1. Adapt your code at the TODO markers. To find the markers: `git grep TODO`.
1. To develop your plugin further, please read [the plugins create section of the docs](https://github.com/rtxvm/rtx-plugin-template/README.md).

>A feature of this plugin-template when hosted on GitHub is the use of [release-please](https://github.com/googleapis/release-please), an automated release tool. It leverages [Conventional Commit messages](https://www.conventionalcommits.org/) to determine semver release type, see the [documentation](https://github.com/googleapis/release-please).

## Contributing

Contributions welcome!

1. Install `rtx` tools

    ```shell
    rtx u rust@latest
    ```

1. Develop!

1. Lint & Format

    ```shell
    ./scripts/format.sh
    ./scripts/lint.sh
    ```

    ```ps1
    ./scripts/format.ps1
    ./scripts/lint.ps1
    ```

1. PR changes -->
