#!/usr/bin/env bash
set -euo pipefail

serial="${1:-emulator-5554}"
input_device="${2:-/dev/input/event1}"
evidence_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

if [[ "$serial" != emulator-* ]]; then
  echo "실기기 serial은 허용하지 않습니다: $serial" >&2
  exit 2
fi
if [[ ! "$input_device" =~ ^/dev/input/event[0-9]+$ ]]; then
  echo "evdev input path 형식이 올바르지 않습니다: $input_device" >&2
  exit 2
fi

avd_name="$(adb -s "$serial" shell getprop ro.boot.qemu.avd_name | tr -d '\r')"
api_level="$(adb -s "$serial" shell getprop ro.build.version.sdk | tr -d '\r')"
abi="$(adb -s "$serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
uid="$(adb -s "$serial" shell id -u | tr -d '\r')"
if [[ "$avd_name" != spinon_api37_1_compat16k || "$api_level" != 37 || "$abi" != arm64-v8a || "$uid" != 0 ]]; then
  echo "API 37 ARM64 전용 root AVD가 필요합니다: avd=$avd_name api=$api_level abi=$abi uid=$uid" >&2
  exit 2
fi

if ! adb -s "$serial" shell dumpsys input | tr -d '\r' | python3 -c '
import re
import sys

target = sys.argv[1]
dump = sys.stdin.read()
event_hub, reader = dump.split("Input Reader State", 1)
paths = {}
current_id = None
current_name = None
for line in event_hub.splitlines():
    match = re.match(r"\s*(\d+): (\S+)$", line)
    if match:
        current_id, current_name = match.groups()
    match = re.match(r"\s+Path: (\S+)", line)
    if match and current_id:
        paths[current_id] = (match.group(1), current_name)

for block in re.split(r"(?m)(?=^  Device \d+: )", reader):
    event_hub_id = re.search(r"EventHub Devices: \[\s*(\d+)\s*\]", block)
    mode = re.search(r"Touch Input Mapper \(mode - (\w+)\)", block)
    viewport = re.search(r"Viewport INTERNAL: displayId=(-?\d+).*?deviceSize=\[(\d+), (\d+)\]", block)
    contacts = re.search(r"Last Cooked Touch: pointerCount=(\d+)", block)
    if event_hub_id and paths.get(event_hub_id.group(1), (None, None))[0] == target:
        device_name = paths[event_hub_id.group(1)][1]
        values = (mode.group(1) if mode else "none",
                  viewport.group(1) if viewport else "none",
                  viewport.group(2) if viewport else "none",
                  viewport.group(3) if viewport else "none",
                  contacts.group(1) if contacts else "unknown")
        print(f"device={device_name} mode={values[0]} displayId={values[1]} size={values[2]}x{values[3]} activePointers={values[4]}")
        sys.exit(0 if values == ("DIRECT", "0", "1080", "2400", "0") else 1)
sys.exit(2)
' "$input_device"; then
  echo "입력 장치는 활성 내부 displayId=0의 직접 터치 장치여야 합니다: $input_device" >&2
  exit 2
fi

send_frame() {
  local commands="$1"
  adb -s "$serial" shell "set -e; $commands"
}

down() {
  local slot="$1" tracking_id="$2" x="$3" y="$4"
  send_frame "sendevent $input_device 3 47 $slot; sendevent $input_device 3 57 $tracking_id; sendevent $input_device 3 53 $x; sendevent $input_device 3 54 $y; sendevent $input_device 3 48 10; sendevent $input_device 3 58 100; sendevent $input_device 0 0 0"
}

move() {
  local slot="$1" x="$2" y="$3"
  send_frame "sendevent $input_device 3 47 $slot; sendevent $input_device 3 53 $x; sendevent $input_device 3 54 $y; sendevent $input_device 0 0 0"
}

up() {
  local slot="$1"
  send_frame "sendevent $input_device 3 47 $slot; sendevent $input_device 3 57 -1; sendevent $input_device 0 0 0"
}

capture() {
  local name="$1"
  adb -s "$serial" exec-out screencap -p > "$evidence_dir/$name.png"
}

capture_contacts() {
  local name="$1" expected="$2"
  if ! adb -s "$serial" shell dumpsys input | tr -d '\r' | python3 -c '
import re
import sys

target, expected = sys.argv[1], int(sys.argv[2])
dump = sys.stdin.read()
event_hub, reader = dump.split("Input Reader State", 1)
paths = {}
current_id = None
current_name = None
for line in event_hub.splitlines():
    match = re.match(r"\s*(\d+): (\S+)$", line)
    if match:
        current_id, current_name = match.groups()
    match = re.match(r"\s+Path: (\S+)", line)
    if match and current_id:
        paths[current_id] = (match.group(1), current_name)

for block in re.split(r"(?m)(?=^  Device \d+: )", reader):
    event_hub_id = re.search(r"EventHub Devices: \[\s*(\d+)\s*\]", block)
    if not event_hub_id or paths.get(event_hub_id.group(1), (None, None))[0] != target:
        continue
    mode = re.search(r"Touch Input Mapper \(mode - (\w+)\)", block)
    viewport = re.search(r"Viewport INTERNAL: displayId=(-?\d+).*?deviceSize=\[(\d+), (\d+)\]", block)
    contacts = re.search(r"Last Cooked Touch: pointerCount=(\d+)", block)
    count = int(contacts.group(1)) if contacts else -1
    print(f"device={paths[event_hub_id.group(1)][1]} mode={mode.group(1) if mode else 'none'}")
    if viewport:
        print(f"displayId={viewport.group(1)} deviceSize={viewport.group(2)}x{viewport.group(3)}")
    for line in block.splitlines():
        if re.search(r"Last Raw Touch:|Raw pointer data:|pointers:|id=|ID bits:|Last Cooked Touch:", line):
            print(line.strip())
    sys.exit(0 if count == expected else 1)
sys.exit(2)
' "$input_device" "$expected" > "$evidence_dir/$name.txt"; then
    echo "입력 시스템 접촉 수가 예상값과 다릅니다: expected=$expected file=$name" >&2
    return 1
  fi
}

cleanup_contacts() {
  adb -s "$serial" shell "sendevent $input_device 3 47 1; sendevent $input_device 3 57 -1; sendevent $input_device 3 47 0; sendevent $input_device 3 57 -1; sendevent $input_device 0 0 0" >/dev/null 2>&1 || true
}
trap cleanup_contacts EXIT

activity_state="$(adb -s "$serial" shell dumpsys activity activities | tr -d '\r')"
if [[ "$activity_state" != *"topResumedActivity=ActivityRecord{"*"dev.spinon.bootstrap/.MainActivity"* ]]; then
  echo "Spinon MainActivity가 실행 중이지 않습니다." >&2
  exit 2
fi
app_pid="$(adb -s "$serial" shell pidof dev.spinon.bootstrap | tr -d '\r')"
if [[ -z "$app_pid" ]] || ! adb -s "$serial" logcat -d --pid="$app_pid" | grep -Fq 'SPINON_C133_READY input=MotionEvent'; then
  echo "opt-in C13.3 MotionEvent fixture가 준비되지 않았습니다." >&2
  exit 2
fi

echo "device=$serial avd=$avd_name api=$api_level abi=$abi input=$input_device uid=$uid"
capture android-initial

# event1 maps raw 0..32767 coordinates to the 1080x2400 internal display.
# The selected point is inside the fixture's inner scrollport.
down 0 201 10922 18240
sleep 0.15
move 0 10922 16876
sleep 0.35
capture android-after-first-pan

# A second slot on the same Linux multitouch device must become POINTER_DOWN.
down 1 202 12743 18240
sleep 0.35
capture_contacts android-input-two-contacts 2
capture android-after-second-pointer-cancel

# Continue moving the original contact while two contacts remain, then after
# the second contact lifts. The runtime must reject all of these moves.
move 0 10922 16400
sleep 0.15
move 0 10922 16000
sleep 0.15
up 1
sleep 0.15
capture_contacts android-input-one-contact-after-second-up 1
move 0 10922 15400
sleep 0.15
up 0
sleep 0.45
capture android-after-all-touches-up

# A fresh single-pointer gesture must work after the complete stream ends.
down 0 203 10922 18240
sleep 0.15
move 0 10922 16876
sleep 0.35
capture android-after-recovery
up 0
sleep 0.35

echo "multitouch sequence completed"
