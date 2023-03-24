# git_tool

Run a sequence of `git` commands to
* replace a section of branch history with a summary commit
* preserve the summarized history with an archive tag named with the summary commit's hash

## Function



## Failure recovery
If a failure of a command is detected, the tool will
* immediately stop making changes to the `git` repo
* return a log of previous commands that succeeded

This allows manual restoration of the repo state, if any is needed.

## License

MIT No Attribution: [LICENSE-MIT-0](LICENSE-MIT-0) or https://opensource.org/license/mit-0/
