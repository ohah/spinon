# 구현/runtime 실패 관점 검토 20개

검토 대상은 auto-repeat named placement projection, layout 검증·정규화, Taffy named-line resolver 연결, 10개 고정 scene과 Android/iOS Simulator 실행이다. 각 관점은 별도 입력 실패 또는 불변 조건으로 확인했다. 실행 결과는 [runtime 근거](./README.md), 독립 기대값은 [Chrome 사전 비교](../c11-4-auto-repeat-named-line-implicit-fallback-precomparison-2026-10-23/README.md)에 있다.

| # | 실패 관점 | 결과 |
| ---: | --- | --- |
| 1 | authored line metadata에 없는 positive named integer가 새 implicit line에서 해석되는가 | 통과 — Android/iOS positive-missing frame `(220,0) 20×10` 및 track list 일치 |
| 2 | authored occurrence보다 큰 positive occurrence가 repeat expansion 이후 implicit line을 포함하는가 | 통과 — positive-insufficient frame `(140,0) 20×10` |
| 3 | unknown name의 negative integer가 explicit grid 끝 기준 방향을 유지하는가 | 통과 — negative-missing frame `(0,0) 20×10` |
| 4 | authored occurrence가 부족한 negative integer가 implicit line을 소비하는가 | 통과 — negative-insufficient frame과 track list 일치 |
| 5 | definite start 뒤 forward named span이 없는 이름을 검색하는가 | 통과 — `(120,0) 20×10` |
| 6 | definite end 앞 backward named span이 없는 이름을 검색하는가 | 통과 — `(0,0) 20×10` |
| 7 | forward span의 부족 occurrence가 요청 count만큼 tracks를 확장하는가 | 통과 — `(80,0) 60×10`, Columns `[40,40,40,20]` |
| 8 | backward span의 부족 occurrence가 방향 반대로 같은 폭을 만드는가 | 통과 — `(0,0) 60×10`, Columns `[20,40,40,40]` |
| 9 | column 로직만 바뀌고 row auto-repeat에 결손이 남는가 | 통과 — positive row missing `(0,220) 20×10`, negative row control `(0,0) 20×10` |
| 10 | line-name이 없는 numeric placement에 symbolic resolver 변경이 새 의미를 주는가 | 통과 — 기존 numeric named-auto-repeat matrix 및 layout suite 통과 |
| 11 | bare authored name의 생략 occurrence가 첫 occurrence 대신 implicit fallback으로 바뀌는가 | 통과 — style projection은 bare occurrence를 0으로 전달하며 기존 8-scene control이 유지됨 |
| 12 | explicit named occurrence 0이 bare occurrence와 혼동되는가 | 통과 — style projection unit test는 explicit 0을 거부 |
| 13 | line occurrence의 정수 변환 경계가 넓어지는가 | 통과 — `i16` 범위 밖 projection 거부 테스트 유지 |
| 14 | span 0 또는 zero integer-span이 resolver로 통과하는가 | 통과 — decode/validator의 기존 zero rejection 유지 |
| 15 | span count가 Taffy `u16` 범위를 넘을 때 잘리거나 감기는가 | 통과 — projection 경계 테스트가 70,000을 거부 |
| 16 | 두 끝이 span인 충돌 placement가 auto-repeat 분류 변경으로 허용되는가 | 통과 — 기존 paired-span conflict test 및 layout 전체 suite 통과 |
| 17 | non-auto-repeat axis의 unknown line rejection이 제거되는가 | 통과 — 변경 호출은 `Some(auto_repeat)` 분기뿐이며 일반 axis projector는 유지 |
| 18 | authored-name precheck 삭제 뒤 name copy의 할당 실패 경계가 사라지는가 | 통과 — `copy_layout_name`의 `try_reserve`와 구체 오류 경로 유지 |
| 19 | validation 오류가 partial frame/track output을 반환하는가 | 통과 — layout engine의 atomic error contract 및 해당 negative tests 유지 |
| 20 | implicit line 확장이 반복되어 Taffy track 상한을 넘는가 | 통과 — existing expanded-track limit validation은 변경하지 않았고 해당 10-scene run은 제한 내에서 draw/present 완료 |

전체 Rust focused suites, fixture boundary, formatter, workspace Clippy가 통과했다. AVD의 30/30 PixelCopy와 iOS의 30/30 Metal drawable은 선택된 좌표에 한정한다. Border paint, physical Android, hardware GPU, WPT, iOS compositor-after 출력은 이 검토의 통과 범위가 아니다.
