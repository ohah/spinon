# Android API 37 AVD 부팅 실패 분석

- 임시 API 37.0 ARM64 Google APIs AVD에서 `adb`가 `emulator-5554 offline`을 보이다가 QEMU 프로세스가 종료되어 transport가 사라졌다.
- macOS 진단 보고서: `EXC_BAD_ACCESS` / `SIGSEGV`, null 주소 `0x0`, QEMU exit `11` (segmentation fault).
- faulting thread stack:
- `qemu-system-aarch64` +4911184 (anonymous namespace)::sAddressSpaceDevicePing(unsigned int)
- `qemu-system-aarch64` +1753144 address_space_control_write
- `qemu-system-aarch64` +1033664 memory_region_dispatch_write
- `qemu-system-aarch64` +110192 io_writex
- `None` +5337920184
- `qemu-system-aarch64` +61552 cpu_tb_exec
- `qemu-system-aarch64` +63424 cpu_exec
- `qemu-system-aarch64` +150860 qemu_tcg_cpu_thread_fn
- 결론: ADB daemon 또는 APK 설치 경로의 실패가 아니다. 에뮬레이터 QEMU가 guest CPU 실행 중 null 주소에 접근해 종료되어 ADB가 연결을 완료하지 못했다.
- `emulator -help-accel`은 CPU 가속이 x86/x86_64 guest에만 적용되고 ARM64에서는 무시된다고 표시했다. 다음 재현은 API 37.2 ARM64 image와 SwiftShader renderer로 분리 확인한다.
- 원본 macOS `.ips` 및 Android emulator minidump는 공용 진단 디렉터리에 있어 보존한다.
