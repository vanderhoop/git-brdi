# git-brdi

Interactively delete local git branches, sorted by last-touched date.

`git brdi` walks through each of your local branches (most recent first, skipping the default branch) and prompts you to delete or skip it.

```
(1/4) Delete tdd-lesson-plan (last touched 3 days ago) [y,n,q,?]? y
    Deleted branch tdd-lesson-plan (was abc1234).
(2/4) Delete elixir-dbg-lesson-plan (last touched 2 weeks ago) [y,n,q,?]? n
(3/4) Delete chrome-debugger-lesson-plan (last touched 3 months ago) [y,n,q,?]? q

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
