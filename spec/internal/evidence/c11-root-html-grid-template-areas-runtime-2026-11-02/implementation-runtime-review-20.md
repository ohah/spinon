# C11.1 named-area 구현·runtime 실패 관점 검토 20개

**대상:** `feat/c11-root-html-grid-named-area-runtime-20261102`의 Root HTML cascade, typed projection, runtime tests 및 Android/iOS Simulator app run. 계획 검토와 구분해 primary agent가 구현 후 순차 확인했다. 아래 항목은 서로 다른 계약 경계와 실패 경로를 사용한다.

| # | 실패 관점 | 실제 확인과 판정 |
| --- | --- | --- |
| 1 | C11.3.3 fixture route가 기본 제품 화면을 대신한다. | Runtime test는 `new_runtime_gpu_with_root_html()`을 사용한다. AVD/iOS log는 기본 `index.html` app route를 기록하며 C11 fixture flag를 사용하지 않았다. 통과. |
| 2 | CSS property allowlist 변경이 모든 profile에 퍼진다. | 새 area/placement properties는 Root HTML text-paint author allowlist에 추가됐고 computed capture 및 CSS-invalid diagnostic recovery의 profile 조건을 확인했다. 다른 author profiles는 변경되지 않는다. 통과. |
| 3 | 전체 `grid-template-areas` grammar가 제한 후보로 오인된다. | Projection은 정확한 2×2 matrix, 고정 두 track과 세 area rectangle만 받는다. Nonrectangular, 차원 불일치, unknown/extra area는 runtime matrix에서 실패하고 scene이 없다. 통과. |
| 4 | 동일 면적의 다른 fixed track을 허용해 fixture 전용 증거를 넓힌다. | Root candidate는 columns `[40,80]px`, rows `[20,50]px`를 검사한다. 변경 track은 지원 밖 실패 case로 확인했다. 통과. |
| 5 | area 위치가 HTML ID, class, selector 이름 또는 DOM index에 종속된다. | 긍정 runtime test에서 ID·data attribute selector를 바꾸고 child order를 reverse한 뒤 semantic frame을 비교했다. 통과. |
| 6 | DOM order 변경이 visual placement와 paint source order를 혼동한다. | 성공 장면은 `content/header/side` 순서에서 template 배치를 유지한다. scene provenance 순서를 frame owner 추론에 사용하지 않는다. 통과. |
| 7 | computed area와 layout이 다른 document/style revision에서 결합된다. | integration assertion이 layout completion key와 typed cascade completion key를 비교한다. 통과. |
| 8 | 새 path가 CSS 문자열을 다시 읽거나 frame을 사후 조정한다. | `grid-layout`과 item placement는 typed snapshot을 projection한다. frame 후처리 보정은 없다. 구현 경계를 확인했다. 통과. |
| 9 | item owner를 NodeId 순서로 header/side/content에 임의 배정한다. | candidate는 각 element의 computed `grid-area` 네 endpoint에서 이름을 얻고, template area 이름 집합과 일대일 소유를 검사한다. 통과. |
| 10 | 일부 placement edge만 area 이름이고 나머지는 numeric/auto다. | 네 computed endpoint가 같은 `NamedLine`이어야 한다. 숫자 placement와 span 입력을 기존 및 추가 negative matrix에서 거부한다. 통과. |
| 11 | 중복 또는 빠진 semantic item이 다른 area의 frame을 채우거나 큰 문서에서 임시 collection이 무제한 커진다. | visible item 수가 3을 넘으면 즉시 거부하고 고정 3-slot 배열로 보관한다. owner mapping과 area 이름도 같은 상한에 대조한다. missing/duplicate/unknown/extra cases가 실패한다. 통과. |
| 12 | hidden, replaced, nonempty 또는 nested item이 부분적으로 그려진다. | visible direct child만 허용하며 hidden, `img`, nested child 및 nested Grid cases가 no-scene 실패한다. 통과. |
| 13 | anonymous non-whitespace text가 Grid item으로 누락된다. | 직접 텍스트 노드를 검사해 CSS whitespace 외 문자를 거부한다. negative case가 실패한다. 통과. |
| 14 | whitespace text가 source DOM에 있으므로 의미 없는 item으로 오인된다. | ASCII CSS whitespace text는 무시하고 빈 세 `div`만 area 소유자로 확인한다. 고정 fixture를 통과한다. |
| 15 | root dimension, min/max, gap 또는 edge 변경이 고정 기대값을 속인다. | 120×70 size, min/max, zero gap, track, margin/edge와 unsupported root style negative cases를 검사한다. 통과. |
| 16 | invalid CSS declaration이 unsupported computed candidate와 혼동된다. | 문법 오류 `grid-template-areas`를 cascade에서 drop하는 fallback control과 syntactically valid out-of-scope template의 layout 거부를 별도 검사한다. 통과. |
| 17 | CSS-invalid recovery helper가 무관한 declaration/error를 무시한다. | 이름이 지정된 Grid area/placement property와 `OtherInvalidValue` 조합만 매치한다. 다른 Grid property, color, `UnexpectedToken` controls는 거부된다. 통과. |
| 18 | allocation 실패가 잘못된 node/property를 보고하거나 부분 template를 돌려준다. | 실제 reserve가 있는 line-group, area vector, area name failpoint에서 `GridAllocationFailure`의 node 및 축/`grid-template-areas` property를 확인했다. 빈 inner `Vec::new()`를 allocation failpoint으로 오인하지 않도록 불필요한 reserve callback도 제거했다. 통과. |
| 19 | 환경 변경 이후 failed layout에 이전 성공 render scene이 남는다. | 넓은 media-query 성공 후 좁은 새 environment revision에서 unknown area를 선택했다. 새 key와 Failed 상태, `completed=None`을 확인했다. 통과. |
| 20 | 플랫폼 테스트가 fixture screenshot 또는 pixel readback 결과로 과장된다. | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 app route에서 동일 HTML hash, `layout=ready`, draw/present와 화면 캡처를 확인했다. PixelCopy, Metal readback, 실기기·hardware GPU·post-compositor 색상은 주장하지 않는다. 통과. |

## 판정

검토 중 candidate 검증의 동적 collection이 child 개수에 따라 무제한 확장되는 문제를 발견했다. 지원 item 수를 초과하는 즉시 거부하고 고정 슬롯을 사용하도록 수정했으며 Root HTML 집중 tests, allocation failpoint, Clippy, rustfmt 및 수정 source의 두 simulator build/run을 재확인했다. 남은 결함은 발견하지 않았다. 전체 `spinon-style-to-layout` 실행의 C07 aspect-ratio serialization 실패는 이 변경과 다른 기존 비교 케이스로 따로 기록하며 crate suite를 green으로 분류하지 않는다. 이 제한 slice는 구현 PR 생성·GitHub checks/review·merge 전이고 C11.1/C11.3.3/C11 완료 또는 공식 `72/213` 변경을 뜻하지 않는다.
