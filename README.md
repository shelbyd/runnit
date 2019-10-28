# runnit

Run all the commands that come through stdin.

```sh
cat <<<EOF
Something went wrong. Try the following command to fix it:
  fix_the_thing --arguments="something useful"
EOF | runnit
```

The above snippet will execute `fix_the_thing --arguments="something useful"`.

## Install

```sh
cargo install runnit
```

## Why

Many test commands will suggest commands to run to fix errors. This command
allows those commands to run automatically while getting out of the way of the
edit-compile-test cycle.
