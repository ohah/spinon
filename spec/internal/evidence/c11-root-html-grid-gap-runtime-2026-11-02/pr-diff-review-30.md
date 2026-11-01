# PR #1002 diff 적대 검토 30개

**범위:** PR #1002의 plan/precomparison, 구현, 테스트, 내부 계약, 플랫폼 evidence와 PR 본문 screenshot 링크를 검토했다. 첫 review 대상은 현재 34개 파일 diff였다. review 기록을 추가한 뒤 PR diff는 35개 파일이 되며, 마지막 항목에서 이 문서 자체의 범위·링크·개수도 따로 확인한다. 병렬 reviewer를 사용하지 않고 primary agent가 순차적으로 각기 다른 실패 관점을 대조했다.

| # | 공격·실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | C11의 더 넓은 지원 범위가 이 PR에서 우연히 완료로 바뀐다. | `spec/STATUS.md`를 수정하지 않았고 계약·계획은 C11 미완료와 공식 `72/213`을 명시한다. 통과. |
| 2 | 구현 전에 Chrome 결과를 보고 기대값을 맞춰 독립 oracle이 사라진다. | 계획은 130×65 root와 track/gap 합산을 먼저 고정하고 Chrome 관측을 교차 확인으로 분리한다. verifier는 Chrome 156 버전도 검사한다. 통과. |
| 3 | precomparison fixture geometry가 계획과 fixture 사이에서 달라진다. | HTML fixture와 `expected.json`의 40/80 columns, 30/30 rows, 5/10 gaps 및 네 frame을 일치 대조했다. 통과. |
| 4 | screenshot이나 브라우저 결과가 독립 산술 기대값을 대체한다. | README와 계획에서 산술값을 별도로 제시하고 Chrome/Spinon 화면은 교차 관측으로만 분류한다. 통과. |
| 5 | gap property admission이 unrelated author CSS 또는 다른 computed profile에 퍼진다. | 새 allowlist는 Root HTML text-paint profile에만 있고 diagnostic recovery도 `RuntimeRootHtmlTextPaintV1`에서만 실행된다. 통과. |
| 6 | shorthand의 row/column 순서가 뒤집히거나 typed `LonghandId`가 잘못 연결된다. | `RowGap`/`ColumnGap` mapping과 typed snapshot 값 5/10을 item frame과 함께 확인한다. 통과. |
| 7 | 진단 문자열의 prefix/suffix가 느슨해 임의 parser error가 무시된다. | helper는 Stylo의 `OtherInvalidValue` declaration 진단만 받아들이고 다른 error kind는 거부한다. unit controls 통과. |
| 8 | 양수, 단위 외 값 또는 3-token declaration이 negative fixed-px 예외로 오인된다. | classifier는 1–2개 유한 px token과 하나 이상의 음수 조건을 요구한다. positive px·`em`·`calc()`·malformed controls 통과. |
| 9 | CSS의 대소문자 규칙과 `!important` 변형이 parser recovery를 우회한다. | property, `px`, `!important` ASCII 대소문자와 comment-split important unit cases를 추가했고 integration negative shorthand도 실행했다. 통과. |
| 10 | 유효하지 않은 shorthand의 두 번째 양수 token이 별도 column gap으로 누출된다. | 전체 invalid declaration을 버린 뒤 두 번째 item x=40px인 zero-gap 결과를 assert한다. 통과. |
| 11 | invalid inline declaration이 정상 author stylesheet의 gap을 함께 없앤다. | inline negative control은 stylesheet shorthand geometry를 유지하고 cascade diagnostic을 남기지 않는다. 통과. |
| 12 | CSS comment가 들어간 invalid declaration이 cascade 전체를 실패시킨다. | `gap: -1px/**/ 10px` 실패를 실제로 재현했고 comment 제거 matcher와 runtime regression으로 수정했다. 통과. |
| 13 | ignore 예외가 Root HTML block paint나 fixture Grid profile의 오류까지 숨긴다. | stylesheet·inline filter 조건에 exact profile equality가 있고 다른 style diagnostics는 변경되지 않는다. 코드 경계 통과. |
| 14 | CSS-invalid value 하나가 문서의 다른 지원 외 parser errors도 덮는다. | unrelated color/display parser diagnostic controls는 여전히 fail-closed cascade를 만든다. 통과. |
| 15 | percent gap이 viewport로 임의 계산되어 fixed-px profile에 섞인다. | `column-gap:10%`는 해당 property 이름의 `unsupported_computed_value`와 no-partial-scene으로 거부된다. 통과. |
| 16 | `calc()` 또는 hidden math backing이 이미 px로 보이는 computed value를 우회한다. | `row-gap:calc(1px + 1%)`와 `layout_math_values` gate가 projection 전에 막는다. 통과. |
| 17 | negative·NaN·infinity computed px가 Taffy coordinates를 손상한다. | predicate 직접 단위 검사에서 셋 모두 거부된다. 통과. |
| 18 | 기존 `normal`·0px 동작이 바뀌거나 0을 유효한 gap으로 취급하지 않는다. | predicate에서 Normal/0px을 허용하고 zero-gap fixture의 네 frame을 비교한다. 통과. |
| 19 | gap을 Taffy에 중복 적용하거나 root 바깥 frame에 잘못 더한다. | `LayoutGap` typed 전달 위치를 한 번 확인했고, root `[0,0,130,65]`와 item frame이 내부 gutter만 포함한다. 통과. |
| 20 | element ID, class, node ID, 숨김 prefix를 fixture에서 고정 선택한다. | selector/class identity를 바꾸고 hidden prefix를 추가해 semantic paint frame을 비교한다. 통과. |
| 21 | default row auto-placement나 source order가 gap 추가로 달라진다. | 네 item frame의 row-major source order를 정확 비교한다. 통과. |
| 22 | 지원 밖 Grid semantics가 fixed gap 경로에서 조용히 활성화된다. | repeat, percentage/fr track, explicit placement, named area/line, implicit row, non-default flow, nested Grid 등 예상 거부 matrix와 no-scene 결과를 검토했다. 통과. |
| 23 | docs에만 gap이 있고 Root HTML 기본 제품 entry는 계속 zero-only다. | 실제 session integration test가 `new_runtime_gpu_with_root_html()`을 호출하며 Android/iOS 앱은 `index.html` route를 사용한다. 통과. |
| 24 | test-only FFI, payload 또는 app feature flag가 성공 화면을 만들어 낸다. | Android C04 fixture-test와 iOS C04/C11 fixture flags는 off이고 platform JSON/build logs가 이를 기록한다. 통과. |
| 25 | Android runtime evidence가 실기기/hardware GPU 증거로 과장된다. | observation과 README가 API 37 ARM64 SwiftShader AVD로 한정한다. Samsung은 연결돼 있었으나 테스트에 사용하지 않았다. 통과. |
| 26 | Android의 `monkey` 시작 실패가 제품 실패 또는 성공 로그로 혼동된다. | no-physical-keys `-5` 시도는 별도 log로 남기고 final pass03는 명시적 `MainActivity` launch log를 사용한다. 통과. |
| 27 | iOS build log와 screenshot이 서로 다른 simulator나 app bundle에서 왔다. | UDID, iPhone 17 Pro/iOS 26.2, build/runtime log와 screenshot SHA를 platform observation에서 연결한다. 통과. |
| 28 | screenshots가 GPU surface readback·pixel equality를 증명한다고 본문이 암시한다. | PR body, README, JSON이 ADB/simctl display capture 한계를 명시하고 pixel equality를 주장하지 않는다. 통과. |
| 29 | PR 본문의 screenshot raw URL이 틀려 GitHub에서 표시되지 않는다. | Agent Browser에서 PR #1002를 열어 두 `img`의 `complete=true`와 Android 1080×2400, iOS 1206×2622 natural size를 확인했다. 통과. |
| 30 | PR body·contract·review 기록이 서로 다른 scope/count를 말하거나 review 문서가 링크되지 않는다. | PR body는 C11 미완료와 72/213 유지, 20개 구현 review, 남은 범위를 명시한다. 이 matrix 자체의 30개 항목 수와 경로를 재대조했다. 통과. |

## 판정

검토 중 발견한 코드 결함은 없었다. 이전 구현/runtime review에서 발견해 수정한 CSS comment diagnostic 결함은 이 최종 PR diff에 포함되어 재검증됐다. PR checks 조회는 비어 있어 CI 통과로 판단하지 않는다. C11과 공식 완료 `72/213`은 변경하지 않았다.
