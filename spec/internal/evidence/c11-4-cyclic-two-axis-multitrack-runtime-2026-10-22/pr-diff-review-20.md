# PR #478 변경분 검토 20개

검토 대상은 `main`의 base `c166bf90b69867f68cdd4f2675c29307cfe03964`와 PR head `f9993061879b2969559eca36f2ed738ea9bf1763` 사이의 전체 변경 및 PR 본문이다. 아래 항목은 구현/runtime 검토와 겹치지 않도록 문서·배포·교차 파일 연결·검토 가능성을 따로 확인했다.

| # | PR 변경 실패 관점 | 검토 및 판정 |
| --- | --- | --- |
| 1 | PR 범위에 fixture와 무관한 제품 동작 변경이 섞였는가 | diff는 네 fixed scene harness/runtime, 상태·계획 설명과 final evidence에 한정된다. 통과. |
| 2 | 공식 C11.4가 완료로 승격되거나 완료 수가 바뀌었는가 | `spec/STATUS.md`에는 미완료 하위 slice 한 줄만 추가했고 parent checkbox 및 `72/213` 완료 수는 바꾸지 않았다. 통과. |
| 3 | 집중 계획의 table ID가 실제 scene registry와 다른가 | 네 표 ID를 FFI/runtime/Android/iOS/script의 이름과 대조했다. 이전 draft의 다른 이름은 수정했다. 통과. |
| 4 | 상위 계획이 사전 계획 단계로 남아 구현을 감추는가 | 상위 roadmap에 네 scene, 양 플랫폼 결과, boundary pixel evidence와 미완료 범위를 연결했다. 통과. |
| 5 | Chrome 사전 관측과 규범 계산의 역할이 뒤섞였는가 | 계획·PR은 CSS Grid 2025 CRD의 독립 계산을 기대값으로 두고 Chrome 156을 보조 oracle로 표현한다. 통과. |
| 6 | 내부 계약 버전 증가나 public CSS admission을 잘못 암시하는가 | public API 변경은 없고 기존 계약 0059의 opt-in fixture 경로 확장으로 명시했다. 통과. |
| 7 | PR 본문에 양 플랫폼의 실제 동작 화면이 보이는가 | Agent Browser에서 두 raw PNG가 모두 `complete=true`; Android natural size `1080×2400`, iOS `1206×2622`, 각 rendered width 300px을 확인했다. 통과. |
| 8 | screenshot URL의 commit/ref가 틀려 merge 전 또는 이후 이미지가 깨지는가 | 최초 URL SHA 오류로 이미지가 깨졌지만 `f9993061879b2969559eca36f2ed738ea9bf1763`로 수정했다. Agent Browser에서 새 PR 본문을 재로딩해 두 이미지가 정상 로드되는 것을 확인했다. 수정 후 통과. |
| 9 | 대표 screenshot만으로 검증 범위를 과장하는가 | README와 본문은 이미지가 box/background 화면임을 밝히고 glyph paint 결과로 표현하지 않는다. 통과. |
| 10 | 표본 결과가 화면 경계 및 구현 시점을 정확히 설명하는가 | README가 AVD PixelCopy와 iOS drawable 제출 전 Metal readback을 구분하고 20/20씩 기록한다. 통과. |
| 11 | pixel sample 좌표가 root 바깥이라고 오표기됐는가 | root `(0,0) 40×20`, green item `(30,15) 20×10`; `(45,18)`, `(35,22)`, `(45,22)`는 item 내부이면서 root 밖이다. 통과. |
| 12 | 원본 앱 metadata가 다른 코드 상태를 가리키는가 | 두 `simulator.json`은 동일한 implementation commit/tree와 빈 `trackedDirtyPaths`를 기록한다. 초기 미고정 capture는 PR evidence 링크에서 제외했다. 통과. |
| 13 | AVD 설치물이 local build와 다른데 일치했다고 보고하는가 | `installed-apk-verification.json`과 simulator metadata의 APK hash가 같고 README가 같은 hash를 기록한다. 통과. |
| 14 | Simulator runtime이나 target을 실기기로 오인하는가 | evidence는 API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator로만 표현하고 실기기·hardware GPU 제한을 쓴다. 통과. |
| 15 | binary boundary가 release artifact 검증처럼 부풀려졌는가 | README는 debug staticlib의 `strings` 부정/긍정 대조와 정확한 limitation을 기록한다. release/app bundle 검증으로 확장하지 않았다. 통과. |
| 16 | Rust tests 또는 lint 범위가 workspace 전체처럼 보이는가 | PR 본문과 README는 focused tests, fixture-feature Clippy, fixture-boundary test를 나열하고 full-workspace test/Clippy 미실행을 명시한다. 통과. |
| 17 | fixture source가 테스트 전용 파일 경계를 벗어나거나 중복될 수 있는가 | 새 Rust module은 지정 `fixtures/` 및 `*_tests/` 경로에 있고 기존 module root가 `#[path]`로 연결한다. module-boundary 6/6 통과. 통과. |
| 18 | 211→215 scene count와 ID uniqueness가 잘못 바뀌는가 | capture manifest는 새 ID 네 개를 더해 215로 갱신하고 uniqueness assertion을 유지한다. 통과. |
| 19 | 이미지 evidence가 hash·logs·metadata 연결 없이 느슨하게 추가됐는가 | 네 scene screenshot, `result.json`, simulator/app hash, raw runtime·launch logs가 두 final run directory 안에 보존되고 README가 대표 화면 및 SHA를 연결한다. 통과. |
| 20 | PR label 또는 template 요구가 빠졌는가 | 기존 `enhancement`, `area: layout`, `area: testing`, `area: android`, `area: ios`가 PR에 설정됐다. 본문은 상태 ID, 명세 범위, 근거, 제한사항과 화면을 포함한다. 통과. |

## 수정 및 merge gate

- 첫 PR 생성 시 저장소의 area label 이름에 공백이 있는 점을 반영하지 않아 label 추가가 실패했고 PR은 생성되지 않았다. 기존 라벨 이름으로 다시 생성했으며 GitHub PR metadata에서 다섯 라벨을 확인했다.
- PR 본문 초안의 raw image URL에 잘못된 전체 SHA가 있어 Agent Browser가 `naturalWidth=0`을 보였다. 실제 head SHA로 수정하고 PR 본문을 고유 query URL로 새로 열어 두 PNG의 natural dimensions를 확인했다.
- 현재 PR diff의 공백 검사는 통과했다. `gh pr checks 478 --required`는 “no checks reported”를 반환했고 GitHub branch metadata는 protected `main`, 빈 required check 목록, `enforcement_level=off`, required review 없음으로 응답했다. 이 PR에는 보고된 CI run도 없으므로 외부 CI 통과로 표시하지 않는다. mergeability는 `CLEAN`이며 Rebase and merge를 사용한다.

이 변경분은 fixed fixture 결과와 검증 한계를 서로 일관되게 기록한다. C11.4 전체 완료, glyph paint, 실기기, hardware GPU, WPT 또는 iOS compositor 이후 표시를 주장하지 않는다.
