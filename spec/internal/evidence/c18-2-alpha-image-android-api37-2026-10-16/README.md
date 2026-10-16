# C18.2 Android API 37 ARM64 AVD — 2026-10-16

## 실행 환경

- 대상은 Android API 37 ARM64 AVD `emulator-5554`뿐이다. 연결된 Samsung 실기기와 다른 emulator serial에는 설치·launch·logcat·screenshot 명령을 보내지 않았다.
- backend는 OpenGL ES / ANGLE Vulkan SwiftShader이며 app log의 surface format은 `Rgba8Unorm`이다.
- C18.2 route run ID `663cf723570d`, PID `29292`, surface generation `2`, document revision `11`, render-tree revision `2`다. viewport는 `80×40 CSS px`, SurfaceView는 `210×105 px`, scale은 `2.625×2.625`다.
- C18.2 opt-in APK SHA-256: `fbf228fe1913bbe85516fef8d78ca6c770d177c8e6acecc465b8ed9747a9e5cc`.

## C18.2 결과

실제 live V8 document의 image 하나를 WGPU로 그린 뒤 PixelCopy drawable 표본 10개와 앱 전체 화면 PNG의 대응 pixel 10개가 Chrome 155 oracle과 exact 일치했다. 각 화면의 제목·부제·상태·배경 anchor 4점도 fixture 밖에서 exact 비교했다. 투명 띠는 backdrop을 보존했고 alpha 128 띠는 `(118,37,65,255)`, 불투명 띠는 `(32,200,64,255)`였다. Comparator는 `matched`, PixelCopy `10/10`, screenshot `10/10`, anchors `4/4`를 반환했다.

| 근거 | 파일 | SHA-256 |
| --- | --- | --- |
| AVD 전체 화면 | [full-screen.png](./full-screen.png) | `9476791f19783c9a058a1156f8781a135e6d31c3a566f9601683a74d514947ae` |
| SurfaceView PixelCopy PNG | [pixelcopy-surface.png](./pixelcopy-surface.png) | `dc0808e600fa9a1337f3cd66d8063d94ec5844012e0fbd7cafa02ed0206098c7` |
| PID 29292 원본 logcat | [logcat.txt](./logcat.txt) | `90a58023af9f21875c61ee44730e048104926d8bb846219e1aba5c1448098801` |
| 전체 화면·drawable 좌표 manifest | [post-compositor-observation.json](./post-compositor-observation.json) | — |

재현 비교:

```sh
node tools/css-reference/compare-c182-alpha-compositor-pixels.mjs \
  spec/internal/evidence/c18-2-alpha-image-android-api37-2026-10-16/post-compositor-observation.json
```

## C18.1 회귀 결과

C18.2와 같은 렌더러 source에서 C18.1 route도 실행했다. visible border overpaint, border 바깥 overflow 첫·중간·마지막 pixel, image 뒤 영역과 hidden padding/border/clip 바깥의 9개 표면 pixel이 모두 exact였다. 전체 화면 대응 표본 `9/9`, fixture 밖 anchor `4/4`도 통과했다. 이 run은 C18.1·C18.2 fixture를 함께 켠 APK를 사용했다.

- PID `29411`, run ID `6677ac8448b0`, C18.1 drawable generation `2`, document revision `27`, render-tree revision `5`.
- APK SHA-256: `90dec4de82788547fa9e8750d0448239aca9261755349352fba8ff691f9c175a`.
- [전체 화면](./c181-regression/full-screen.png) · [PixelCopy](./c181-regression/pixelcopy-surface.png) · [원본 logcat](./c181-regression/logcat.txt) · [9-point manifest](./c181-regression/post-compositor-observation.json).
- Comparator: `node tools/css-reference/compare-c181-compositor-pixels.mjs spec/internal/evidence/c18-2-alpha-image-android-api37-2026-10-16/c181-regression/post-compositor-observation.json` → `matched`, surface `9/9`, screenshot `9/9`, anchors `4/4`.

## 해석 범위

이 결과는 지정 AVD의 ANGLE/SwiftShader virtual GPU와 PixelCopy/emulator screenshot에 한정한다. 실기기, hardware GPU, Android display controller의 물리 scanout과 일반 URL image loader는 확인하지 않았다. 첫 실행의 alpha-128 1-step mismatch와 원본 로그는 `first-pass-*` 파일에 남겨, 보정 전 실패를 숨기지 않는다.
