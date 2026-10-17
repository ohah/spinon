# C13.1 전체 viewport hit identity PR 변경 검토 30개

이 검토는 구현 20개 실패 경로 검토와 다른 관점에서 최종 diff, 상태 문서, 플랫폼별 근거와 PR 전달 품질을 확인한다. 검토 기준 source는 1375b7978e6d10d200b8d7b1cc0471dda35e5b73이며, PR 생성 후 원격 diff와 CI 상태를 다시 확인한다.

| # | 변경 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | fixture harness가 public API/header에 추가되는지 | 추가되지 않았다. FFI report는 기존 opt-in fixture route에만 붙는다. |
| 2 | 내부 계약이 버전·지원 경계를 명시하는지 | 내부 문서 0078에 초안 0.1.0, 비공개 계약, 오류와 플랫폼 경계를 기록한다. |
| 3 | 계획의 이전 미착수 상태가 남는지 | 계획 문서 상태를 Android/iOS fixture 구현 및 Simulator 실행 완료·PR 대기로 갱신했다. |
| 4 | STATUS가 검증 범위보다 큰 완료를 표시하는지 | C13.1 후속 행만 추가했다. 상위 C13.1/C13 완료 체크는 그대로 미완료다. |
| 5 | live 완료 집계가 고정 숫자로 잘못 갱신되는지 | 완료 총계를 바꾸지 않고 공식 상태 대장에서 계산하도록 기존 규칙을 유지했다. |
| 6 | oracle 파일과 설명의 hash가 불일치하는지 | 문서, source constant, Android/iOS report가 oracle SHA 279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2로 일치한다. |
| 7 | expanded identity digest가 문서와 실행에서 다르게 적히는지 | README·계약·세 앱 run에서 866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483로 일치한다. |
| 8 | point 수·row/run 수를 하나의 혼동된 수치로 적는지 | 69,120점, 128 row, 1,136 run을 구분해 oracle 설명에 고정했다. |
| 9 | fixture의 data-id inventory 및 순서가 설명에서 누락되는지 | 내부 계약과 계획에서 13개 node order를 명시하고 source oracle에도 pin한다. |
| 10 | 기존 13 frame·18 hit sample을 대체했다고 오해하는지 | 양 플랫폼 summary와 README에서 기존 frame·고정 hit sample이 유지됨을 확인했다. |
| 11 | hit grid와 surface pixel parity가 같은 claim으로 보이는지 | 앱 README와 STATUS가 hit 69,120점과 표면 RGBA 18점만 명확히 분리한다. |
| 12 | Android PixelCopy 캡처 크기가 화면 크기와 혼동되는지 | 1418×336 draw surface와 1080×2400 AVD screenshot을 별도 기록하고 각 예상 크기와 대조했다. |
| 13 | iOS drawable sample이 compositor 이후 결과로 오해되는지 | iOS 근거와 범위에서 Metal drawable 18점만 확인하며 compositor/scanout은 미검증이라고 명시한다. |
| 14 | simulator/AVD를 물리 기기 근거로 부풀리는지 | Android API 37 ARM64 QEMU AVD, iPhone 17 Pro iOS 26.2 Simulator로 정확히 구분한다. |
| 15 | 각 run이 최종 source를 가리키는지 | postcommit summary의 sourceRevision이 1375b7978e6d10d200b8d7b1cc0471dda35e5b73와 일치한다. |
| 16 | 신규 프로세스 3회가 PID 재사용이나 같은 세션으로 기록되는지 | 플랫폼별 세 정상 run은 서로 다른 PID와 run ID를 보존한다. |
| 17 | APK/app executable hash가 빠지거나 양 플랫폼 hash가 뒤바뀌는지 | Android APK와 iOS executable hash를 플랫폼별 summary와 실행 README에 기록했다. |
| 18 | V8 dependency revision을 움직이는 checkout에서 추적 가능한지 | 두 summary가 공통 V8 revision 7b50b62cb18f28617959e8452e2cd18195b38bcf를 기록한다. |
| 19 | PR 화면 링크가 Chrome oracle와 앱 화면을 섞는지 | oracle screenshot과 Android/iOS app screenshot을 별도 제목·path로 제시하도록 PR 본문을 구성한다. |
| 20 | PR image URL이 로컬 경로 또는 잘못된 이미지 확장자를 가리키는지 | 통과. push한 branch의 Android PNG와 iOS PNG raw URL을 Agent Browser에서 직접 열었다. 각각 complete=true이며 natural size 1080×2400, 1206×2622로 로드됐다. |
| 21 | artifact manifests가 source screenshot을 빠뜨리는지 | 최종 Android/iOS run 폴더마다 artifacts.sha256을 보존하고 README에서 링크한다. |
| 22 | 화면 캡처 크기·surface PNG 크기가 확인되지 않는지 | capture script dimension check와 각 evidence README에 검증된 실제 dimensions를 남겼다. |
| 23 | 실패한 이전 캡처 산출물이 성공 근거로 stage되는지 | PR stage 목록은 postcommit 최종 run 두 디렉터리만 포함한다. 이전 실패 artifact는 untracked로 남기고 stage하지 않는다. |
| 24 | default build에서 fixture payload가 새어 들어가는지 | 별도 feature-boundary 기록에 default archive 부재와 opt-in archive positive control을 저장한다. |
| 25 | Android Cargo feature와 native define이 불일치하는지 | Android/iOS capture build 명령의 fixture feature 및 플랫폼 define 경계를 문서화한다. |
| 26 | Xcode target에서 신규 Swift parser source가 누락되는지 | pbxproj source membership에 C131StackingFullViewportHitGridSummary.swift가 등록되고 plutil/build가 통과했다. |
| 27 | capture script에서 기기 종료·route cleanup 단계가 빠지는지 | 각 script는 앱 process를 종료하고 최종 shutdown log를 보존한다. 실행 후 앱 PID가 없음을 확인했다. |
| 28 | PR 설명이 미실행 전체 workspace·WPT 테스트를 통과로 말하는지 | focused tests와 Clippy만 열거하고 전체 workspace test/WPT 미실행을 명시한다. |
| 29 | 화면상 parent 바깥 영역이 검사 대상에서 빠진 것처럼 보이는지 | viewport 전체 정수 grid이며 parent frame 바깥 descendant/overlay point도 포함한다고 README·계획에 설명했다. |
| 30 | 후속 작업이 상위 단계 완료로 승격되는지 | 범위를 세 고정 fixture scene의 hit identity에 한정하고 일반 CSS, DOM event, CSSOM, WPT 및 C13.1/C13 completion을 제외했다. |

## 발견 사항

차단 결함은 남지 않았다. 검증한 두 원격 화면을 PR 본문에 inline 첨부한다. 원격 PR 생성 이후 CI, review decision, mergeability는 이 정적 diff 검토의 판정과 구분해 기록한다.
