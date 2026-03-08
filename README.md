# git-contributors

This is a simple git subcommand to visualize author activity over time as a histogram in the terminal, like the "Contributors" view in GitHub Insights.

![Example graph](images/example-git.png)

## Install

Homebrew:

```sh
brew install basictofu/tap/git-contributors
```

Then you show be able to run "git contributors" from within any git repo.

## Development

See `make install` for testing on local machine.

As long as the executable is placed on the path, git should pick it up as a new subcommand.

To test everything:
```sh
make test
```
