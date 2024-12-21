# D-Bus Executor

Launch arbitrary programs via D-Bus.

## Background

Sometimes one needs to launch a program over a D-Bus channel (e.g., if the
originating context isn't permitted to exec arbitrary software). There is a
good utility for this,
[dbus-app-launcher](https://github.com/DvdGiessen/dbus-app-launcher/), but
using this in certain Linux distros can be a challenge as the package needs to
be manually rebuilt every time there's an update to the Haskell runtime or a
dependency.

This is a statically-compilable reimplementation that is intended to be drop-in
replacement and should not require rebuilding.

## Usage

Put this somewhere on your $PATH and then set up a systemd service that
launches it when a D-Bus message comes through. A sample systemd service is
provided in `./net.arusahni.dbusexecutor.service`, and can be placed in
`~/.local/share/dbus-1/services/`. Be sure to update it with the absolute path
to the `dbus-executor` binary.

## Development

Launch the utility

```shell
DBUS_EXEC_LOG=dbus_executor=trace cargo run
```

Fire a message. `gdbus` is recommended since it exposes a nicer, JSON-like
syntax for complex types such as hashmaps and lists.

```shell
gdbus call --session \
    --dest net.arusahni.DbusExecutor \
    --object-path /net/arusahni/DbusExecutor \
    --method net.arusahni.DbusExecutor.Exec.CmdArgsEnv \
    "/bin/ls" '["-la", "/bin/"]' "{\"KEY1\": \"VALUE1\", \"KEY2\": \"VALUE2\"}"
```
