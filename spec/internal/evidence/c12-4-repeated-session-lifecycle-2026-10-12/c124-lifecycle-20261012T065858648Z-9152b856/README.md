# C12.4 Android AVD 반복 session 수명 검증

- run ID: c124-lifecycle-20261012T065858648Z-9152b856
- 기기: sdk_gphone16k_arm64 · Android 17/API 37 · serial emulator-5554
- target: Android API 37 AVD · 시뮬레이터 표면 결과
- APK SHA-256: 6091fd0f758976bcad64615ca388769ca5abc18fe31637f8559932544e67c2bd
- 단일 process 안에서 16 cycle × cycle당 session 8개를 생성·해제했다.
- 매 cycle의 V8 owner 8개와 CSS worker 8개 TID가 생성 후 존재하고, 동기 해제 뒤 모두 사라짐을 확인했다.
- baseline, cycle 1·8·16의 peak, 최종 process PSS/RSS와 VmSize/VmData를 저장했다. 이 값은 관찰값이며 누수 판정으로 사용하지 않는다.
- renderer/surface/layout evaluation은 실행하지 않았다. WPT·CSS 적합성 근거가 아니다.
- stack-only commit/high-water와 전역 allocator OOM은 이 실행에서 측정하지 않았다.
