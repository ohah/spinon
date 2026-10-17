# C15.3 pure Rust Unicode shaping slice

## 상태와 경계

이 자료는 C15.3의 첫 구현 slice인 순수 Rust shaping snapshot 근거다. 고정 글꼴로 Unicode glyph/run/cluster/source mapping을 만들고 확인한다. WGPU glyph paint, 앱 runtime text admission, Block/Flex/Grid intrinsic sizing, overflow/scroll, paint order, clip, hit-test는 구현하지 않았다. Android·iOS Simulator의 Spinon text ink도 관찰하지 않았다. 따라서 C15.3과 상위 C15/C16/C19 상태는 미완료다.

Chrome 155 oracle와 그 screenshot은 [사전 비교 근거](../c15-3-unicode-shaping-precomparison-2026-10-17/README.md)에 있다. 그것은 기준 브라우저 화면이지 Spinon 구현 화면이 아니다. 이 PR에는 구현 화면을 나타내는 screenshot이 없다.

## 구현한 경계

- paragraph base direction `Auto`/`Ltr`/`Rtl`, line break `Normal`/`Strict`, OpenType ligature `Normal`/`None`을 요청하고 snapshot에 보존한다.
- visual glyph stream과 Parley positioned glyph stream의 길이·ID 순서를 대조한다. glyph source는 normalized UTF-8 grapheme/ligature cluster와 원본 DOM UTF-16 범위 및 text-node 소유자를 함께 보존한다.
- glyph ID 0은 `.notdef`로 보고 고정 paragraph와 UTF-8 범위를 포함한 `MissingGlyphCoverage`로 닫는다. C15.3 fixture는 system font discovery를 끄고 테스트마다 명시 font 하나만 등록한다.
- 빈 텍스트 snapshot에도 요청 shaping mode를 유지한다. 이 경로는 font/style validation과 source normalization을 건너뛰지 않는다.

## 검증 실행

환경은 macOS 25.5 arm64, `rustc 1.96.1`, `cargo 1.96.1`이다. 근거 파일은 이 디렉터리에 보존했다.

| 명령/검사 | 결과 |
| --- | --- |
| `cargo test --locked -p spinon-text-layout -p spinon-layout` | 통과: `spinon-text-layout` 26/26, `spinon-layout` 154/154. C15.3 전용 9개 fixture test도 모두 통과. 전체 원본 출력은 이 디렉터리의 `cargo-test.log`에 있다. |
| `cargo fmt --all -- --check` | 통과. 원본 출력은 `rustfmt.log`에 있다. |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과. 첫 실행에서 f32 oracle 상수의 과도한 소수 정밀도 두 건을 발견해 표현을 고친 뒤 최종 실행을 통과시켰다. 원본 출력은 `clippy-final.log`와 `clippy-initial-failure.log`에 있다. |
| `cargo build --locked -p spinon-text-layout` | 통과. 원본 출력은 `product-build.log`에 있다. |
| 기본 `.rlib` 대 C15.3 test binary fixture-byte positive/negative control | Noto Sans KR, Noto Sans Arabic, Noto Emoji, Ahem 원본 바이트가 기본 `.rlib`에서는 모두 없고 test binary에는 모두 있다. 검사 결과는 `fixture-asset-boundary.log`에 있다. |
| pinned fixture font SHA-256 | README에 기록된 SHA-256과 세 파일이 일치한다. |
| `mise exec -- bun run docs:build` | 실패: 저장소의 473개 Markdown source가 기존 dead link로 보고됐다. 이 C15.3 README는 실패 목록에 없으며, `STATUS.md`와 내부 색인은 이 변경과 무관한 기존 경로를 보고한다. 세부 사항은 [`docs-build-result.md`](docs-build-result.md)에 있다. |

## C15.3 고정 입력 결과

1. Noto Sans KR `한글 테스트`: 6개 nonzero glyph, advance 약 `96.47017 CSS px`, line height `24 CSS px`.
2. Noto Sans KR `甲乙、丙丁`, strict, 폭 `40 CSS px`: `甲` / `乙、` / `丙丁`로 나뉘고 닫는 U+3001가 다음 줄 첫 glyph가 되지 않는다.
3. Roboto `x` + U+0323: 하나의 `0..3` UTF-8 cluster와 UTF-16 `0..2` source range를 보존한다.
4. Roboto `office`: ligature on/off advance `49.5625`/`51.25 CSS px`; `ffi` glyph는 UTF-8·UTF-16 source `1..4`를 보존한다.
5. Noto Sans Arabic `السَّلَامُ 123`: RTL Arabic run과 LTR European digit run을 구분하고 digit의 visual x는 증가한다.
6. 같은 Arabic script 단독 문단에서 Auto/RTL 시작 x는 일치하고 명시 LTR은 x=0으로 시작한다. 비어 있지 않은 `line-break:normal` 입력도 실행한다.
7. Noto Emoji `👩‍💻`: nonzero glyph 하나가 UTF-8 `0..11`, UTF-16 `0..5` cluster/source를 소유한다.
8. base `x`와 U+0323을 서로 다른 DOM text node로 나누어도 두 source owner 범위가 cluster에 남는다.
9. Ahem에 없는 otter glyph는 system fallback이나 `.notdef`로 대체되지 않고 `MissingGlyphCoverage`가 된다.

Noto Sans KR fixture에는 Hangul과 이 CJK punctuation case에 필요한 glyph가 있고 hash는 고정돼 있다. locked ICU4X 2.3.0의 Chinese/Japanese segmentation model은 이 환경에 포함되지 않는다. punctuation fixture만 통과한 것이며 한자 어휘 분절이나 일반 CJK line breaking의 근거로 확대하지 않는다.

## 적대 검토

- [구현 실패 경로 20개](implementation-review-20.md)는 source mapping, bidi, line breaking, ligature, fallback, malformed geometry, asset 경계와 runtime 제한을 별도 관점으로 검토한다.
- [PR diff 30개](pr-delta-review-30.md)는 변경된 API·구현·문서·evidence·status 경계를 독립적으로 검토한다.
