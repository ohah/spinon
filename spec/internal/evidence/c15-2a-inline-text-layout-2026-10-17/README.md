# C15.2a 중첩 inline 텍스트 geometry 실행 근거

이 slice는 고정 폰트와 definite width를 받는 `spinon-text-layout`/`spinon-layout` 내부 geometry 계산이다. HostDocument의 최대 두 단계 HTML `span`, 고정 px border/padding, source ownership, line별 fragment를 검증한다. 공식 상태에서 C15 parent는 미완료로 유지한다.

## 사전 oracle과 통과 기준

사전 기준은 [C15.2 Chrome 155 비교 자료](../c15-2-inline-text-precomparison-2026-10-17/README.md)와 [작업 계획](https://github.com/ohah/spinon/blob/main/plan/c15-inline-text-layout.md)의 네 필수 case다. CSS px 수치 허용차는 `0.05px`다.

| Case | Rust 결과 |
| --- | --- |
| `nested-short` | Ahem `10px/10px`, width `50px`; paragraph `50×10`, outer `[0,-3,36,16]`, inner `[3,-5,30,20]`, text `[8,0,20,10]`. |
| `nested-wrap` | width `30px`, Ahem `XX XX`; line box `30×20`; text `[4,0,20,10]`, `[0,10,20,10]`; outer/inner line별 rect가 Chrome oracle과 일치한다. inline start와 end edge는 첫/마지막 줄에 각각 포함된다. |
| `ahem-baseline` | Ahem `10px/10px`; line height `10px`, baseline은 line top에서 `8px`다. |
| `roboto-metrics` | Roboto `20px/24px`, 두 `Hgyp` source range와 1px baseline anchor; paragraph width `93.4063px`, height `24px`, baseline `19px`; text range advance는 각각 `46.2031px`, anchor는 `[46.2031,19,1,0]`이다. Anchor는 Chrome의 baseline 위치를 측정하는 `cfg(test)` 전용 입력이다. 제품 `inline-block`/replaced-object 지원을 추가하지 않는다. |

![Chrome 155 C15.2 oracle 화면](../c15-2-inline-text-precomparison-2026-10-17/chrome155.png)

위 이미지는 Chrome 기준 fixture 화면이며 Spinon 앱 출력이 아니다. 현재 text projection/renderer는 runtime text를 거부한다.

## 실행 결과

- `cargo test --locked -p spinon-text-layout` (`text-layout-tests.log`): **17 passed**, 0 failed.
- `cargo test --locked -p spinon-layout c15` (`c15-layout-focused-tests.log`): **9 passed**, 0 failed.
- `cargo test --locked -p spinon-layout` (`spinon-layout-tests.log`): **147 passed**, 0 failed.
- `cargo test --locked --workspace` (`workspace-tests.log`): **947 passed**, 0 failed, 4 ignored.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` (`clippy.log`): 통과.
- `cargo fmt --all -- --check` (`rustfmt.log`) 및 `git diff --check`: 통과.
- `aarch64-linux-android` (`android-target-check.log`), `aarch64-apple-ios-sim` (`ios-simulator-target-check.log`)에서 `spinon-text-layout`과 `spinon-layout`의 `cargo check --locked` 통과. 이는 Android/iOS Simulator **target compile**이며 app build, simulator 실행, 화면·glyph·pixel 검증이 아니다.
- `bun run docs:build`는 기존 Markdown 472개 source의 dead link로 실패했다. 변경한 C15 page/evidence에서 dead link는 없었다. 간결한 결과는 `docs-build.log`, 전체 console 출력은 `docs-build.full.log.gz`에 보존했다.
- 변경된 C15 route 9개만 모은 Rspress isolated build는 `markdown.link.checkDeadLinks=false`, `markdown.image.checkDeadImages=false` 설정으로 HTML 생성을 통과했다. 로그는 `scoped-docs-build.log`다. 이 범위 build는 전체 docs dead-link 검사를 대신하지 않는다.

## 적대 경로에서 찾은 결함과 수정

- Public `ParagraphInput`이 owner 경로 `[A] → [] → [A]`를 허용해 중간의 무소유 text까지 A의 min/max span으로 감싸는 결함을 재현했다. 닫힌 owner의 재진입을 `InvalidInlineHierarchy`로 거부하고 회귀 테스트를 추가했다.
- 각 중첩 style의 edge는 유한해도 합계가 overflow할 때 Parley가 `usize::MAX..0`인 line range 두 개를 만들고 성공 snapshot에 넣는 입력을 재현했다. line/run/cluster range의 순서, 문자열 경계와 UTF-8 문자 경계를 검사해 `InvalidGlyphLayout`로 닫는다.
- 기존 Roboto test가 텍스트 폭만 계산해 계획의 1px baseline anchor를 포함한 parent width보다 1px 작게 확인하던 차이를 찾았다. test-only inline anchor를 넣어 사전 oracle의 `93.4063px` parent geometry와 `19px` baseline을 검증한다.

구현 실패 경로 20개는 [별도 기록](./implementation-review-20.md), PR 변경 검토 30개는 [별도 기록](./pr-delta-review-30.md)이다.

## 지원 밖 범위

- runtime cascade/style projection, app text node admission, glyph raster/painter, overflow/clip pixels, hit testing, selection, accessibility.
- Android app/AVD 화면, iOS Simulator app 화면, 실기기와 hardware GPU.
- mixed-font baseline, anonymous block flow, inline margin, non-baseline `vertical-align`, replaced/inline-block content, 세 단계 이상 nesting, bidi와 vertical writing.
- 전체 CSS text 적합성 또는 C15/C16 완료.

공식 상태는 [C15 미완료 기록](../../../STATUS.md#css-구현-체크리스트), 내부 호출 계약은 [0070](../../0070-c15-1-inline-text-layout.md)에서 관리한다.
