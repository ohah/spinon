# C15 계획 적대 검토 20개

**검토 대상:** [`plan/c15-inline-text-layout.md`](../../../plan/c15-inline-text-layout.md), C15 Chrome precomparison fixture/evidence<br>
**검토 시점:** 구현 전 · 2026-10-17<br>
**판정:** P1/P2 미해결 없음. Chrome executable SHA/CDP protocol revision 부재는 구현 시작 전 gate로 명시했다. C15/C16/C19 status는 미완료로 유지한다.

| # | 독립 실패 관점 | 적대 확인과 판정 |
| ---: | --- | --- |
| 1 | 로드맵을 계획만으로 완료 처리 | `spec/STATUS.md` C15/C16 모두 `[ ]`이고 C15 행에 계획·사전 비교 링크만 붙는다. 완료 판정은 C15.1–C15.3과 최종 통합까지 남긴다. 통과. |
| 2 | C15/C16 경계에서 `line-height`가 두 번 지원된 것으로 오인 | 초안의 표현을 고쳐 C15는 oracle이 지정한 내부 line-height scalar만 받고 CSS property parsing/cascade는 C16으로 명시했다. C15.1 fixed-font profile, C16 font cascade·`@font-face`·fallback/loading도 분리했다. 통과. |
| 3 | synthetic glyph width가 실제 shaping으로 오인 | 같은 code-unit 수의 Roboto `iiii`와 `WWWW`가 19.4531px/70.9844px다. 문자 수에 고정 폭을 곱하는 measure를 명시적인 negative control로 고정했다. 통과. |
| 4 | Ahem fixture가 실제 typographic font 전체를 대표 | Ahem은 baseline/line-box 대조용에 한정하고 Ahem만으로 shaping을 증명하지 않는다고 README와 계획에 기록했다. proportional Roboto는 별도 fixture다. 통과. |
| 5 | Ahem 파일 출처 또는 저작권 누락 | WPT commit `de86b2b3...`의 원본과 저장 font의 SHA-256을 비교했고 byte-identical이다. Ahem project가 public domain이라고 명시한 출처를 연결했다. 통과. |
| 6 | Roboto 변형 파일 또는 OFL이 서로 다른 upstream 상태 | Google Fonts commit `47d54ca1...`, 원본 Roboto revision, `Roboto-OFL.txt`를 저장하고 세 파일 SHA를 고정했다. 통과. |
| 7 | Chrome 실행 환경이 버전 변화를 허용 | Chrome high-entropy full version `155.0.8059.85`, macOS `26.5.1` ARM, locale, viewport, DPR, 두 폰트 load 성공을 JSON에 보존했다. executable SHA와 CDP revision은 없는 것을 감추지 않고 코드 착수 gate로 남겼다. 통과. |
| 8 | Chrome 출력 측정을 Spinon 구현값으로 자기 검증 | fixture는 Agent Browser의 Chromium DOM `Range`/element rect와 screenshot만 읽는다. Spinon runtime은 실행하지 않았고 spec/status에 그 차이를 적었다. 통과. |
| 9 | 측정 전 font load race로 system fallback 사용 | Agent Browser는 `document.fonts.check()`가 Ahem과 Roboto 모두 true인 뒤 observation을 만들었다. JSON에도 두 load state를 보존했다. 통과. |
| 10 | viewport 설정 이후 observation metadata stale | 마지막 observation은 viewport를 `320×480`으로 설정한 뒤 `observe()`를 다시 호출했고, JSON viewport와 screenshot 조건이 일치한다. 통과. |
| 11 | fixture sibling layout이 결과 높이를 오염 | 초기 2-column grid 실험에서 auto track이 sibling 높이를 공유했다. 최종 fixture는 독립 block row로 바꾸었고 각 element의 line/height가 분리됐다. 통과. |
| 12 | collapsed whitespace 결과가 code unit 소비량에 잘못 의존 | `collapsed-spaces`는 raw text가 X+세 공백+X이며, 세 공백 중 두 rect가 zero advance임을 JSON으로 보존했다. 통과. |
| 13 | word wrap와 unbreakable overflow가 같은 성공값으로 뭉개짐 | 폭 30px `XX XX`의 두 line과 폭 15px `XX`의 한 line/20px text overflow를 별도 ID·기대로 둔다. 통과. |
| 14 | `break-all` 동작이 default word-break 테스트와 혼동 | 폭 20px `XXXXX`에 20/20/10px line matrix를 따로 고정하고 default `long-word`와 대조한다. 통과. |
| 15 | newline이 공백 collapse와 동일하게 처리 | `pre-line`의 실제 U+000A와 zero-width newline rect, 두 line box를 따로 기록한다. `white-space:normal` case와 입력이 분리돼 있다. 통과. |
| 16 | inline element 하나의 union rect로 line fragmentation 은닉 | border 1px span의 `getClientRects()`를 조회하고 두 rect 각각 `21×12px`를 보존한다. total union bbox만 비교하지 않는다. 통과. |
| 17 | baseline을 line top/font size로 추정 | Ahem font의 baseline marker와 10px line을 함께 기록하고 offset 8px를 사전 판정으로 삼았다. baseline은 후속 font metrics 검사에서도 다시 확인한다. 통과. |
| 18 | Taffy flex-wrap을 inline formatting 대용으로 채택 | plan은 Taffy 0.14.0의 Spinon custom `CalcLayoutTree` 경로를 확인하고, downstream inline adapter와 definite available-size를 연동한다. `flex-wrap` simulation을 제외했다. 통과. |
| 19 | 일부 inline tree를 조용히 성공시켜 box와 paragraph가 분리 | mixed block/inline anonymous block splitting, unsupported whitespace/profile, stale font/width revision을 fail closed 대상으로 고정했다. 각 topology는 지원 전에 별도 fixture가 필요하다. 통과. |
| 20 | layout 결과만 만들고 invisible text를 완료로 보고 | 계획은 shaped output과 glyph paint를 분리하고 WGPU glyph command 및 Android PixelCopy/iOS Metal capture 전에는 C15 완료를 허용하지 않는다. pixel antialias 차이와 layout geometry tolerance도 구별했다. 통과. |

## 해결 후 남은 구현 전 gate

- C15 code PR 전에 WPT commit/path를 선택하고 고정한다.
- Chrome full version은 보존했지만 executable binary SHA와 CDP protocol revision이 없다. 수집을 시도하고, unavailable이면 manifest·PR에 제한을 기록한다.
- Parley 0.12.0은 최근 release다. lockfile pin, high-level API compile, line-break reshaping behavior, Ahem/Roboto font ownership을 실제 잠금 source로 재확인한다.
- Android API 37 AVD와 iOS 26.2 Simulator 결과가 나오기 전에는 Spinon text layout/paint가 통과했다고 말하지 않는다.
