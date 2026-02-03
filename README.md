# git-brdi

`git brdi` provides interactive branch deletion, progressing through each of your local branches and prompting you to delete or skip each branch.

## Install

```sh
cargo install git-brdi
```

This installs the `git-brdi` binary. Because it follows the `git-<name>` convention, git picks it up as a subcommand.

## Usage

```txt
git brdi

(1/4) Delete tdd-lesson-plan (last touched 3 months ago) [y,n,q,?]? y
    Deleted branch tdd-lesson-plan (was abc1234).
(2/4) Delete elixir-dbg-lesson-plan (last touched 2 weeks ago) [y,n,q,?]? n
(3/4) Delete chrome-debugger-lesson-plan (last touched 3 days ago) [y,n,q,?]? q

Done. Deleted 1 branch, skipped 1.
```

By default, `git brdi` progresses through branches by stalest-first, however this can be reversed via `git brdi --reverse`

