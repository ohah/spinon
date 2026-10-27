# C11.4 direct-text legacy fallback runtime matrix 계획 검토 20

검토 대상: [`c11-4-generic-direct-text-legacy-fallback-runtime.md`](./c11-4-generic-direct-text-legacy-fallback-runtime.md)

## 서로 다른 실패 관점

| # | 실패 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | 목록의 13개 scene이 `TEXT_CASES`와 다르거나 누락됨 | `cases.rs` 선언과 scene ID를 대조했다. 13개 모두 있고 순서·문자열이 표와 일치한다. PASS |
| 2 | 새 generic collector가 one-item scene을 positive로 삼아 구 계약을 넓힘 | 목표를 정확히 `Ok(None)`으로 고정하고 둘 이상의 text item을 generic 수용하는 회귀를 실패로 정의했다. PASS |
| 3 | collector의 None만으로 full legacy fallback 성공을 주장함 | 계획은 same snapshot/style revision을 full `c11_4_text::prepare`에 넣고 별도 결과를 확인한다. PASS |
| 4 | special adapter가 먼저 intercept하여 `TEXT_CASES` 분기를 실제로 타지 않을 수 있음 | 각 이전 specialized adapter의 `None`, generic collector의 `None`, legacy resolver의 `Some`, full dispatcher output 일치를 case별로 확인하도록 계획을 구체화했다. PASS |
| 5 | Chrome의 document y 좌표를 app-local 좌표로 오해함 | Acceptance 표는 모든 item을 root-local `(0,0)`으로 기록한다. PASS |
| 6 | Chrome 관찰값을 independent normative expectation으로 순환 사용함 | CSS Grid/Ahem 기대값과 Chrome raw observation을 별도 파일로 유지하며 Chrome을 reference observation으로만 쓴다. PASS |
| 7 | cyclic 3 scene의 별도 fixture/hash가 틀린 source를 가리킴 | 별도 Chrome 156 observation과 spec-derived JSON 경로를 함께 지정했다. PASS |
| 8 | row/column arrays와 CSS used track count를 잘못 대응함 | 각 13 scene의 최종 content track list와 axis별 matcher를 대조하고 serialized trace는 runtime 결과로 별도 남긴다. PASS |
| 9 | Ahem font loading/revision이 다르지만 같은 수치로 통과함 | Runtime input에서 Ahem registry revision과 raw text source를 확인하도록 요구한다. PASS |
| 10 | `fit-content(%)` cyclic case의 expected basis 또는 content-size behavior가 빠짐 | cyclic scene은 원 계획의 별도 oracle/JSON을 재사용하고 기존 app matcher의 pass trace를 확인한다. PASS |
| 11 | `min-width`/`max-width`/box edge가 track contribution에 잘못 섞임 | 세 case를 분리된 row로 기대값과 input table에 보존하고 typed intrinsic input과 used frame을 검사한다. PASS |
| 12 | overflow item의 box frame을 text glyph overflow 또는 paint로 확대 해석함 | 계획은 frame/trace와 draw/present를 구분하며 glyph raster와 pixel equality를 제외한다. PASS |
| 13 | 13 scene 중 일부가 Android/iOS에 없거나 중복 실행됨 | 양 플랫폼에서 각각 13/13과 exact unique scene set을 요구한다. PASS |
| 14 | 기존 broad matcher의 `expected == null => true` 경로가 누락 assertion을 성공 처리함 | 계획은 기존 strict per-scene matcher를 사용하고 누락·중복·미등록 scene 성공을 실패로 정의했다. PASS |
| 15 | runtime 결과가 다른 source commit의 native app binary에서 나옴 | Scene logs, source commit, app artifact hash를 같은 run manifest로 묶도록 한다. PASS |
| 16 | stale document/style revision trace가 current frame과 섞임 | 한 scene 내 revision tuple 일치와 stale trace 실패를 명시했다. PASS |
| 17 | 앱 종료나 simulator 상태를 근거 없이 보고함 | 이번 run에서 실행한 앱만 종료하고, 외부/기존 simulator 상태를 보존하도록 한정했다. PASS |
| 18 | 변경이 없는 artifact exclusion을 새로 주장함 | FFI ABI/source를 바꾸지 않으면 artifact 결과를 새 근거로 주장하지 않는다고 명시했다. PASS |
| 19 | test-only matrix가 공개 support 또는 공식 완료 수를 올림 | public CSS, WPT, C11.4 전체 제외와 `72/213` 유지 조건이 분명하다. PASS |
| 20 | PR image link나 CI 상태를 검증 없이 완료 처리함 | 이미지가 있을 때만 Agent Browser로 immutable link를 확인하고, 빈 check rollup은 CI pass로 보고하지 않는다. PASS |

## 구현 진입 판정

20개 실패 관점에서 계획 범위·oracle·분기 검증·플랫폼 판정의 기능 blocker는 찾지 못했다. #4는 기존 specialized dispatch 분기와 13개 case의 source mapping을 구현 전 source trace/test에서 확인한다는 구체 조건으로 닫았다. **계획 검토: PASS 20 / PARTIAL 0 / FAIL 0.** 계획 PR diff 검토는 이 계획 자체 검토와 별도로 수행한다.
