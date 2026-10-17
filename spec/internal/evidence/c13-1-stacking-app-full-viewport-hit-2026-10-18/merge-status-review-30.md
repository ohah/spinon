# PR #287 병합 상태와 문서 동기화 검토 30개

2026-10-18 KST main 상태를 대상으로 PR #287 Rebase and merge의 source tree·문서 상태·범위 기록을 점검했다. 이 검토는 구현 검토 20개와 PR 변경 검토 30개를 재사용하지 않으며, 병합 후 상태 동기화에서 생길 수 있는 별도 실패를 확인한다.

| # | 검토 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 병합된 PR 번호가 실제 변경 PR을 가리키는가 | 통과. 제목과 PR #287의 source diff가 구현 및 evidence 변경이다. |
| 2 | PR 상태가 실제로 MERGED인가 | 통과. GitHub API에서 state=MERGED다. |
| 3 | PR 병합 방식이 저장소 정책과 맞는가 | 통과. PR은 Rebase and merge 방식으로 선형 이력에 반영됐다. |
| 4 | merge SHA가 원격 main HEAD와 맞는가 | 통과. merge SHA 392bfb19c958b80e16a615286896c09498e200d3가 origin/main HEAD다. |
| 5 | 병합 시간이 UTC/KST로 혼동되는가 | 통과. GitHub의 2026-10-17 19:58:44 UTC는 문서의 2026-10-18 KST와 맞는다. |
| 6 | rebased 구현 commit이 실행 source와 다른 코드를 가리키는가 | 통과. 1375b797과 45c93688의 tree가 모두 a63633ca94f0c191a0f135343c1a86a7748bfc3b다. |
| 7 | 원본 runtime evidence의 sourceRevision이 새 SHA로 덮이는가 | 통과. 실제 측정 당시 sourceRevision 1375b797을 보존하고 rebase mapping은 별도로 적는다. |
| 8 | 문서가 main의 latest merge SHA를 잘못 가리키는가 | 통과. plan, internal index, contract가 동일 SHA를 사용한다. |
| 9 | 계획 상태에 merge 대기가 남는가 | 수정. plan header를 PR #287 병합 완료로 바꾸고 상위 범위 미완료를 병기한다. |
| 10 | 계획 단계의 마지막 작업이 완료 후에도 미래형으로 남는가 | 수정. 순서 6에 실제 rebase merge 결과와 SHA를 기록했다. |
| 11 | STATUS가 병합 때문에 C13.1 전체를 체크하는가 | 통과. 하위 fixture 결과만 적고 상위 C13.1/C13 checkbox는 미체크다. |
| 12 | 완료 총계가 하위 검증으로 오르는가 | 통과. 상태 대장의 기존 완료 체크 수를 수정하지 않는다. |
| 13 | PR 병합 링크가 모든 문서에서 같은 PR을 가리키는가 | 통과. plan, internal index, contract가 ohah/spinon/pull/287을 연결한다. |
| 14 | PR 본문 화면 검증과 source evidence가 사라지는가 | 통과. 기존 final Android/iOS evidence와 PR 이미지 확인 기록을 유지한다. |
| 15 | Android AVD가 물리기기로 과장되는가 | 통과. emulator-5554, API 37 ARM64 QEMU AVD라고 계속 표기한다. |
| 16 | iOS Simulator가 실기기로 과장되는가 | 통과. iPhone 17 Pro/iOS 26.2 Simulator라고 계속 표기한다. |
| 17 | 18개 표면 표본이 전체 surface parity로 잘못 서술되는가 | 통과. PixelCopy/Metal RGBA는 각각 18개 고정 sample로 구분한다. |
| 18 | 69,120개 hit identity가 physical pixel count로 오해되는가 | 통과. 540×128 integer CSS coordinate query로 고정해 표기한다. |
| 19 | fractional edge 경계가 일반 hit-test 지원 범위로 확장되는가 | 통과. viewport visibility rounding 변경과 original float geometry query를 별도로 적는다. |
| 20 | default/opt-in artifact 경계 결과가 제품 bundle 검증으로 과장되는가 | 통과. static spinon-ffi archive 한정임을 보존한다. |
| 21 | stale revision control이 성공 surface capture로 오해되는가 | 통과. PixelCopy/Metal readback 전 거부로 기록한다. |
| 22 | route collision control이 runtime-host 실패로 늦게 분류되는가 | 통과. host init 이전 거부로 기록한다. |
| 23 | Chrome oracle-only screenshot이 앱 화면으로 오해되는가 | 통과. Chrome 기준 화면과 Android/iOS app screen을 분리한다. |
| 24 | PR screenshot이 브랜치 상대 URL의 merge 후 404가 되는가 | 통과. PR 본문은 raw branch URL을 사용하고 source branch는 remote에 보존돼 있다. |
| 25 | 내부 계약 0078이 public API로 표시되는가 | 통과. 숫자 버전 0.1.0인 fixture-only 내부 계약이라는 경계를 유지한다. |
| 26 | renderer rounding fix가 전체 geometry rounding으로 오해되는가 | 통과. visibility containment만 round하고 hit geometry 좌표는 float로 유지한다고 쓴다. |
| 27 | 전체 workspace test 또는 WPT가 실행됐다고 주장되는가 | 통과. PR 본문과 evidence 모두 미실행을 표시한다. |
| 28 | CI 없음이 CI 통과로 오해되는가 | 통과. PR #287에는 status check가 보고되지 않았고 branch protection은 필수 check 0건이었다. |
| 29 | 병합 상태 점검이 새 제품 완료 판정으로 승격되는가 | 통과. 문서 동기화 PR이며 C13.1/C13 상태는 바꾸지 않는다. |
| 30 | Tailnet preview sync를 이 문서 PR의 git 변경만으로 완료했다고 오해하는가 | 통과. 외부 preview는 별도 manifest와 served hash·HTTP·Agent Browser 확인으로 검증하며 GitHub Pages는 배포하지 않는다. |

## 판정

PR #287 병합 상태와 계획/인덱스 참조는 서로 일치한다. 이 변경은 문서 상태만 정리하며 런타임이나 완료 count를 변경하지 않는다. Tailnet preview는 별도 절차에서 PR merge source, 파일 hash, HTTP 응답과 Agent Browser 표시로 검증한다.
