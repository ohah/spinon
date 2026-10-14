# C13.1 iOS Simulator 앱 hit identity grid 계획 실패 관점 검토 · 2026-10-15

## 검토 대상

[iOS app hit identity grid 계획](../../../plan/c13-1-ios-app-hit-identity-grid.md), 기존 [Android 앱 계획](../../../plan/c13-1-android-avd-app-hit-identity-grid.md), 고정 Chrome grid oracle와 [현재 iOS compact report 경로](../../../platforms/ios/Sources/SpinonRunner.mm)를 대조했다. 이 검토는 구현 전 계획의 누락·모순 검토이며 iOS runtime 통과 결과가 아니다.

## 서로 다른 실패 관점 20개

| # | 실패 관점 | 계획 검토와 반영 |
| --- | --- | --- |
| 1 | Rust FFI가 이미 전체 grid를 계산하는데 iOS에서 계산을 재구현해 oracle 해석이 달라짐 | 계획은 기존 FFI summary를 검증하며 Rust 비교 알고리즘·scene을 수정하지 않도록 범위를 고정했다. |
| 2 | iOS bridge가 grid summary를 전달하지 않아 앱 parser가 빈 성공을 기록 | `hit_identity_grid_summary_json` marker가 없거나 잘리면 실패하도록 통과 기준과 negative control을 추가했다. |
| 3 | report 뒤에 52-sample array가 붙는데 `{` 다음 첫 `}`로 잘라 preview object를 손상 | 중첩 object와 문자열 escape를 고려한 구조 추출을 요구한다. |
| 4 | Swift `NSNumber`가 boolean·소수·문자열을 정수 count로 받아들임 | JSON 수치 타입과 정수성을 확인하고 bool/소수/문자열 mutation을 거부하도록 계획에 명시했다. |
| 5 | `matched_count + mismatch_count`가 point count와 다르지만 `matched=30000`만 확인 | 정확한 count 합, 범위, status를 독립 확인하도록 pass criteria를 분리했다. |
| 6 | oracle digest만 일치하고 runtime identity가 다름 | expected·actual identity SHA를 별도로 비교하고 실제 mismatch count가 0이어야 성공한다. |
| 7 | `status=mismatch`인데 count만 정상이라 성공으로 처리 | `status=matched`, 30,000 match, mismatch 0, empty preview를 모두 요구한다. |
| 8 | missing hit이 root NodeId fallback으로 바뀌어 실제 `None` mismatch를 숨김 | 이 단계는 FFI의 기존 actual identity SHA와 mismatch count를 그대로 확인하며 새 fallback을 추가하지 않는다. |
| 9 | frame preorder와 Chrome node registry가 다르지만 같은 integer index로 우연히 합격 | oracle node order SHA와 identity SHA를 고정하고 기존 Rust scene validation을 재사용한다. |
| 10 | 30,000 정수 좌표 결과를 fractional edge 또는 CSSOM event 계약으로 확대 | `elementFromPoint` integer grid 범위와 event/fractional 제외를 목표·제외에 반복 명시했다. |
| 11 | report preview의 누락·범위 밖 좌표·과도한 항목으로 parser가 무제한 메모리나 로그를 사용 | preview 최대 16, 좌표 경계, parser mutation, 요약 1줄+최대 16건 로그를 요구한다. |
| 12 | 30,000-point evaluation이 UI thread를 막아 검증 화면이 정지하거나 상태가 늦게 나타남 | 기존 앱 evaluation lane을 사용하고 실행 후 report와 screenshot을 같이 보존하는 조건을 추가했다. |
| 13 | 이전 앱 실행의 로그를 새 실행 결과에 붙여 30,000/30,000을 주장 | 각 route launch에 새 run ID를 만들고 모든 marker에 generation/run ID를 연결한다. |
| 14 | route argument 중복으로 다른 fixture host와 compact host가 함께 만들어짐 | 단독 route를 두고 C13/C11 fixture argument 충돌은 host 생성 전에 거부한다. |
| 15 | compact route의 52 fixed sample이 깨졌지만 full grid summary만 통과 | 41 frame와 기존 52 sample을 계속 검사하고 각 결과를 별도 marker로 유지한다. |
| 16 | iOS Simulator 모델/OS/feature 조합이 달라 이전 캡처와 같은 시험인 것처럼 보고 | iPhone 17 Pro / iOS 26.2 Simulator 및 opt-in feature를 고정하고 run metadata를 보존한다. |
| 17 | 화면 screenshot을 Metal drawable pixel readback으로 오해 | screenshot을 앱 동작 상태 증거로만 쓰고 drawable readback은 명시적으로 제외했다. |
| 18 | 앱에 fixture route 문자열이 있다는 이유로 제품 CSS/API 지원을 선언 | 신규 FFI/profile을 만들지 않고 `*-fixture-test` compile 경계를 유지하며 C13.1 상태는 미완료로 남긴다. |
| 19 | 한 번의 실행을 안정성 행렬이나 실기기 결과로 일반화 | 세 번의 고유 run ID를 요구하되 결과는 지정 Simulator에 한정한다. 실기기는 요구하지 않는다. |
| 20 | PR screenshot/문서가 runtime grid success와 모순 | status, bounded summary, evidence screenshot과 logs를 같은 run ID로 연결하고 PR image를 browser에서 검증한다. |

## 판정

고정 oracle, 기존 FFI와 iOS app의 연결 차이, parser 경계, 실행 ID·generation, Simulator 한계 및 PR 증거 기준이 계획에 분리됐다. implementation review와 PR delta review는 별도 수행한다. 이 문서는 iOS 앱 검증을 실행하거나 C13.1 gate를 완료한 근거가 아니다.
