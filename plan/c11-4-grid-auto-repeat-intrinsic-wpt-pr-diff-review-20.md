# C11.4 `grid-auto-repeat-intrinsic-001.html` 계획 PR diff 적대 검토 20개

계획과 사전 비교 diff를 대상으로 링크·출처·기대값·공식 상태·증거 경계가 실제 변경 파일끼리 모순되지 않는지 검토했다. 이 표는 집중 계획의 선행 review 20개와 다른 PR 변경 관점이다.

| # | 변경 diff 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 상위 C11.4 계획 표에서 집중 계획 경로가 틀린다. | `plan/c11-4-grid-auto-repeat-intrinsic-wpt.md` 상대 링크가 실제 새 문서로 연결된다. PASS |
| 2 | STATUS의 계획-only 항목이 구현 완료처럼 읽힌다. | 문구는 계획·Chrome 관찰만 보고하고 C11.4 unchecked 및 `72/213` 유지가 명시돼 있다. PASS |
| 3 | STATUS, plan, precomparison 이름이 서로 다른 WPT를 가리킨다. | 정확한 `grid-auto-repeat-intrinsic-001.html` 파일명과 hash가 세 파일에 일치한다. PASS |
| 4 | manifest revision이 로컬 WPT checkout revision과 다르다. | manifest revision과 `build/wpt-c124-source` HEAD가 `c999c58338ee1d223df5ad62e48be1202ced0d37`로 일치한다. PASS |
| 5 | helper CSS/script의 hash가 현재 WPT source에서 재현되지 않는다. | WPT와 5개 direct helper의 실제 SHA-256 여섯 개를 source root에서 다시 대조했다. PASS |
| 6 | evidence README가 Chrome의 결과를 Spinon 통과로 표현한다. | README는 Chrome 기준 결과로 한정하고 Spinon·Simulator 미검증을 구분한다. PASS |
| 7 | screenshot이 testharness 결과를 포함하지 않거나 case를 일부 숨긴다. | 1280×1026 전체 화면에 여덟 test 행과 8 Pass, Grid 영역이 보인다. PASS |
| 8 | browser metadata가 관찰과 다른 Chrome/DPR/viewport를 기록한다. | Agent Browser observation JSON의 UA, DPR 1, 1280×633 viewport 및 harness text와 manifest 값이 일치한다. PASS |
| 9 | observation JSON이 CLI 출력의 이중 JSON string wrapping 상태로 커밋된다. | 파일을 jq로 디코딩했고 JSON object로 8개 row를 검증했다. PASS |
| 10 | `expected.json`의 root dimensions가 browser row order와 어긋난다. | 8개 root의 expected width/height와 Chrome 관측 border-box 크기를 순서대로 대조했다. PASS |
| 11 | expected track list와 repeatCount가 서로 다르다. | 모든 행에서 `usedColumnsPx`가 Chrome의 resolved track sequence와 맞고, count는 suffix를 뺀 repeated track 수와 같다. PASS |
| 12 | item/probe expected frames가 WPT DOM child order와 다르다. | 각 root의 첫 child와 뒤 네 probe frame을 Chrome observation JSON에 대조해 8/8 전부 일치했다. PASS |
| 13 | W3C excerpt URL이 다른 Grid spec revision을 가리킨다. | 저장된 excerpt source URL은 2025-03-26 CSS Grid 2025 CRD다. PASS |
| 14 | 계획의 out-of-range absolute line 결과가 expected JSON과 반대다. | 두 min-content/auto-tail case 모두 `[10,60]` tracks와 probe `(70,0)`, `(0,70)`이 양 문서에 같다. PASS |
| 15 | overflow 수치가 content width와 border-box width를 혼합한다. | item 구간 `[2,102]`, root border-box 구간 `[0,74]`의 차이 28px을 기대값·계획·README에 맞췄다. PASS |
| 16 | simulator surface negative point가 item 안이나 root border 위에 있다. | root-relative x=84는 border 밖 item 안, x=103은 item 끝 뒤, x=73은 right-border control이다. PASS |
| 17 | new scope가 기존 PR #683의 다른 intrinsic-size-change WPT와 중복된다. | 계획과 source path는 sibling Grid의 auto-fill/auto-fit matrix만 다루고 기존 same-document mutation task를 재사용하지 않는다. PASS |
| 18 | feature-only fixture 계획이 일반 CSS admission/API 범위를 확대한다. | 계획은 opt-in exact matcher, unknown/unsupported fail-closed, public API 미추가를 명시한다. PASS |
| 19 | 체크인되는 파일 목록에 기존 untracked simulator logs나 screenshot이 섞인다. | staging은 plan, STATUS, 상위 계획, 새 precomparison 폴더만 경로 지정하고 기존 산출물은 건드리지 않는다. PASS |
| 20 | plan review 결과를 PR diff review로 재사용하거나 구현 review로 세어 버린다. | 이 파일은 별도 20행 변경 검토이며 계획 선행 20행과 구현 후 검토 기준을 구분한다. PASS |

**계획 PR diff 검토:** PASS 20/20, 미해결 0. WPT/Chrome는 baseline, 이 검토는 문서 변경 검토이며 구현 검증이 아니다.
