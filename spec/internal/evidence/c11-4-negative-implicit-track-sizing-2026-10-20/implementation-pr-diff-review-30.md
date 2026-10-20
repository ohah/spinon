# C11.4 negative implicit tracks implementation PR diff review

2026-10-20 구현 변경을 병합 후보 diff로 30개 서로 다른 실패 관점에서 검토했다. 기준 checkout은 `aa5e6c41`이며, 현재 구현 변경과 최종 124-scene simulator evidence를 확인했다. 다른 작업의 C12.4 untracked evidence와 실패한 중간 캡처는 이 PR 변경 대상에 넣지 않는다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 계획 조각만으로 C11.4/C11 공식 완료 상태나 완료 수가 앞서 올라가는가 | 구현 PR은 해당 완료 체크를 올리지 않는다. 계획과 상태 대장은 별도 후속 문서 갱신 대상으로 둔다. |
| 2 | 구현 범위가 지원되지 않는 일반 Grid 문법까지 넓게 표시되는가 | fixture와 오류 경계는 이번 auto-repeat/negative-line 조합만 다룬다. named line·subgrid·indefinite sizing 등 제외 범위를 evidence에 유지한다. |
| 3 | CSS 규범 기대와 Chrome 관찰값이 하나의 oracle로 섞이는가 | `spec-derived-expectations.json`과 Chrome observation을 별도 입력으로 읽고 둘의 ID 집합 및 구현 geometry를 각각 확인한다. |
| 4 | auto-repeat 축의 authored pattern 길이를 반복 횟수로 오해하는가 | 18개 case의 expanded repeat count와 used track 수, prefix/suffix를 독립 expectation에 대조한다. |
| 5 | 이미 있던 auto-repeat 33 scene이 같은 개수의 다른 scene으로 교체되는가 | 기존 첫 33개 scene ID 배열을 순서까지 고정해 assert하고 현재 목록에서 통과했다. |
| 6 | 새 18 scene ID가 fixture, normative expectation, Chrome observation 사이에서 누락되는가 | 세 집합을 exact equality로 비교하고 FFI opt-in test가 18개 runtime ID를 확인한다. |
| 7 | 기존 106 scene이 사라지거나 중복 scene이 추가되는가 | 단계별 53·73·106 count와 final 124 count, 매 단계 unique ID를 검증한다. |
| 8 | 반복 수가 definite available size, neighbor track과 gap을 무시하는가 | columns/rows, prefix/suffix, fixed/minmax, lower-bound size case를 각각 비교한다. |
| 9 | negative line anchor가 반복 확장 전 authored track 끝으로 잡히는가 | 양축 auto-repeat에서는 secondary axis의 authored count로 선계산하지 않고 raw placement를 Taffy로 전달한다. `-8` boundary와 leading extent trace가 확인됐다. |
| 10 | auto-placement 분류가 numeric negative line을 automatic으로 오판하는가 | 양축 repeat 경로는 definite 여부만 분류하고 signed 좌표 normalizer와 분리했다. zero line은 사전 오류 case로 유지된다. |
| 11 | repeat 뒤 positive numeric line이 확장된 repeat count 기준으로 처리되지 않는가 | `secondary_positive_line_uses_expanded_repeat_grid`와 `columns-fill-negative-positive-cycle` frame/trace가 secondary positive placement를 확인한다. |
| 12 | reversed/equal line, span과 prefix 횡단 영역이 서로 다른 축에서 달라지는가 | normalizer regressions와 18-case matrix에 reversed lines, prefix-spanning placement, 양축 독립 case가 있다. |
| 13 | leading `grid-auto-*` pattern이 authored order 대신 뒤집힌 순서로 적용되는가 | leading auto track list는 역방향 cycle, trailing은 authored cycle로 frame과 track size를 비교한다. |
| 14 | scalar `LengthPx`, zero px 또는 unsupported auto track list가 예기치 않게 바뀌는가 | scalar/zero controls는 유지하고 auto-repeat 결합은 fixed px list만 허용한다. 잘못된 typed track은 구체 오류로 닫힌다. |
| 15 | empty list, negative/NaN track, 다른 profile 값이 새 허용 경로로 새는가 | `validate_grid_layout`의 기존 auto-track validation 뒤 auto-repeat 전용 fixed-px guard가 실행되고 non-TrackSizing profile은 기존 경로를 유지한다. |
| 16 | `auto-fill` 반복 track이 잘못 collapse되거나 `auto-fit` occupied track이 접히는가 | fill은 미접힘, fit은 item placement 뒤 empty repeated track만 접힘을 124-case trace로 확인했다. |
| 17 | auto-fit repeat boundary의 빈 track 양옆 gutter가 남거나 prefix·suffix geometry가 어긋나는가 | vendored Taffy가 repeat boundary를 포함한 양쪽 gutter를 collapse한다. focused style/layout matrix와 simulator gap trace가 같은 규범 기대를 사용한다. |
| 18 | 첫·중간·마지막 active repeat의 남은 공간과 item origin이 같은 값으로 뭉개지는가 | 회귀가 162/170/162px track 분배와 x=28/20/20 origin을 각각 구별한다. span case는 77px track 2개를 확인한다. |
| 19 | allocation failure나 10,000-track cap 오류 뒤 일부 output 또는 다음 worker 상태가 오염되는가 | Spinon-owned reserve failpoint regression과 same-worker recovery가 유지된다. Taffy 내부 allocator 전체 복구를 주장하지 않는다. |
| 20 | fixture source·payload가 default artifact에 들어가거나 production route가 fixture로 바뀌는가 | adapter는 opt-in `c04-runtime-gpu-fixture-test` 아래 있으며 기본/positive-control artifact 경계 증거와 1/1 scene route test를 갖는다. |
| 21 | Android allowlist·box count·node frame이 신규 18 ID와 어긋나는가 | 새 ID별 expected frame/box count가 일치하고 AVD 124/124에서 runtime summary 및 draw/present가 확인됐다. |
| 22 | iOS allowlist·box count·node frame이 Android와 다르거나 Metal fixture가 빠지는가 | iPhone 17 Pro/iOS 26.2 Simulator에서 같은 124 ID가 통과했다. Metal pixel feature flag를 명시해 빌드했다. |
| 23 | Android PixelCopy가 root 밖 item과 tail을 다른 좌표·node로 읽는가 | `columns-fill-negative-positive-cycle` 표본은 root x=200 경계, item-2 x=290, tail x=319를 확인하고 AVD PixelCopy 5/5 exact를 기록한다. |
| 24 | iOS readback이 root 외부 영역을 drawable 밖에서 샘플하거나 item 색을 혼동하는가 | 동일 CSS 좌표가 3x drawable의 유효 픽셀로 변환되고 Metal sample 5/5 exact다. |
| 25 | root edge 표본을 border stroke로 부르거나 외부 여백을 CSS root 배경으로 오해하는가 | sample은 root background, drawable clear 영역, overflow item/tail을 구분한다. border stroke 검증이라고 표현하지 않는다. |
| 26 | pixel 표본 요청이 엉뚱한 scene에 붙거나 실제 색 비교가 빠지는가 | Android scene-index allowlist, node ID/좌표 whitelist, 색 기대값과 readback exact 비교를 모두 요구한다. iOS도 정적 sample tuple로 요청을 고정한다. |
| 27 | 요약 marker만 있고 실제 layout trace 또는 draw/present가 없는 scene이 성공으로 기록되는가 | 124 scene마다 summary, complete trace chunks, node frames, draw/present marker를 validator가 요구한다. |
| 28 | 캡처 evidence가 일부 scene만 저장하거나 이전 실패 run을 최신 통과로 표시하는가 | 최종 경로는 플랫폼별 124 PNG와 124 observation을 갖고 retry 0회다. README는 최신 passing path만 연결하며 중간 실패 폴더는 PR에서 제외한다. |
| 29 | build/test 결과가 실행되지 않은 검사 또는 simulator 성공을 실기기 증거로 과장하는가 | workspace 1,178/0/4, Clippy, rustfmt, fixture boundary 6/6, FFI opt-in test 1/1, 양 simulator build/capture 결과를 구분한다. 물리 기기는 연결하지 않았다. |
| 30 | screenshot과 문서가 simulator·browser oracle·렌더링 범위를 혼합하는가 | PR에 Android/iOS 앱 화면을 별도로 첨부하고 Chrome 155 이미지는 oracle로 표시한다. AVD software rendering, iOS compositor 이전 readback 한계를 밝힌다. |

## 수정 사항 및 잔여 판정

- 검토 중 발견한 Chrome 155 경계 gutter 기반 style-to-layout 예상값, Android 신규 scene PixelCopy allowlist, iOS Metal feature build flag를 고쳤다.
- 124/124 simulator matrix와 새 overflow scene의 5/5 표면 표본을 두 플랫폼에서 다시 실행했다. 최종 workspace test, all-target Clippy, rustfmt, fixture boundary와 opt-in FFI 검사도 모두 통과했다.
- 병합 blocker는 남지 않았다. CSS Grid 전체 지원, WPT 적합성, 실기기·하드웨어 GPU·OS compositor 이후 결과는 이번 diff에서 보증하지 않는다.
