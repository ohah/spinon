# C11.1 direct-text aspect-ratio 구현·runtime 적대 검토 20개

이 검토는 현재 후보 변경 코드와 pass 04 evidence를 대상으로 한다. 계획 검토 및 나중의 PR diff 검토와 별개다.

| # | 독립 실패 관점 | 확인 결과와 근거 |
|---:|---|---|
| 1 | `aspect-ratio` 허용이 다른 computed-style profile로 새어 나감 | Root HTML text profile만 numeric longhand를 유지하고 block-paint profile은 계속 거부한다. cascade test 1/1 통과. |
| 2 | CSS computed serialization과 typed ratio가 서로 다른 값을 전달 | candidate gate가 `2 / 1`, typed `2.0`, `BoxSizing` identity를 함께 확인한다. positive projection/layout test 통과. |
| 3 | `auto <ratio>`, 0 또는 다른 숫자 ratio가 지원처럼 처리 | cascade의 auto negative와 projection의 `3/2`, `0/1` 거부 test 통과. |
| 4 | 한 Grid owner에 ratio item이 둘 이상일 때 일부만 배치 | owner별 ratio item count를 먼저 검사하고 둘 이상을 `aspect-ratio` 오류로 거부한다. negative test 통과. |
| 5 | ratio가 Grid item이 아닌 block/root/다른 문맥에 적용 | owner lookup 부재와 Grid-root ratio를 거부한다. block-root/Grid-root negative test 통과. |
| 6 | nested Grid를 direct owner 장면처럼 받아들임 | Grid ancestor가 있는 owner를 candidate에서 제외한다. layout gate도 mode와 node-child 계약을 검사한다. |
| 7 | 두 fixed column·한 auto row·70px root 외 track 계산이 넓어짐 | projection/layout gate가 `[10,60]`, `[auto]`, 70px width 및 zero gap을 고정한다. non-candidate row test 통과. |
| 8 | named area, line, repeat, implicit track이 ratio mode에 섞임 | typed track metadata·area·repeat와 placement contract를 candidate gate가 거부한다. named-area negative test 통과. |
| 9 | 자식 순서·개수 변경이 `A`/`AAAA` fixture로 오인됨 | owner direct-child count와 source-order text pair를 정확히 검사한다. layout test는 지정 NodeId의 각 frame을 확인한다. |
| 10 | inline descendant, anonymous item, replaced element가 direct text로 취급 | HTML namespace/local-name `div`와 text-only child shape를 검사한다. tag mismatch 및 child shape가 fail-closed다. |
| 11 | ratio가 width/height의 지정된 definite size 외 축으로 전이 | target dimensions는 width 60/height auto 또는 width auto/height 30만 허용하고 frame을 양 축에서 검증한다. |
| 12 | control item 또는 두 번째 item이 ratio 영향을 상속 | short item은 ratio 없음/auto size, target 하나만 typed ratio 허용이다. width/height controls의 frame test 통과. |
| 13 | box edges, min/max, overflow, position 또는 order가 계산을 바꿈 | root/item의 zero edges, min auto/max none, static, visible overflow와 order 0을 gate로 확인한다. max-height negative test 통과. |
| 14 | 새 ratio route가 비기본 alignment를 C11.5 경유로 우회 허용 | root content alignment normal, item self-alignment auto를 추가 gate로 고정했다. `align-content`, `justify-content`, `align-self`, `justify-self` 각각의 negative controls 통과. |
| 15 | writing mode나 direction에 따라 축 의미가 뒤집힘 | owner/item과 paragraph가 LTR·horizontal-tb인지 확인하고 layout input은 LTR이다. 후보 외 방향은 거부한다. |
| 16 | intrinsic text snapshot cache miss가 다른 Root HTML route까지 바꿈 | width별 cached snapshot 우선이며 cache miss 재계산은 새 placement mode의 direct child에만 제한된다. 다른 mode는 기존 mismatch 오류를 유지한다. |
| 17 | cache에서 돌려준 text layout이 요청 width와 다름 | 재계산/캐시 결과 모두 `available_width` key를 검사하고 mismatch면 오류로 끝난다. |
| 18 | 무한 auto row가 무제한 implicit size를 만들거나 definite height를 잘못 요구 | ratio placement mode만 bounded auto-row contract와 허용된 auto/fixed child size를 거친다. 양 scene Rust frame test 통과. |
| 19 | cascade/projection/layout 오류 뒤 partial output이 공개됨 | runtime test는 완성된 동일 key의 cascade/layout과 ID별 frame을 요구하고, unsupported input tests는 `Err`를 확인한다. |
| 20 | 플랫폼 결과가 pixel/WPT/물리기기 적합성으로 과장됨 | Android/iOS 모두 기본 app route에서 layout-ready와 draw/present를 확인했으나 README는 Android SwiftShader, iOS Simulator, screenshot-only, readback 미실행을 구분한다. |

새로 확인된 기본 alignment scope 누락은 후보 gate에 수정했고 focused negative controls로 확인했다. 전체 package suite의 실패 10개(style-to-layout)와 1개(spinon-layout)는 기준 HEAD에서 같은 test names/assertion으로 재현됐다. 세부 원본은 [runtime README](./README.md)와 baseline/candidate log다.
