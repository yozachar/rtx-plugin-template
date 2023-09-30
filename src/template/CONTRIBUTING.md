# Contributing

Testing Locally:

```shell
rtx plugin test <plugin-name> <plugin-url> [--rtx-tool-version <version>] [--rtx-plugin-gitref <git-ref>] [test-command*]

# TODO: adapt this
rtx plugin test <YOUR TOOL> https://github.com/<YOUR GITHUB USERNAME>/rtx-<YOUR TOOL>.git "<TOOL CHECK>"
```

Tests are automatically run in GitHub Actions on push and PR.
