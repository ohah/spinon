# C12.4 Android 실기기 반복 session 수명 검증

- run ID: c124-lifecycle-20261011T182124208Z-0f637aab
- 기기: SM-S731N · Android 16/API 36 · serial R5KYB06SMPE
- APK SHA-256: f9d51f5430608387cfe0efb78a667607e715a9d6f137d2d30e438b701dae3d37
- 단일 process 안에서 16 cycle × cycle당 session 8개를 생성·해제했다.
- 매 cycle의 V8 owner 8개와 CSS worker 8개 TID가 생성 후 존재하고, 동기 해제 뒤 모두 사라짐을 확인했다.
- baseline, cycle 1·8·16의 peak, 최종 process PSS/RSS와 VmSize/VmData를 저장했다. 이 값은 관찰값이며 누수 판정으로 사용하지 않는다.
- renderer/surface/layout evaluation은 실행하지 않았다. WPT·CSS 적합성 근거가 아니다.
- stack-only commit/high-water, 전역 allocator OOM, 타 기기와 iOS 실기기는 남아 있다.
