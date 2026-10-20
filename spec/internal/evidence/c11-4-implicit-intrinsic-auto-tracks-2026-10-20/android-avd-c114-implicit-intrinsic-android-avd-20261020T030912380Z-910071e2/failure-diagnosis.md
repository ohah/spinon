# Android AVD partial capture: ADB logcat timeout

This run is incomplete and is not a passing simulator result. It produced 26 scene PNGs before waiting on `both-fit-single`.

The runner failed while reading logs with:

```text
adb -s emulator-5554 logcat -d -v threadtime SpinonBootstrap:V *:S
spawnSync adb ETIMEDOUT
```

The 120-second request dumped the whole device log buffer on every polling iteration. Its captured output included trace chunks from an earlier run ID (`c114-implicit-intrinsic-android-avd-20261020T024818132Z-0b17dee8`), so the runner transferred unrelated historical output before filtering by PID locally. This was a capture-script/ADB log-volume timeout, not a Grid assertion failure.

`capture-c11-4-auto-repeat-neighbors-simulator.mjs` now requests only the current app PID and the latest 1,000 log lines, with a 30-second command timeout. The bounded `adb logcat -d -T 1000 -v threadtime --pid=1 '*:S'` form was confirmed to return successfully after restarting the same API 37 AVD. A fresh 73-scene run is required for passing evidence.
