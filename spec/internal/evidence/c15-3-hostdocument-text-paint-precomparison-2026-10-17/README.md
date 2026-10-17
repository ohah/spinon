# C15.3 HostDocument text layout→runtime paint 사전 비교

**상태:** 구현 전 고정 입력과 oracle 재사용 기록 · 새 Spinon runtime 결과 아님

## 고정 입력과 독립 oracle

- 입력은 기존 [C15.1 `long-word` HTML fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c15/inline-text-precomparison.html)의 Ahem 10px, 폭 15px Block, direct text `XX`다. fixture SHA-256은 `efba6981d73c2e8d2aa21e38b92f2936ce2a6ecb56bc758ce420a785b5cdaf64`다.
- oracle는 Agent Browser `0.31.1`의 Chrome `155.0.8059.85` DOM geometry이며, 원 관찰 JSON SHA-256은 `8c4673dfb271923cf332845d14faa9bf6fd1101cb72d2f73ec277bc8ec226e86`다. 해당 fixture의 `#long-word`는 flow에 영향을 주지 않는 absolute `.label` span을 포함한다. 새 CSS case를 만들지 않고 Agent Browser live DOM에서 이 label만 제거한 뒤 parent/range/per-character geometry가 byte-for-byte 같은지 다시 측정했다. [새 관찰 JSON](./chrome155-direct-text-observation.json)과 [화면](./chrome155-label-removed.png)은 이 변형만 기록하며 원 fixture와 원 관찰 파일은 바꾸지 않았다. 확인 뒤 `agent-browser close`를 실행했고 `pgrep -x agent-browser` 결과는 비어 있었다.
- `Ahem.ttf` SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다. 이 파일은 pinned C15 resource이며 system font fallback을 oracle이나 구현에서 사용하지 않는다.

## 고정 수치와 판정

| 관찰 | Chrome 155 | HostDocument/layout/runtime acceptance |
| --- | --- | --- |
| Block border box | `15×10 CSS px` | 동일 frame와 auto-height |
| text range | parent-relative `[0,0,20,10]` | 한 줄, advance `20px`, text ink가 owner 폭 밖으로 `5px` 이어짐 |
| source | DOM text `XX` | 실제 HostDocument text node와 UTF-16 range `[0,2]`를 보존 |
| per-character advance | `10px`, `10px` | 고정 Ahem shaping/raster glyph origin이 각 `10px`씩 이동 |
| paint sample | Chrome screenshot은 시각 참고만 함 | `#092170` text의 opaque sample `[9,33,112,255]`과 고정 stage background `[247,249,252,255]`를 Android PixelCopy/iOS Metal drawable에서 읽음 |

Geometry는 CSS px `0.05px` 이내로 비교한다. Chrome screenshot 전체의 antialias byte parity는 판정하지 않는다. 플랫폼 표면의 opaque Ahem sample은 실제 앱 readback 값으로 서로 따로 검증한다. Pixel sample은 glyph raster의 alpha coverage가 255인 좌표에서 layout origin으로부터 계산하며, 테스트 코드가 비교 좌표를 임의로 이동할 수 없도록 origin/source report와 함께 보존한다. 두 번째 `X` sample은 paragraph 오른쪽 경계 이상이면서 viewport 안쪽이어야 하며, 고정 절대 좌표에 의존하지 않는다.

## 이번 slice에서 다루지 않는 관찰

이 baseline은 CSS font cascade, style-to-layout의 일반 Text admission, Flex/Grid intrinsic sizing, event dispatch, font fallback/async load, DOM `Range` API, 전체 WPT 또는 실기기 paint를 입증하지 않는다. 계획된 구현은 fixture-owned HostDocument snapshot에서 나온 C15.1 `LayoutOutput.text_layouts`가 runtime paint input이 되는 한정 경로만 연결한다. live V8 DOM 입력은 이 slice에 포함되지 않는다. C15/C15.3의 공식 완료 상태는 변경하지 않는다.

새 관찰 JSON과 screenshot의 SHA-256은 [manifest](./sha256.txt)에 있다. manifest는 자기 자신을 제외한다.
