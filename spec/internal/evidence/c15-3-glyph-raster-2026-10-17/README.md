# C15.3 고정 글꼴 glyph raster snapshot 근거

**날짜:** 2026-10-17
**범위:** Parley `TextLayoutSnapshot` → 고정 글꼴 Swash glyph bitmap snapshot
**기준 커밋:** `c6b9e91b361e1f0a02150b1c4f4f84a21542aeca`

## 사전 oracle과 판정 조건

이 단계의 oracle는 Chrome screenshot이 아니라 같은 고정 font bytes로 만든 shaped snapshot의 불변 조건이다. Chrome/Stylo의 출력 색 비교나 OS rasterizer와의 pixel parity를 주장하지 않는다.

- 각 input run/glyph의 순서, glyph ID, CSS origin/baseline, UTF-8 cluster와 DOM UTF-16 source mapping이 raster 결과에서 그대로 유지되어야 한다.
- Hangul, strict CJK 줄바꿈, Arabic/Bidi와 digit ordering, emoji ZWJ, combining sequence, ligature에 대해 보이는 non-whitespace glyph에는 dimension과 channel 수가 맞는 bitmap이 있어야 한다. 공백 및 빈 glyph에는 bitmap이 없어야 한다.
- Ahem 10px은 DPR 1/2/3에서 각각 10/20/30 physical pixel glyph를 만들어야 하고 CSS 좌표는 유지되어야 한다. fractional x/y 위치는 정수 anchor와 Swash subpixel phase로 분리되어야 한다.
- foreign/stale registry, 누락/손상 font, 지원하지 않는 face/glyph, invalid scale/size/coordinate/source, 손상 bitmap 및 64 MiB 누적 snapshot 경계는 구체적인 오류로 끝나야 한다.
- 이 게이트가 통과해도 glyph가 renderer에 제출되었다고 보지 않는다. WGPU command, app runtime, visible ink, parent clip, overflow hit-test는 다음 단계다.

## 실행 결과

| 검사 | 결과 |
| --- | --- |
| `cargo test --locked -p spinon-text-layout -p spinon-layout` | `spinon-text-layout` 35 passed, `spinon-layout` 154 passed, 실패 0 |
| `cargo fmt --all -- --check` | 통과 |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| 실제 앱/WGPU/Simulator/화면 pixel | 실행하지 않음: 이 변경에는 text paint route가 없음 |

원본 로그: [Rust tests](rust-tests.log) · [rustfmt](rustfmt.log) · [Clippy](clippy.log). 구현 검토의 20개 독립 실패 관점은 [implementation-review-20](implementation-review-20.md)에 기록했다. PR 생성 뒤의 별도 변경 검토는 [pr-delta-review-20](pr-delta-review-20.md)에 기록한다.

## 고정 글꼴 SHA-256

| fixture | SHA-256 |
| --- | --- |
| `tests/fixtures/resources/c15/Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| `tests/fixtures/resources/c15/Roboto-Variable.ttf` | `d7598e12c5dbef095ff8272cfc55da0250bd07fbdecbac8a530b9b277872a134` |
| `tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf` | `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76` |
| `tests/fixtures/resources/c15/c15-3/NotoSansArabic-VF.ttf` | `63111b5b2e074dd48cc67692e0a2726d86ee94c1c37fe8598257b7b4e87e869e` |
| `tests/fixtures/resources/c15/c15-3/NotoEmoji-VF.ttf` | `de6c18832938afc99caf132b39d6a30a19bac7f2e812e28db2535b4608d27551` |

Dependency is pinned to `swash = 0.2.10` in `Cargo.toml` and `Cargo.lock`. The registration input bytes are owned by the engine so raster does not depend on caller buffer lifetime.

## 적대 점검에서 수정한 결함

- 같은 engine에서 복제된 두 registry가 이후 서로 다른 font를 등록하면, 숫자 revision만으로 snapshot provenance를 판정할 때 충돌할 수 있었다. 성공한 font registration마다 identity를 회전하고, revision이 같은 divergent clone snapshot을 거부하도록 했다.
- 고정 Noto Sans KR의 공백 raster는 Swash에서 `1×0`, empty payload로 나타난다. zero-sized empty output은 ink 없는 glyph로 보존하고, non-empty malformed payload는 거부한다.
- 한 bitmap 최대 16 MiB 외에 paragraph snapshot 전체 64 MiB 누적 제한이 없었다. checked byte accumulator와 overflow negative control을 추가했다.
- test module이 500줄 권장 상한을 넘었고, 새 hidden registry identity가 기존 snapshot `PartialEq`에도 반영될 수 있었다. shaping/raster tests를 분리하고, raster provenance 검사는 identity를 사용하되 snapshot 값 동등성은 기존 공개 필드만 비교하도록 수정했다.

## 남은 검증 경계

- 16 MiB 단일 glyph 크기 검사는 Swash가 image를 만든 다음 수행한다. 악성 font가 Swash 내부에서 유도할 수 있는 temporary allocation 상한은 이 slice에서 증명하지 않았다.
- Noto Emoji fixture는 문서상 단색 outline이다. 실제 COLR palette, embedded color bitmap, color RGBA alpha convention 및 compositor 결과는 검사하지 않았다.
- 단일-face font 경로만 허용한다. 실제 TTC/OTC fixture로 다중 face 거부를 검증하지 않았다.
- font fallback, font variation axis selection, WGPU upload/cache, color management, alpha compositing, text paint order, ancestor overflow clipping/hit-test, Android/iOS Simulator frame ink는 연결되지 않았다. C15/C15.3은 계속 미완료다.
