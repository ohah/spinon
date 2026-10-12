# C11.2.1 Android 실기기 runtime·PixelCopy 행렬

- 실행 ID: `c1121-android-20261012T161006179Z-4ffd28bc` (UTC, 한국 시간 2026-10-13)
- 대상: Samsung SM-S731N · Android 16/API 36 · serial `R5KYB06SMPE` · qemu `0`
- SurfaceFlinger renderer: Samsung Xclipse 940, Vulkan 1.3.279 위 ANGLE, OpenGL ES 3.2
- APK SHA-256: `04ac816f0e573ff91e73cc7d48d90eea252d9dfa19c1b68d05c7275b21c62c32`
- 고정 입력: inventory `b2801e708e4c1ad540e119dcdba04d7c215e4029d5ec5627683e15ede8ef368b` · HTML `e9b080bbc65901b17f7dead60eb8071df8ecbefee2ea62f1318776848e75f78d` · Chrome 154 reference `5bbbd041e2e59fda32c684f7290f61bff41ab6dda9ab9f77ce2fb36f0e0ee034`
- 결과: 10/10 scene · 성공 6/6 · 기대 거부 4/4 · 21 node · 최대 geometry 오차 0 CSS px (허용 0.5)
- PixelCopy: 성공 6개 scene · 20/20 sample exact · 최대 channel 오차 0
- 각 scene은 새 앱 process와 HostDocument를 사용했다. 거부 scene은 기대 오류·partial frame 없음·non-empty WGPU 제출 없음으로 판정했다.
- 전체 summary: [comparison.json](./comparison.json). 앱 빌드·설치 로그, scene별 logcat·화면 캡처, PixelCopy 원본과 비교 JSON을 이 디렉터리에 보존했다.
- 이 결과는 해당 기기와 고정 C11.2.1 subset에 한정한다. 다른 기기군·WPT 적합성·제품 성능 또는 전체 CSS Grid 동작을 주장하지 않는다.
