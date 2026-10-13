# C11.2.2 전용 PR 후보 구현 공격 검토

이 검토는 base `d489361`에서 만든 C11.2.2 전용 후보를 대상으로 한다. C11.2.3·C12.4 후속 구현은 후보 코드에서 제외하고, 이 PR이 제공하는 제한 Grid profile과 실행 증거를 확인했다.

| # | 공격 관점 | 결과 |
|---:|---|---|
| 1 | 서로 다른 기능이 한 PR에 섞여 있는가 | 후보 경로와 diff를 확인했다. C11.2.2 layout·cascade·FFI·fixture·플랫폼 실행 경로와 해당 계획·계약·증거만 포함한다. C11.2.3와 C12.4 후속 코드 변경은 없다. |
| 2 | C11.2.3 dense/column 구현이 조용히 따라 들어왔는가 | Android·iOS demo, JNI, Swift/Objective-C++ 변경에 C11.2.3 symbol·fixture 진입점이 없다. dense·column 입력은 C11.2.2에서 거부된다. |
| 3 | C12.4 soak·Metal scene·depth 후속 변경이 섞였는가 | 플랫폼 diff의 C12.4 추가·삭제 줄을 base와 대조했다. C12.4 후속 변경은 제거했고 기존 main의 C12.4 동작은 보존했다. |
| 4 | Java 선언과 JNI 구현이 서로 다른 symbol을 사용하는가 | Android app 빌드가 Java·JNI·Rust native library 링크를 통과했다. C11.2.2 scene별 runtime matrix도 해당 진입점을 실행했다. |
| 5 | iOS Swift selector와 Objective-C++ ABI가 맞지 않는가 | Xcode Simulator build가 Swift compile과 Objective-C++ link를 통과했고, 16-scene 행렬이 C11.2.2 selector를 실행했다. |
| 6 | 새 profile이 C11.1·C11.2.1 성공 의미를 바꾸는가 | `RuntimeBlockFlexGridAutoPlacementV1`을 분리해 추가했다. 기존 profile 입력·성공 의미를 공용 default로 확대하지 않는다. Rust workspace 회귀가 통과했다. |
| 7 | CSS shorthand/cascade 값이 문자열 추측이나 기본값으로 바뀌는가 | Stylo computed typed `grid-auto-flow`·`grid-auto-rows`·placement 값을 projection에서 검사한다. targeted cascade·style-to-layout 테스트가 통과했다. |
| 8 | definite item을 auto item 뒤에 처리해 셀 충돌을 놓치는가 | definite-first 배치를 Chrome 154 fixture와 대조했다. Android AVD와 iOS Simulator runtime 결과가 모두 일치했다. |
| 9 | definite column cursor가 이전 column으로 이동할 때 같은 row를 재사용하는가 | `column-cursor-wrap` scene을 포함했다. 두 모바일 runtime 행렬에서 해당 scene의 geometry가 기준과 일치했다. |
| 10 | sparse cursor가 앞에서 지난 빈 셀을 다시 채우는가 | `sparse-hole-not-backfilled` 장면과 전용 Rust/fixture assertion으로 hole 비재사용을 확인했다. |
| 11 | item span이 occupancy에서 다른 행·열을 빠뜨리는가 | 행별 interval occupancy와 span fixture를 검사했다. `auto-column-span` scene의 geometry·surface 표본이 두 플랫폼 기준과 일치했다. |
| 12 | hidden item이나 nested Grid가 바깥 배치 상태를 오염시키는가 | `hidden-child-excluded`와 `nested-independent-placement` scene을 각각 확인했다. 두 플랫폼 행렬과 cascade/layout 회귀가 통과했다. |
| 13 | 역순·동일 line·음수 explicit line conflict를 부정확하게 정규화하는가 | 기존 C11.2.1 typed line 입력과의 경계 테스트, Grid placement Rust 테스트를 통과했다. 지원 범위 밖 implicit expansion은 거부한다. |
| 14 | implicit row sizing이 `auto`나 반복 pattern을 고정 px처럼 가장하는가 | 양의 implicit row와 단일 px sizing·gap만 성공한다. auto-sized/repeated pattern은 두 모바일 행렬에서 기대 거부됐다. |
| 15 | dense·column flow나 implicit column을 Taffy 기본값으로 성공 처리하는가 | `reject-dense-flow`, `reject-column-flow`, `reject-implicit-column` 모두 두 플랫폼에서 진단·frame 부재·양수 draw 부재를 확인했다. |
| 16 | span·line 산술 overflow나 Taffy track 상한을 잘라 성공 처리하는가 | checked arithmetic과 track cap 검증이 adapter 호출 전에 적용된다. overflow·상한·잘못된 placement Rust regression이 통과했다. |
| 17 | 계산 실패 후 부분 frame이나 이전 revision이 공개되는가 | 실패는 `Err`로 전달되고 성공 frame을 만들지 않는다. 동일 input의 후속 정상 계산을 확인했다. runtime reject scene도 부분 frame·draw가 없다. |
| 18 | allocator failpoint가 가짜 오류이거나 다른 테스트 thread에 새는가 | 다섯 `Vec::try_reserve[_exact]` site의 실제 test allocator 실패·node/property 귀속·output 부재·같은 worker 및 engine 복구를 검증했다. hook과 allocator는 `cfg(test)`다. |
| 19 | fixture 누락, stale comparator 또는 테스트 artifact가 지원 범위를 과장하는가 | runtime·pixel matrix JSON 두 파일을 포함하고 target JS 검사 37/37, Rust workspace 640 passed·4 ignored를 확인했다. Chrome fixture는 고정 subset oracle로만 표기한다. |
| 20 | simulator 표본을 하드웨어·최종 화면·전체 WPT 적합성으로 오인하는가 | Android 17/API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 16 scene·69 node, 11 성공·5 기대 거부, geometry 오차 0 CSS px, 51/51 renderer-surface sample exact다. AVD는 ANGLE/SwiftShader이고 iOS readback은 compositor 전이다. 실기기 재검증, compositor 결과, Spinon WPT runner·전체 Grid 지원을 주장하지 않는다. |

## 검토 중 발견해 수정한 항목

- PR 경계 분리 중 iOS source에 남은 괄호와 `shouldLog` 중복 선언을 Xcode build가 잡았다. 이를 수정하고 build 및 현재 source 행렬을 다시 실행했다.
- iOS demo에 남아 있던 C12.4 depth retry 변경과 soak/scene 인자 분기를 base 동작으로 복구했다. Android·iOS demo diff에서 C11.2.3 및 C12.4 후속 변경이 빠진 것을 다시 대조했다.
- 첫 임시 AVD 수집 결과의 플랫폼 metadata가 physical로 잘못 표기되어 해당 결과를 폐기했다. 수정된 임시 수집기 patch와 기기 정보를 보존한 재실행만 근거로 삼았다.
- C11.2.3 계획이 아직 없는 후보 tree를 가리키던 링크를 제거했다. `STATUS`, 내부 계약, Grid 계획과 roadmap의 “runtime 미시작” 항목도 현재 PR 후보 상태에 맞췄다.

## 열린 경계

현재 Android exact-candidate 재실행은 API 37 AVD이며 실기기 GPU 결과가 아니다. iOS는 Simulator이고 readback은 OS compositor 제출 전이다. 전체 CSS Grid, Spinon WPT 적합성, 전역 OOM 복구, 다른 기기군 parity와 성능 우위는 이 PR의 완료 주장에 포함하지 않는다.
