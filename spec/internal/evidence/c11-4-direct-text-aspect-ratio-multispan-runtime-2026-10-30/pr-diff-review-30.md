# C11.4 direct-text aspect-ratio two-column span implementation diff review 30개

검토 대상은 현재 feature branch의 전체 diff다. 범위는 단일 HTML Grid root, direct text `AAAA` 하나, 두 equal `max-content` column span, 9개 고정 ratio/minmax 장면 및 Android/iOS 비교기다. 30개 항목에서 추가 차단 문제는 발견하지 않았다. 이는 일반 CSS 적합성이나 C11.4/C11 전체 완료 판정이 아니다.

| # | PR diff 실패 관점 | 결과 |
|---:|---|---|
| 1 | 새 collector가 기존 direct-text 처리보다 늦게 호출되어 span 장면이 다른 adapter에 흡수되는가 | `c11_4_text::prepare`에서 기존 baseline 뒤, 두-column span 전용 profile을 minmax·기존 경로보다 먼저 호출한다. |
| 2 | span 모듈 선언이나 호출이 빠져 컴파일은 되지만 runtime에 도달하지 않는가 | `generic_direct_text.rs`의 path module과 `c11_4_text.rs`의 collector 호출이 연결돼 있다. runtime focused test가 이 adapter를 직접 호출한다. |
| 3 | 임의 scene suffix가 고정 입력인 것처럼 허용되는가 | FFI `SCENES` 9개 allowlist를 exact suffix로 조회하고 unknown suffix는 `None`이다. |
| 4 | 서로 다른 모듈 prefix가 새 route에 충돌하는가 | route prefix `grid-text-aspect-ratio-multispan-`을 정확히 제거한 뒤 allowlist에 일치해야 한다. |
| 5 | 화면 수나 scene 이름이 oracle과 다른가 | FFI route test와 capture runner가 신규 9개 및 전체 51개 장면 수를 단언한다. |
| 6 | 생성된 route에서 node ID와 scene ID가 일치하지 않는가 | root, item, 반환된 `sceneId`가 동일한 case suffix에서 생성되며 route test가 결과 문자열을 확인한다. |
| 7 | root CSS 입력이 독립 비교 fixture와 달라지는가 | root width 100px, `max-content max-content`, auto row, row-flow와 0 gap을 route 문자열에서 고정한다. |
| 8 | item 외에 두 번째 item이나 텍스트가 조용히 만들어지는가 | source route는 element 두 개와 `AAAA` Text 하나만 만들고 route test가 그 개수를 검사한다. |
| 9 | 별도 anonymous direct text가 item 기여로 합쳐지는가 | meaningful anonymous text를 거부하고 collapsible HTML whitespace만 무시한다. |
| 10 | nested text나 item 내부 다른 topology가 direct-text 측정으로 오인되는가 | `direct_text_source` 결과가 `AAAA` Text node인지 확인하며 negative test가 nested element를 거부한다. |
| 11 | HTML image/replaced content가 텍스트 경로로 처리되는가 | root subtree에서 HTML image를 탐색해 span profile에서 fail-closed한다. |
| 12 | 의미 없는 element나 빈 추가 child가 one-item 조건을 통과하는가 | direct element 수와 meaningful-text item 수를 각각 검사해 정확히 하나만 허용한다. |
| 13 | 자동 배치 장면이 새 explicit-span profile로 바뀌는가 | explicit placement가 없으면 새 collector가 `None`을 반환하며 negative test가 기존 경로로 넘기는 것을 검사한다. |
| 14 | 1개/3개 span, 다른 시작 선 또는 명시적 row span이 허용되는가 | typed placement가 column line 1/span 2, 자동 row start/end인지 검사하고 네 topology negative case가 거부된다. |
| 15 | named line이 숫자 선으로 잘못 해석되는가 | named-line negative case가 허용되지 않는 computed placement를 거부한다. |
| 16 | 두 column 중 하나가 max-content가 아니거나 root에 추가 row/gap이 있는가 | root typed track/grid geometry 검사와 fixed second-track, nonzero gap, fixed row negative cases가 제한을 확인한다. |
| 17 | root의 namespace, dimension, edge, position 변형이 profile에 섞이는가 | 공통 root validator가 HTML/Grid profile, 100px auto-height, static position과 zero edges를 요구한다. |
| 18 | item 외곽이 content box 계산에 영향을 주는가 | item padding과 border가 0이어야 하며 negative padding case는 실패한다. |
| 19 | stale snapshot/style revision과 잘못된 ratio owner가 이어지는가 | collector가 snapshot/style revision을 비교하고 ratio owner는 item의 typed style에서만 얻는다. |
| 20 | 직렬화된 비율과 typed 비율이 다르거나 역비율이 통과하는가 | 정확한 `2 / 1`, border-box sizing type, numeric `2.0`과 computed ratio 일치를 요구하고 inverted/auto-ratio 변형이 거부된다. |
| 21 | no-ratio control이 numeric ratio 결과를 재사용하는가 | `aspect-ratio:auto`는 typed ratio 부재를 요구하고 9-case JSON과 simulator matrix에서 별도 40×10 control로 검사한다. |
| 22 | percentage, calc, 음수, 비유한 또는 지원 상한 밖 min/max가 부분 적용되는가 | 고정 길이 0..=60px subset을 검증하며 percentage와 60px 초과 negative cases가 거부된다. |
| 23 | 더 큰 minimum이 작은 maximum에 잘못 눌리는가 | runtime regression은 `min-width:60px; max-width:40px`에서 60×30 frame과 30px씩 두 열을 확인한다. |
| 24 | 새 paragraph가 CSS text style이나 지원되지 않는 explicit placement를 숨기는가 | span 전용 paragraph profile이 explicit placement/ratio/minmax만 허용하고 나머지 text declarations는 기존 validators가 검사한다. |
| 25 | 시스템 font fallback이 Ahem contribution을 바꾸는가 | 시스템 font 없는 engine에 Ahem을 등록하고 source와 paragraph style 모두 Ahem 10px/10px을 고정한다. |
| 26 | ratio owner와 paragraph map의 node가 어긋나는가 | runtime wrapper가 exact owner가 paragraph map에 있는지 검사하며 한 item 테스트가 node ownership을 확인한다. |
| 27 | equal span distribution의 기대값을 Spinon 현재 출력에서 역산했는가 | 9개 geometry는 구현 전 Agent Browser 관측과 독립 CSS Grid 계산 JSON에서 오며 verifier가 해시와 1px 변형 거부를 확인한다. |
| 28 | runtime trace 검사가 최종 크기만 보고 잘못된 단계나 contribution을 통과시키는가 | Rust, Android, iOS 검사가 per-track stage base/growth-limit, span 2 column·span 1 row contribution과 multiplicity를 분리해 검사한다. |
| 29 | 단일 item 확장으로 기존 42개 direct-text 장면의 검사 강도가 약해지는가 | 기존 42개와 신규 9개를 합친 각 51-scene 성공 실행에서 기존 2-item expectation과 신규 1-item expectation이 각자 적용된다. |
| 30 | 앱 표본 또는 문서가 glyph, seam, 실기기, 전체 C11.4를 과장하는가 | 표본은 item/root/clear만 증명하고 glyph는 runtime text layout·Chrome geometry만 근거로 둔다. README는 AVD software renderer, iOS pre-compositor readback, 72/213 및 미지원 범위를 명시한다. |

## 검토 근거와 잔여 한계

- Source diff 및 새 Rust/FFI 파일, Android/iOS expectation·trace comparator, simulator capture runner를 검토했다.
- [구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md), [독립 precomparison](../c11-4-direct-text-aspect-ratio-multispan-precomparison-2026-10-30/README.md), 그리고 [두 플랫폼 51-scene 성공 실행과 검사 결과](./README.md)를 함께 확인했다.
- 새 blocker는 발견되지 않았다. 이 검토는 fixture-test profile을 벗어난 일반 author CSS, 다른 span topology, hardware/실기기 GPU, iOS compositor 이후 화면, WPT 또는 전체 C11.4를 검증하지 않는다.
