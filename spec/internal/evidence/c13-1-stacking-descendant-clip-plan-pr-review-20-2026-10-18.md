# C13.1 nested stacking descendant 계획 PR 변경 검토 20개 · 2026-10-18

이 검토는 계획 자체의 실패 관점 검토 20개와 별도로, 이번 계획 PR diff·기준 fixture·문서 연결을 살폈다. 사용자 요청에 따라 PR 변경 관점은 서로 다른 20개다.

| # | PR diff 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 오래된 C13.1 사실이나 완료 상태를 새 문서가 덮는가 | `spec/STATUS.md`는 계획·oracle만 추가하고 C13.1/C13 미완료를 유지한다. 로드맵 합계는 상태 대장에서 동적으로 읽는다. |
| 2 | 이 계획이 현재 C13.1 계약의 실제 남은 항목과 연결되는가 | 내부 계약 0060에 기존 복합 stacking descendant 미검증 문구가 있어 해당 경계를 직접 연결했다. |
| 3 | 계획 변경에 implementation code가 섞였는가 | runtime, renderer, FFI, Android/iOS 앱 코드는 건드리지 않았고 fixture와 Chrome 관찰 자료만 추가했다. |
| 4 | Chrome fixture가 target/ancestor/sibling 구조를 모호하게 만드는가 | parent, nested layer, target, root sibling overlay 각각 stable `data-id`를 두었다. |
| 5 | visible 대조군과 clipped cases가 geometry 차이 때문에 달라지는가 | 세 case의 parent/target/overlay 좌표와 크기는 동일하고 `overflow` value만 달라진다. |
| 6 | high descendant z-index가 parent stacking context 외부의 sibling order를 숨기는가 | parent `z=0`, child layer `z=900`, target `z=10`, overlay sibling `z=1`을 고정하고 overlap 결과를 수집했다. |
| 7 | clip edge 경계가 양쪽 표본 없이 추정되는가 | parent 오른쪽 edge 안쪽 `+119.5`와 바깥 `+120.5`를 각각 positive/negative로 둔다. |
| 8 | clip 밖 target frame과 overlay overlap을 한 표본으로 처리하는가 | overlay 전 escape와 overlay overlap을 별도 좌표·기대 색·identity로 분리했다. |
| 9 | paint equality가 hit identity를 대신하는가 | JSON에 `elementFromPoint()` identity와 PNG 기반 RGBA를 독립 필드로 보존한다. |
| 10 | screenshot sample 좌표와 PNG texel index 변환이 불명확한가 | DPR 1, CSS pixel center, pixel RGBA를 읽을 때 `floor(x), floor(y)` 규칙을 기록했다. |
| 11 | 기대 RGBA가 screenshot와 다른 renderer output에서 추정됐는가 | 18개 expected RGBA는 Agent Browser screenshot PNG 바이트에서 직접 추출해 저장했다. |
| 12 | 브라우저 버전만 적고 실제 binary를 고정하지 않았는가 | full Chrome version, UA, macOS/architecture와 launcher/framework binary SHA-256을 남겼다. |
| 13 | 브라우저 viewport가 fixture screenshot과 달라졌는가 | Chrome viewport와 PNG는 모두 `540×128`, DPR 1이며 integrity script로 크기와 hash를 확인했다. |
| 14 | screenshot이나 fixture가 수정돼도 리뷰 근거가 남는가 | HTML SHA-256, PNG SHA-256, observation과 sample JSON을 plan/evidence README에 교차 연결했다. |
| 15 | stable root fallback NodeId가 browser의 body/html 구현 상세와 혼동되는가 | fixture가 명시적 `#fixture[data-id="root"]`를 제공하고 세 clipped escape 결과 모두 그 ID다. |
| 16 | 실제 앱 실행의 revision/generation 경계가 빠졌는가 | 후속 구현 pass 조건에 run/process/surface generation 및 document/style/render revision 일치를 요구한다. |
| 17 | artifact boundary가 새 runtime feature를 기본 산출물에 넣게 되는가 | 계획은 기존 `c13-1-overflow-clip-fixture-test` opt-in 경계만 허용하고 default/product negative control을 요구한다. |
| 18 | simulator evidence가 physical-device/hardware 증거로 부풀려지는가 | API 37 ARM64 AVD software renderer와 iOS Simulator compositor 전 drawable로 제한했고 physical device/다른 emulator를 제외했다. |
| 19 | 독립 API owner의 이벤트/WPT 범위가 C13.1 계획에 섞이는가 | DOM event dispatch는 S05/C29, WPT 실행·일반 적합성 선언은 제외 범위로 분리했다. |
| 20 | 고정된 계획 검토를 PR diff 검토로 재사용하는가 | 계획 failure review와 이 PR delta review는 별도 파일이며 관점·목적을 분리했다. |

**판정:** 계획 PR diff 검토 통과. 이 PR은 Chrome reference fixture와 plan/status/contract links만 제공하며 Spinon runtime pass를 주장하지 않는다.
