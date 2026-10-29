# C11.3.3 template-area auto-track-list cycling · plan review

**검토 대상:** [계획](./c11-3-3-template-area-track-list-cycling.md), [Chrome 156 사전 비교](../spec/internal/evidence/c11-3-3-template-area-track-list-cycle-runtime-precomparison-2026-10-30/README.md), 고정 HTML fixture 및 독립 기대값 JSON.

**시점:** 구현 전. 계획 PR diff 검토는 별도 기록으로 수행하며 이 계획 리뷰를 재사용하지 않는다.

| # | 실패 관점 | 판정 | 계획상 방지 근거 |
| ---: | --- | --- | --- |
| 1 | `grid-auto-*`가 하나의 값만 받는다고 오해해 `<track-size>+` 규범을 놓침 | PASS | §7.6 track-list 문법과 반복 규칙을 고정했다. |
| 2 | area가 만든 첫 implicit track에 목록의 두 번째 값을 적용 | PASS | CSSOM 결과와 독립 기대값이 첫 implicit track을 index 0으로 고정한다. |
| 3 | row 목록과 column 목록이 서로 바뀜 | PASS | 두 축 값을 서로 다르게 둔 `both-axis-cycle`이 있다. |
| 4 | 목록 길이로 나누어떨어지는 track 수만 테스트해 wrap-around 결함을 놓침 | PASS | three-item implicit sequence `[5,7,5]` 및 `[11,13,11]`을 고정했다. |
| 5 | area track을 materialize한 뒤 다음 implicit placement에서 목록을 처음부터 재시작 | PASS | 양 축의 continuation scene은 앞서 한 implicit value를 소비한 뒤 다음 track의 위상을 확인한다. 계획은 소비한 개수만큼 목록을 회전하도록 명시한다. |
| 6 | authored prefix를 목록 첫 값으로 대체하거나 덮어씀 | PASS | 각 expansion scene의 authored track을 기대값에 별도 prefix로 보존한다. |
| 7 | authored track 수와 area 차원이 같을 때 불필요한 implicit track을 추가 | PASS | 기존 matrix 회귀와 authored-longer scene을 유지한다. |
| 8 | area보다 authored track이 많을 때 trailing authored track을 잘라냄 | PASS | `authored-tracks-exceed-area`는 두 축의 trailing track을 고정한다. |
| 9 | 기존 scalar `auto`/px 결과를 목록 경로 변경으로 바꿈 | PASS | C11.3.3 기존 31 scene 및 scalar projection 회귀를 유지한다. |
| 10 | 빈 typed `TrackList`에서 modulo 0 또는 조용한 auto fallback 발생 | PASS | Projection과 layout 검증에서 빈 목록을 명시적으로 거부한다. |
| 11 | 내부 입력의 음수·NaN·무한 px 값이 track 생성으로 유입 | PASS | fixed px 유효성 검사를 거부 gate와 projection/layout tests에 포함한다. |
| 12 | 목록 안의 `fr`, percentage, intrinsic, calc 값을 px-only profile에서 허용 | PASS | 기존 Template profile grammar를 고정하고 비-px control을 거부한다. |
| 13 | C11.4 TrackSizing이나 C11.2 auto-placement profile에 admission을 넓힘 | PASS | profile별 positive/negative gate를 명시했다. |
| 14 | `template_areas` 없는 TemplateGrid에서 목록이 허용됨 | PASS | layout은 area metadata가 같은 제한 경로에 있는 경우에만 목록을 허용한다. |
| 15 | named-line implicit/signed-implicit profile의 별도 px 계약을 약화 | PASS | 이 두 mode는 목록 허용 대상에서 제외하고 negative controls를 둔다. |
| 16 | track reservation 실패 후 rows/columns 일부만 확장된 입력을 반환 | PASS | fallible reserve와 동일 worker 복구를 계획 acceptance에 포함했다. |
| 17 | track은 늘었지만 line-name group 수는 따라오지 않거나 반대로 과다 예약 | PASS | `track_count + 1` line group 및 allocation failure를 독립 검사한다. |
| 18 | 큰 area 차원에서 grid track cap 또는 정수 산술을 넘음 | PASS | 기존 10,000-track·metadata cap과 overflow 실패 경계를 유지한다. |
| 19 | 새 fixture scene이 allowlist, source order, revision, draw/present 검사 없이 성공 | PASS | 기존 31 plus six new scenes의 strict Android/iOS matrix를 acceptance에 고정한다. |
| 20 | fixture symbol이 기본 product artifact로 노출되거나 단일 slice를 C11 완료로 기록 | PASS | 기본/opt-in artifact 대조와 C11.3.3 unchecked·`72/213` 유지 조건이 있다. |

## 판정

PASS 20/20, 계획 blocker 0개. 이 검토는 구현 성공이나 PR diff 통과를 의미하지 않는다. 구현 PR은 새 코드/runtime 20관점 검토와 별도 PR diff 20관점 검토를 거쳐야 한다.

## Source-audit correction · 2026-10-30

구현 착수 전 layout validation을 다시 읽다가 관점 #15의 부정 대조 설명이 기존 동작을 잘못 분류한 것을 발견했다. `TemplateGridImplicitTracks`와 `TemplateGridSignedImplicitTracks`는 이미 fixed-px `TrackList`를 허용한다. 계획의 “named mode는 목록 거부” 문구를 제거하고, 구현 검증은 named mode의 기존 fixed-px 수용 및 non-px 거부를 함께 고정한다. 새 feature gate는 area-expansion의 forward cycle에만 적용된다. 이 수정 때문에 범위가 확장되지 않았고, #15를 기존 동작 보존 판정으로 다시 확인했다. 초기 검토 누락은 숨기지 않고 이 addendum에 남긴다.
