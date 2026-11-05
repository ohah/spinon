# Android 최종 재빌드 경로 오류

2026-11-05 최종 코드 재빌드 중 첫 시도에 `SPINON_V8_DIR=build/c11-v8-rebuild/v8-source/v8`를 상대경로로 전달했다. Android native build가 이를 linker 이름으로 사용해 아래 진단으로 실패했다.

```text
clang++: error: invalid linker name in argument '-fuse-ld=build/c11-v8-rebuild/v8-source/v8/third_party/llvm-build/Release+Asserts/bin/ld.lld'
```

실패는 앱 입력 파일이나 C11 코드 컴파일 오류가 아니었다. 재시도에서는 같은 고정 V8 checkout을 절대경로 `/Users/yoonhb/Documents/workspace/spinon/build/c11-v8-rebuild/v8-source/v8`로 전달했고, [build-final.log](build-final.log)에 `BUILD SUCCESSFUL`을 기록했다. 실패 경로와 성공 경로 모두 shell trap이 두 앱 asset을 복원했으며 원본 SHA-256은 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`다.
