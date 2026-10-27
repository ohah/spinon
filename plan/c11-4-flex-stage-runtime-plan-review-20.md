# C11.4 flexible track expansion runtime 계획 · 실패 관점 20개

대상: [앱 runtime slice 계획](./c11-4-flex-stage-runtime.md), 기존 §11.7 expected JSON과 Chrome 156 사전 관측.

| # | 공격 관점 | 판정 및 계획에 반영한 조치 |
|---:|---|---|
| 1 | 테스트 case ID가 oracle의 네 scene과 다르면 JSON 연결이 끊긴다. | 정확한 ID 집합을 Rust와 플랫폼 allowlist에서 고정한다. PASS |
| 2 | scene 순서만 맞고 ID 중복을 허용할 수 있다. | 정확한 집합과 개수를 확인하고 중복 ID를 거부한다. PASS |
| 3 | Columns case를 Rows matcher에 넣어도 성공할 수 있다. | case별 target axis를 명시하고 trace axis와 대조한다. PASS |
| 4 | 첫 pass를 무조건 대상 축으로 읽으면 양축 생성 순서 변화에 속는다. | axis enum으로 pass를 찾고 정확히 하나만 허용한다. PASS |
| 5 | `ExpandFlexibleTracks` stage가 빠졌는데 기본값으로 통과할 수 있다. | target pass와 stage의 존재 및 유일성을 요구한다. PASS |
| 6 | stage 중복이나 순서 역전을 놓칠 수 있다. | `MaximizeTracks < ExpandFlexibleTracks < StretchAutoTracks`를 검사한다. PASS |
| 7 | boundary sentinel을 CSS track으로 세어 expected array가 잘못 맞을 수 있다. | trace의 CSS-track flags로 track을 분리하고 sentinel/gutter shape도 확인한다. PASS |
| 8 | 10px gap을 빼먹거나 두 번 빼면 결과가 달라지지만 각 track만 확인해 누락할 수 있다. | target-axis gap 하나를 확인하고 track sum + gap 불변식을 검사한다. PASS |
| 9 | gap을 반대 축에도 설정해 두 번째 pass에 영향을 줄 수 있다. | opposite-axis gap은 0인지 검사한다. PASS |
| 10 | 반대 축을 indefinite로 두어 sizing-mode가 바뀔 수 있다. | root와 viewport에 두 축의 definite 크기를 선언하고 runtime trace를 확인한다. PASS |
| 11 | 1fr:2fr case의 Chrome quantization을 규범 값으로 오해할 수 있다. | exact Fraction expected를 사용하고 Chrome은 1/64px control로 분리한다. PASS |
| 12 | rational denominator 0/음수 또는 overflow parsing이 잘못될 수 있다. | 양수 denominator와 finite 결과를 요구하는 checked parser로 제한한다. PASS |
| 13 | 실제 계산값이 NaN/무한대인데 비교식만으로 통과할 수 있다. | 비교 전에 finite 여부를 요구한다. PASS |
| 14 | subunit factor case에서 남는 85px를 자동으로 채우는 다른 stage를 놓칠 수 있다. | track 합 + gap이 115px이고 남은 85px인 독립 기대를 검사한다. PASS |
| 15 | 0fr track의 24px floor나 frozen 1fr track이 관찰되지 않을 수 있다. | runtime log verifier가 expected JSON의 네 배열과 각 CSS track을 대조한다. PASS |
| 16 | app route가 다른 profile을 선택해 결과가 우연히 비슷할 수 있다. | runtime summary profile을 정확히 `RuntimeBlockFlexGridTrackSizingV1`로 요구한다. PASS |
| 17 | 알 수 없는 scene이 permissive fallback으로 성공하거나 native expected table이 JSON에서 벗어날 수 있다. | Rust dispatcher와 Android/iOS matcher가 unknown ID를 실패 처리하고 verifier가 플랫폼 상수 표와 expected JSON의 drift를 거부한다. PASS |
| 18 | 빈 시각 marker의 automatic minimum이나 content가 track sizing을 바꿀 수 있다. | Chrome 원본과 같은 empty marker에 `min-width:0; min-height:0`을 지정하고, 양 플랫폼 stage array와 최종 frame을 oracle에 대조한다. PASS |
| 19 | root만 그려지고 실제 앱 render/present 경로가 실행되지 않을 수 있다. | 각 run의 ready, draw, presented 결과를 모두 보존한다. PASS |
| 20 | screenshot, simulator 종류 또는 artifact 범위를 과장할 수 있다. | 원본 hash/device 식별을 남기고 화면은 배치 증거로만, archive는 정적 근거로만 보고한다. PASS |

### 계획 검토 결론

위 20개는 ID/axis mapping, trace stage, numeric gate, strict failure, app lifecycle, artifact boundary에서 서로 다른 실패 경로를 검토했다. 모든 항목을 계획의 구체적 합격 조건에 연결했다. 이 검토만으로 구현/runtime이 검증됐다고 간주하지 않는다.
