# git_tool

Run a sequence of `git` commands to
* replace a section of branch history with a summary commit
* preserve the summarized history with an archive tag named with the summary commit's hash

## Function
The tool accepts
* parent: the branch/hash of the parent of the section to be summarized
* section: the branch/hash of the end (inclusive) of the section to be summarized
* and a commit message for the summary commit

When run on a branch history like
```
 o--o--o--o  parent
           \
            o--o--o--o  section
                      \
                       o--o--o--o  current_branch
```
the result will be
```
            o--o--o--o  archive/<summary commit short hash>
           /
 o--o--o--o  parent
           \
            o (the summary commit)
             \
              o--o--o--o  current_branch

```
This style of history preservation allows for easy browsing of a main branch without loss of potentially useful detail.

The `archive/<hash>` tag uses the `git` short hash (the first 7 characters) to record the hash of its summary commit.


## Failure recovery
If a failure of a command is detected, the tool will
* immediately stop making changes to the `git` repo
* return a log of previous commands that succeeded

This allows manual restoration of the repo state, if any is needed.

## Intent

This tool accelerates an experimental repo organization style I'm trying. The main branch will tend to look something like
```
... ---o---------------------------o--------- ... main
        \                           \
         o--o--o--o archive/<hash>   o--o--o--o archive/<hash>
```
which allows a quick reduction to a display of the summary commits by viewing the main log alone.

## License

MIT No Attribution: [LICENSE-MIT-0](LICENSE-MIT-0) or https://opensource.org/license/mit-0/
