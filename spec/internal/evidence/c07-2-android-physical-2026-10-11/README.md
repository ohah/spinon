# C07.2 Android 실기기 보조 실행 근거

## 실행 범위

2026-10-11 Android 16/API 36 실기기에서 기존 `runtime-border-width.js` fixture를 실행했다. 기기는 Samsung SM-S731N, 화면 1080×2340 physical px·density 450이며, WGPU runtime 로그는 Vulkan backend와 Samsung Xclipse 940을 보고한다.

`SPINON_C072_EVAL`은 `status=0`, `layout=ready`, `boxes=6`을 보고했다. 화면 크기 변경 후 environment revision 1에서 레이아웃이 다시 계산됐고 WGPU draw 로그는 revision 1의 6개 box 제출을 기록했다. 화면에는 301×100 CSS px layout geometry 시각화와 runtime summary가 보인다.

## 증거 파일

- [실기기 화면](./c072-android-physical-2026-10-11.png)
- [전체 Logcat](./c072-android-physical-2026-10-11.log) — 시스템 로그 행 끝의 공백만 제거했다. 메시지와 timestamp는 보존했다.
- 기기·Android·화면 정보: Samsung SM-S731N, Android 16/API 36, 1080×2340 physical px, density 450.
- WGPU 제출 정보: Vulkan, Samsung Xclipse 940, surface 847×281 physical px, `presented boxes=6`, `environment_revision=1`.

## 해석 범위

이 실행은 Android 실기기의 V8→Stylo→Taffy→WGPU 경로에서 C07.2 fixture가 계산되고 6개 box가 제출되는지 확인하는 보조 smoke다. 화면의 사각형은 layout geometry 시각화이며 테두리 선은 그리지 않는다. 이 결과는 border stroke 구현, Chrome과의 실기기별 child-frame 수치 비교, 50-node fixture 전체 실행, fractional border 경계의 기기별 동등성, 성능 또는 iOS 실기기 검증을 뜻하지 않는다. 50-node 수치 비교는 별도의 고정 Chrome/Rust fixture 근거를 따른다.

## 파일 digest

| 파일 | SHA-256 |
| --- | --- |
| 화면 | `27a9a922f924a98306eda2b71bea6d9d391e943f4abe2e2ef940f84d66c5d291` |
| Logcat (행 끝 공백 제거) | `93bfa589a5d655588ff43c2fe5431d447d61cb58c894670095e77f3e4941f599` |
