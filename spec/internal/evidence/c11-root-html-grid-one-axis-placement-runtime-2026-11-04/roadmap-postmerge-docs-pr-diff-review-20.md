# C11.1 한 축 배치 병합 후 문서 PR diff 적대 검토 20개

검토 대상은 PR #1142 병합 뒤 계획, 공식 상태, 내부 계약, runtime evidence README를 갱신하는 문서 변경과 이 검토 기록이다. 라이브 GitHub 상태와 이미 보존된 실행 evidence를 대조했다. 문서 전용 변경이므로 Rust·Android·iOS 검증을 다시 실행하지 않았다.

| # | 적대 관점 | 확인한 근거와 판정 |
| --- | --- | --- |
| 1 | 병합 뒤 문서 변경에 무관한 구현 파일이 들어왔는가 | 변경 경로는 전용 계획, spec/STATUS.md, 계약 0059, C11.1 runtime README 및 이 review 기록으로 제한한다 — 통과. |
| 2 | 기록된 PR 번호와 merge state가 실제 GitHub 상태와 일치하는가 | PR #1142 조회 결과 state=MERGED, base=94afecef, head=521741d7, merge SHA=0f550642다 — 통과. |
| 3 | 기록된 merge SHA가 origin/main의 실제 tip인가 | fetch한 origin/main과 GitHub main API가 모두 0f5506423a1150c80e5684fd087b948923fdbcb7을 가리킨다 — 통과. |
| 4 | UTC merge 시각 또는 병합 방법을 잘못 표기하는가 | GitHub mergedAt은 2026-11-04T02:24:11Z이고 사용한 방식은 Rebase and merge다 — 통과. |
| 5 | PR head commit과 merge commit을 혼동하는가 | status·plan·contract는 구현 head 521741d7b533adbe6a08b531f5b4e8aaaa6c91ad와 merge SHA를 각각 분리해 기록한다 — 통과. |
| 6 | 빈 checks 결과를 CI 성공으로 바꾸어 말하는가 | PR check rollup은 빈 배열이고 main required status checks endpoint는 미설정 응답을 반환한다. 문서는 CI 통과를 주장하지 않는다 — 통과. |
| 7 | 승인 review 요구가 있었는데 생략되었다고 기록하는가 | main required approving review count는 0이며 PR에는 review request가 없다 — 통과. |
| 8 | 제한 구현 병합이 공식 C11 완료로 승격되는가 | spec/STATUS와 계약은 상위 C11.1/C11 checkbox를 그대로 두고 제한 구현으로만 설명한다 — 통과. |
| 9 | 공식 완료 수가 병합에 따라 증가했는가 | 변경된 상태 기록은 공식 72/213을 유지하고, 공식 카운트 변경 diff는 없다 — 통과. |
| 10 | 내부 계약 숫자 버전이 무단으로 올라가는가 | 계약 0059는 0.1.0으로 유지되며 behavior/API version 변경은 없다 — 통과. |
| 11 | 계획에 구현 전 baseline 재실행 여부를 숨기는가 | 계획 상태는 구현 전 제품 baseline 재실행이 기록되지 않았음을 그대로 보존한다 — 통과. |
| 12 | 계획이 PR review 또는 merge가 아직 남았다고 표시하는가 | 계획 상태는 PR #1142의 20-point diff review와 merge SHA를 기록하고 pending 문구를 제거했다 — 통과. |
| 13 | Chrome 사전 비교 건수나 오차를 바꾸어 기록하는가 | 저장된 precomparison evidence는 14 scenes, 41 item nodes, 40 frames, 8 accepted, 5 expected rejection, all-auto control 1, 최대 0 CSS px다 — 통과. |
| 14 | Rust 검증 수가 실행 기록과 어긋나는가 | 보존 로그는 layout 5/5, style-to-layout 6/6, RuntimeSession 2/2 및 sparse/dense 회귀 77/77을 기록하며 문서 수치와 일치한다 — 통과. |
| 15 | Android 표본 결과를 hardware device로 과장하는가 | PixelCopy 6/6은 API 37 ARM64 AVD·ANGLE/Vulkan/SwiftShader로 한정해 썼고 실기기/hardware GPU를 주장하지 않는다 — 통과. |
| 16 | iOS drawable 표본을 compositor 후 화면으로 부풀리는가 | Metal 6/6은 iPhone 17 Pro/iOS 26.2 Simulator drawable, compositor 제출 전 readback으로 명시한다 — 통과. |
| 17 | 기본 artifact의 fixture 경계를 모든 플랫폼에서 입증한 것처럼 쓰는가 | evidence README는 Android APK와 iOS app의 전용 marker/provider/payload/FFI 부재 기록 및 범위를 지칭한다 — 통과. |
| 18 | 일반 CSS·WPT 적합성과 전체 C11 지원을 암시하는가 | 계획·0059·상태 모두 일반 CSS admission, WPT 적합성, 전체 C11을 검증하지 않았다고 남긴다 — 통과. |
| 19 | 계획·상태·계약의 상대 링크가 잘못되거나 삭제된 파일을 가리키는가 | 계획, comparison/runtime evidence, implementation review, PR diff review 및 계약 링크의 대상 파일이 존재한다. PR 링크는 #1142에 고정돼 있다 — 통과. |
| 20 | 이 문서 review가 누락되거나 자체 기록과 변경 파일 수가 어긋나는가 | status line과 이 표가 병합 SHA·한계·20개 판정을 포함하며 변경 전용 review 파일을 보존한다. whitespace 검사도 통과했다 — 통과. |

## 판정

문서의 merge state, official status, evidence 수치와 미검증 한계가 일치한다. CI 성공이나 C11 완료를 주장하지 않는다. 이 검토는 코드 재검토, 새 Rust test run 또는 추가 simulator/device gate가 아니다.
