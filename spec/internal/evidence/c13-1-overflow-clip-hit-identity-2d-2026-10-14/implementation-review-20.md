# C13.1 target-frame 2D hit identity 구현 실패 관점 검토 · 20개 · 2026-10-14

검토 대상은 [`hit_identity_2d.rs`](../../../../crates/spinon-style-to-render/src/stacking/tests/c13_1_clip_matrix/runtime_matrix/hit_identity_2d.rs), 13개 호출 case, Chrome observation과 [capture validator](validate-capture-integrity.mjs)다. 제품 renderer 동작은 바꾸지 않았다. Rust 회귀는 [focused log](cargo-hit-identity-2d-test.log), capture 무결성은 [validator log](capture-integrity-validation.log)로 확인했다.

| # | 실패 관점 | 확인 및 결과 |
| ---: | --- | --- |
| 1 | 다른 Chrome revision을 같은 oracle로 취급 | Rust가 version·revision·UA를 고정 비교하고 validator도 Chrome for Testing `155.0.8059.40` 및 revision을 다시 검사한다. 불일치는 실패한다. |
| 2 | viewport 또는 DPR 변경으로 픽셀 표본 이동 | Rust와 validator가 `1280×1100`, DPR 1을 확인한다. 다른 viewport capture는 수용하지 않는다. |
| 3 | fixture가 바뀐 뒤 예전 capture 재사용 | validator가 저장소의 원본 HTML SHA-256을 다시 계산해 observation metadata와 대조한다. |
| 4 | 이전 1D baseline이 달라졌는데 새 capture를 유지 | validator가 parent observation의 실파일 digest와 고정된 `parentHitIdentitySha256`을 비교한다. |
| 5 | 유효 case 하나 누락 또는 invalid/float case를 성공으로 추가 | Rust는 정확히 13행을 요구하고 모든 기존 case 호출에서 해당 ID를 찾는다. validator는 허용 ID 전체와 일치시킨다. invalid margin 및 float는 이 runtime 성공표본 밖에 둔다. |
| 6 | 동일 ID 중복으로 다른 case의 표본을 재사용 | validator가 13개 기대 ID의 고정 목록·순서를 비교한다. 중복 또는 누락은 실패한다. |
| 7 | fixture에서 잘못된 child를 target으로 선택 | capture는 `.target, .abs-target`을 찾고 `targetId`를 보존한다. Rust는 `{case_id}-target`과 비교해 잘못된 요소 선택을 거부한다. |
| 8 | target 위치·크기 변화에도 예전 hit map 적용 | Rust가 x/y/width/height를 Chrome frame과 최대 0.5 CSS px로 비교하고, 샘플 좌표를 oracle frame에서 계산한 범위에 고정한다. |
| 9 | fractional frame 경계의 `floor`/`ceil` 혼동 | Rust와 validator가 x/y 시작은 `floor`, 끝은 `ceil(edge)-1`인지 각각 검사한다. |
| 10 | zero-width 또는 역전 범위가 빈 성공 행으로 통과 | Rust와 validator가 끝점이 시작점보다 앞서지 않는지 검사한다. 각 y 행에는 최소 한 run이 필요하다. |
| 11 | 첫/마지막 y 행 누락 또는 y 행 중복 | 행 개수와 각 `row.y == yStart + index`를 검사한다. 누락·중복·순서 변경은 실패한다. |
| 12 | 중간 y 행이 빈 결과 | Rust와 validator가 빈 RLE run 목록을 허용하지 않는다. |
| 13 | x run이 한 칸 빠지거나 겹침 | 각 run의 `from`이 직전 `to + 1`이어야 한다. gap과 overlap은 둘 다 실패한다. |
| 14 | x run의 역전 또는 target frame 밖 run | `to >= from`와 마지막 `to == xEnd`를 검사한다. |
| 15 | 마지막 run 뒤 target frame 오른쪽 열이 빠짐 | Rust의 `next_x == xEnd + 1`과 validator의 동일 전체 범위 검사가 실패를 감지한다. |
| 16 | unknown element나 `null`을 false/미스와 혼동 | capture는 모르는 element에서 중단하고, Rust는 `target/box/inner/root/main/body/html` 외 label을 panic 처리한다. `null`도 허용되지 않는다. |
| 17 | `.box` fallback을 root 또는 target으로 합침 | label별 NodeId mapping이 target과 clip parent를 별도 비교한다. Chrome 관찰에는 frame 내부 box/root fallback이 실제 포함된다. |
| 18 | nested inner ID가 없는 scene에 inner label을 적용 | nested row만 `Some(inner_node)`를 넘기며, 다른 row에서 `inner`가 나타나면 `None` unwrap으로 실패한다. 관찰에서는 inner label이 발생하지 않아 해당 label의 positive sample은 아직 없다. |
| 19 | RLE 합계가 실제 frame 면적과 다름 | Rust가 각 case의 실제 sample 방문 수와 정수 frame area를 비교한다. 독립 validator는 각 row·전체를 다시 합산해 187,320점과 세 metadata 합계를 확인했다. |
| 20 | target-frame 결과를 viewport·surface·event 검증으로 과장 | capture 좌표는 target frame으로 제한했고 README·상태·계약은 frame 밖 전체 viewport, PixelCopy/Metal, DOM event dispatch, WPT를 미완료로 남겼다. |

검토 중 발견한 불일치는 없다. 관찰된 identity 중 `inner`는 표본 frame 안에 없어 positive mapping test가 아니다. 2D 표본은 target frame 바깥의 fallback 경계를 검증하지 않는다.
