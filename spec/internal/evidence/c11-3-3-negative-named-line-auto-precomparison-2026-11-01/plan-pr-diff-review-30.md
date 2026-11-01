# C11.3.3 negative named-line pair 계획 PR diff 검토 30개

검토 대상은 계획 PR의 staged diff 전체다. 계획 자체의 20개 검토와 별도로, PR에서 실제 전달되는 파일·근거·문구 간 불일치와 변경 범위를 확인했다. 검토 중 발견한 equal endpoint 기대값의 오래된 설명은 CSS §8.3.1 규칙에 맞춰 수정했고 기대값 hash를 갱신했다.

| # | PR diff 실패 관점 | 확인·조치 |
| ---: | --- | --- |
| 1 | 새 계획이 구현 완료처럼 읽힘 | 계획 파일과 C11 상위 계획 모두 구현 없음, C11.3.3/C11 미완료라고 명시한다. |
| 2 | 공식 완료 집계가 증가한 것처럼 표시 | 공식 `72/213`은 그대로 미완료 상태로 두며 STATUS 체크를 바꾸지 않았다. |
| 3 | 계획 후보가 C11 전체 공식 원본을 덮어씀 | 변경은 C11 planning index의 최근 후보 설명이며 `spec/STATUS.md`는 변경하지 않는다. |
| 4 | 최근 C11.4 merge 내역을 누락하거나 잘못 합침 | 상위 plan에 #945/#962 line-break와 #959 text-indent merge를 분리해 기록했다. |
| 5 | 이미 완료된 positive named-line slice와 새 범위를 혼동 | #933 positive trailing profile을 현재 동작으로 설명하고 negative occurrences를 후속 결손으로 한정한다. |
| 6 | fixed-px signed profile이 새 auto sizing에 영향받음 | `TemplateGridSignedImplicitTracks`의 fixed-px 보존을 명시하고 새 typed mode를 제안한다. |
| 7 | 구현 route 이름과 실제 projection guard가 다름 | 현재 `style_values.rs`, `grid_placement.rs`, `grid_auto_placement/dense_column.rs`의 positive mode/guard 이름을 대조했다. |
| 8 | 구현 계획이 이름만 바꾸고 기존 mode를 넓힘 | separate `GridPlacementMode` 또는 동등 typed policy로 경계를 유지하도록 썼다. |
| 9 | `grid-template-areas` 사용자 계약을 몰래 포함 | fixture-only route와 no-areas gate를 명시한다. |
| 10 | 두 축 동시 implicit 확장이 한 축 gate를 우회 | 대상 한 축과 definite 반대 축을 명시하고 양 축 확장은 거부한다. |
| 11 | authored `ghost` control을 implicit-only feature 증거로 오인 | 이 scene은 precedence control로 분류하고 새 mode에는 두 implicit endpoint가 필요하다. |
| 12 | named-line end/start role을 바꿔 읽음 | 입력 JSON의 placement와 HTML longhands, 관찰 JSON의 start/end 키를 대조했다. |
| 13 | reversed endpoint conflict를 track order로 혼동 | 독립 기대값은 reversed와 canonical case의 track/frame 동일성을 요구한다. |
| 14 | equal endpoint를 auto-placement 전체로 다룸 | PR diff 검토에서 stale derivation을 찾았고 end-line removal 뒤 default span 1로 수정했다. |
| 15 | equal endpoint에 대한 새 correction의 해시가 stale | `expected-values.json` hash와 README manifest를 재생성해 일치시켰다. |
| 16 | offset span의 필요한 extent와 item span을 혼동 | `-5/-2`의 leading track 5개 및 enclosed span 3개를 별개로 기록한다. |
| 17 | fixed track list가 leading side에서 정방향 확장 | 계획은 explicit edge에서 start 방향 확장, 결과는 CSSOM start-to-end로 비교하도록 구분한다. |
| 18 | auto sizing에 explicit free space가 끼어듦 | fixture는 `justify-content/align-content:start`, 고정 container size, zero gap으로 free-space stretch를 억제한다. |
| 19 | item frame이 grid area나 root frame과 혼동 | fixture item border-box와 root-relative frame을 JSON에 고정하고 verifier가 전체 geometry를 비교한다. |
| 20 | 화면 캡처를 수치 oracle로만 사용 | 별도 observation JSON과 independent expected JSON이 track/frame 비교를 맡고 PNG는 시각 부록이다. |
| 21 | verifier가 케이스 누락 또는 순서 변경을 허용 | input·expected·observed 길이와 index별 ID를 확인한다. |
| 22 | verifier가 서로 다른 축의 CSSOM을 읽음 | case axis에 따라 columns 또는 rows resolved listing을 비교한다. |
| 23 | verifier가 다른 Chromium 계열 결과를 묵인 | UA가 HeadlessChrome 156인지 assert하며 README는 revision/hash 미고정 한계를 공개한다. |
| 24 | 값 parser가 CSSOM line-name 토큰 때문에 track을 누락 | verifier는 line-name을 건너뛰고 `px` size 순서를 보존하며 10/10 관찰에서 일치했다. |
| 25 | HTML 기본 style이 예상과 다르게 viewport에 영향 | root/item, body margin, box sizing, gap, alignment는 fixture와 대조하고, 1280×1040 DPR 1 viewport는 input과 Agent Browser observation 사이에서 검증한다. |
| 26 | Chrome 관찰을 Spinon/WPT 결과로 승격 | README와 계획이 사전 비교는 Chrome-only라고 구분하며 앱 runtime/WPT 실행이 없음을 밝힌다. |
| 27 | Android/iOS evidence tier를 과장 | API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator를 미래 gate로 적고 renderer·compositor 제한을 남긴다. |
| 28 | feature marker가 제품 빌드로 유출 | default artifact/opt-in fixture symbol 비교를 구현 후 검증 항목으로 포함한다. |
| 29 | planning links가 근거 파일과 불일치 | 계획, 사전 입력, 기대값, 20-review의 상대 링크가 PR 경로에 존재하는지 확인했다. PR diff review 링크는 이 파일 추가 후 유효하다. |
| 30 | review 파일 자체가 다른 검토를 재사용하거나 검토 상태를 과장 | 본 표는 계획 PR 변경 파일만 대상으로 새 30항목을 기록하고, 계획 검토·구현 검토와 분리한다. |

**결과:** 계획 PR diff에 대해 30개의 서로 다른 실패 관점을 확인했다. equal endpoint 설명과 관련 hash를 수정했다. `git diff --cached --check`와 10/10 precomparison verifier가 통과했으며, 이 결과는 구현/runtime 검증을 의미하지 않는다.
