# C11.3.3 unknown named-area implicit lines 계획 실패 관점 20개

**검토 시점:** 2026-10-20, product Rust 변경 전.
**검토 대상:** [계획](../../../plan/c11-3-3-unknown-named-area-implicit-lines.md), pinned WPT source, Chrome 154 사전 비교, Agent Browser overflow 확인.
**근거:** [precomparison evidence](./c11-3-3-unknown-named-area-precomparison-2026-10-20/README.md) · [fixed Chrome JSON](../../../tests/fixtures/css/references/c11-3-3-unknown-named-area-implicit-lines-precomparison-v1.json).

각 항목은 서로 다른 실패 경계다. 이 문서는 계획 검토이며 구현이나 platform 결과를 통과로 세지 않는다. 전체 20개가 PASS여서 product implementation을 시작할 계획 기준을 고정했다.

| # | 실패 질문 | 계획의 방어·근거 | 결과 |
|---:|---|---|---|
| 1 | explicit 끝 line과 첫 implicit line의 번호를 혼동해 track 하나를 생략하는가 | §8.3 positive occurrence와 explicit 마지막 경계를 분리한다. Chrome resolved track 및 frame에서 첫 implicit track 앞의 line4를 재현했다. | PASS |
| 2 | 양쪽 bare ident가 같은 line일 때 conflict handling 뒤 기본 span을 잘못 계산하는가 | `unknown-both-named-row` Chrome rect와 두 implicit row를 기준으로 고정했다. | PASS |
| 3 | row/column axis swap으로 named line이 다른 track list에 적용되는가 | 네 WPT case 중 row 두 개·column 두 개를 분리하고 CSSOM 각 축을 기록했다. | PASS |
| 4 | explicit authored name이 missing-name fallback에 가려지거나 C11.3.2 wrapper 동작이 넓어지는가 | explicit line-name lookup을 우선하고 C11.3.3 전용 opt-in wrapper만 fallback을 허용한다. 기존 C11.3.2 wrapper의 missing-name rejection도 구현 test로 고정한다. | PASS |
| 5 | area edge alias가 이름 부재 fallback보다 늦게 선택되는가 | 기존 C11.3.3 area edge precedence regression을 계속 관문에 둔다. | PASS |
| 6 | implicit `foo-start`/`foo-end` pair가 unknown name으로 오인되는가 | 기존 implicit area alias rule이 fallback 전에 적용되며, 계획은 이 순서를 바꾸지 않는다. | PASS |
| 7 | area matrix/explicit track count 오류가 implicit generation으로 유효화되는가 | mismatch는 기존 pre-layout rejection을 유지하고 계획 밖 입력으로 고정했다. | PASS |
| 8 | named span의 endward shortage와 leading 방향 implicit generation을 혼동하는가 | WPT endward row/column span은 포함하고 backward/leading generation은 기대 거부로 분리했다. | PASS |
| 9 | 일부 explicit named match 후 남은 span 수를 중복 계산하는가 | 별도 Rust projection case에서 explicit match 수와 생성할 trailing line 수를 나눠 검사하도록 계획했다. | PASS |
| 10 | `span 0`, reserved `auto`/`span` token이 fallback을 우회하는가 | CSS parser의 invalid/cascade 경계를 유지하며 direct invalid typed value는 오류 검증한다. | PASS |
| 11 | bare name만 지원하며 numbered occurrence shortage를 몰래 확장하는가 | `<integer> <custom-ident>` missing occurrence는 계획 범위 밖이며 기존 expected rejection 유지다. | PASS |
| 12 | line/span arithmetic overflow가 wrap 또는 panic을 일으키는가 | checked `i32` 변환, Taffy line range, track extent 경계 및 overflow rejection을 구현 검증에 요구한다. | PASS |
| 13 | generated track cap 초과 allocation이 먼저 발생하는가 | `GRID_MAX_TRACKS`를 cap으로 유지하고 allocation·normalization 실패 원자성을 확인한다. | PASS |
| 14 | 기본 `grid-auto-*:auto`가 우연히 intrinsic sizing 지원으로 바뀌는가 | 자동 implicit size 입력은 계속 거부; 새 successful fixtures는 fixed px만 쓴다. | PASS |
| 15 | negative/leading placement가 positive trailing subset으로 오해되는가 | negative named line/backward span은 별도 expected rejection이다. | PASS |
| 16 | 같은 missing-name 결과가 양축 중 한 축만 확장하는가 | row/column WPT case 및 per-axis CSSOM track counts를 비교한다. | PASS |
| 17 | new placement mode가 template-area sibling의 크기·placement를 바꾸는가 | `TemplateGridImplicitTracks`는 TemplateGrid allowlist를 보존하며 mixed `known` sibling scene을 포함한다. | PASS |
| 18 | hidden/nested descendants가 implicit extent 또는 sibling identity에 섞이는가 | 기존 C11.3.3 nested/hidden matrix를 regression gate로 유지한다. | PASS |
| 19 | 다른 item의 invalid placement 뒤 partial geometry/draw가 반환되는가 | layout output/frame/draw 부재 및 error node/property atomicity를 구현·runtime에서 확인한다. | PASS |
| 20 | root 경계 밖 item이 DOM rect만 있고 실제 surface에서는 잘리거나 검증 과정이 열린 채 남는가 | Chrome baseline은 item rect `(175,125,15,5)`가 root `(0,0,160,120)` 밖임을 기록했다. 내부/root 밖/중간 pixel 3점을 Android/iOS simulator에서 비교한다. Agent Browser screenshot·세션 종료를 확인했다. | PASS |

**판정:** 계획 review 20/20 PASS. Chrome 기준은 WPT 네 좌표와 일치하고 파생 area scene·overflow sample이 별도 구분되어 있다. 다음 단계는 계획에 적힌 별도 20 implementation failure review다.

## 구현 전 계획과 실제 surface 비교의 차이

구현 후 최초 네 번째 표본 `(170,127)`에서 Chrome은 uncovered viewport를 흰색으로 그리고 Android/iOS native surface는 host-owned 배경을 검게 그리는 차이를 확인했다. 이는 Grid root나 overflow item의 clipping 차이가 아니므로 최종 C11.3.3 mobile pixel fixture는 grid interior `(150,110)`, known-area sibling `(10,10)`, overflow item `(182,127)` 세 표본만 exact 비교하고, Chrome의 white page-backdrop 표본은 `excludedReferenceSamples`에 이유와 함께 남긴다. Android PixelCopy와 iOS Metal drawable은 현재 모두 3/3 exact이며 OS compositor 이후 결과는 아니다. plan의 기준 설명도 구현 결과에 맞게 갱신했다. 기존 20개 항목은 구현 전 계획만 검토한 기록으로 유지한다.
