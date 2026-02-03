# git-brdi

Interactively delete local git branches, sorted by last-touched date.

`git brdi` walks through each of your local branches (most recent first, skipping the default branch) and prompts you to delete or skip it.

```
(1/4) Delete feature-login (last touched 3 days ago) [y,n,q,?]? y
    Deleted branch feature-login (was abc1234).
(2/4) Delete old-experiment (last touched 2 weeks ago) [y,n,q,?]? n
(3/4) Delete wip-refactor (last touched 3 months ago) [y,n,q,?]? q
Quit.

Done. Deleted 1 branch, skipped 1.
```

## Install

```sh
cargo install git-brdi
```

This installs the `git-brdi` binary. Because it follows the `git-<name>` convention, git picks it up as a subcommand:

```sh
git brdi
```

## Usage

Run `git brdi` from any git repository. For each branch you'll be prompted with:

- **y** - delete the branch (`git branch -d`)
- **n** - skip
- **q** - quit
- **?** - print help

If a branch has unmerged changes, `git branch -d` will fail and you'll be asked whether to force-delete it (`git branch -D`).
