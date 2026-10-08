# Android 16 실기기 회귀 확인

- 기기: Samsung SM-S731N, Android 16, API 36, ARM64, 1080×2340, density 450. 식별 serial은 보존하지 않았다.
- 앱: dev.spinon.bootstrap Debug. APK와 설치 APK SHA-256이 모두 1639d939a34c7a7dfcf99b53b906d73c12951eae99b87757ecc2e8589b1e572e다.
- cold launch: native library load와 JS bootstrap 결과를 확인했다. 기본 bootstrap 화면은 UI fixture가 없어 흰색이며, R08 시각 검증으로 계산하지 않았다.
- WGPU probe: 새 process에서 present-fence/async-fence debug extras를 켰다. Vulkan backend와 Samsung Xclipse 940 초기화, SurfaceView 및 파란 GPU 사각형을 확인했다.
- synthetic input: ADB tap 13회가 input→WGPU submit→실제 TransactionStats callback→usable async fence wait로 각각 13/13 연결됐다. sequence/revision은 1–13, generation은 1, 마지막 queue는 pending=0/active=0/depth=0이다. 화면 activation count 13, 주황색 도형을 캡처했다.
- 입력 출처 대조: sec_touchscreen은 INPUT_PROP_DIRECT 장치다. 별도 ADB input tap control 동안 raw touchscreen event가 0건이었다. 약 30초 direct capture에도 접촉이 기록되지 않아 직접 손가락 positive sample은 없다.
- 한계: input_source=unknown, target_vsync_id=-1이다. event-to-present latency, VSync, optical scanout 및 성능 비교를 주장하지 않는다. 이 API 36 기기 결과는 API 29 loader 검증을 대신하지 않는다.
- 복구: Spinon을 종료하고 Chrome foreground를 복구했다. screen timeout 30000ms, brightness mode 0, brightness 248은 전후 같았다.

원본 로그와 캡처는 이 폴더에 있다. 13개 sequence exact join 판정은 join-report.md를 참고한다.
