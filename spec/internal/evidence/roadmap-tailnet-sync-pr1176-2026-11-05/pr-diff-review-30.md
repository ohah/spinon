# PR #1176 병합 후 문서·Tailnet 동기화 변경 검토 30개

검토 대상은 C11 계획, 공식 상태 대장, 내부 계약 0059, runtime evidence README, 공개 roadmap 및 선택 route 동기화 evidence다. 이 검토는 구현 PR #1176의 코드 검토를 대신하지 않는다.

| # | 독립 검토 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 원 PR의 병합 상태가 열림/닫힘 표기와 맞는가 | PASS — #1176은 GitHub에서 MERGED다. |
| 2 | merge SHA가 정확한가 | PASS — 문서와 카드가 `da67a18032f449d8477cdd5d7ea0aba02e691b0b`로 일치한다. |
| 3 | merge 시각이 시간대와 함께 정확한가 | PASS — `2026-11-05T06:26:10Z`를 UTC로 적었다. |
| 4 | 문서 동기화 기준 tree가 병합 직후 main인가 | PASS — `HEAD`와 `origin/main`은 모두 merge SHA다. |
| 5 | merge metadata가 각 상태 문서에서 교차 일치하는가 | PASS — 계획, STATUS, 계약 0059, runtime README, roadmap 카드가 같다. |
| 6 | 상위 CSS Grid 완료를 과장했는가 | PASS — C11 checkbox는 미완료로 남았다. |
| 7 | C11.1/C11.4의 제한 slice를 전체 완료로 승격했는가 | PASS — 두 항목의 미완료 문구를 유지했다. |
| 8 | 전체 공식 집계가 달라졌는가 | PASS — `78/214`를 유지했다. |
| 9 | 비어 있는 GitHub checks를 CI 성공이라고 했는가 | PASS — check rollup과 required check 부재를 명시했다. |
| 10 | 내부 계약의 숫자 버전이 임의로 올랐는가 | PASS — `0.1.0` 고정을 유지한다. |
| 11 | 지원 범위가 기존 runtime profile 밖으로 넓어 보이는가 | PASS — definite fixed-axis `RootHtmlIntrinsicTrackSizingV1`로 한정했다. |
| 12 | direct percentage의 유효 범위를 정확히 표현했는가 | PASS — finite non-negative 값으로 한정했다. |
| 13 | row/column percentage basis가 뒤바뀌었는가 | PASS — row gap은 height, column gap은 width다. |
| 14 | shorthand와 alias 적용 범위를 오해하게 적었는가 | PASS — computed longhand 경로와 `grid-gap` alias만 기록했다. |
| 15 | 100% 초과 gap의 visible overflow를 clamp한다고 표현했는가 | PASS — 120% overflow를 보존한다고 기록했다. |
| 16 | unsupported computed value와 수치 overflow 실패를 감췄는가 | PASS — fail-closed 및 원자 실패 동작이 계획과 evidence에 남아 있다. |
| 17 | 독립 기대값과 Chrome comparison scene 수가 일치하는가 | PASS — 9 scene을 기록했다. |
| 18 | focused layout 테스트 수가 근거 로그와 맞는가 | PASS — 8/8이다. |
| 19 | focused Root HTML runtime 테스트 수가 근거 로그와 맞는가 | PASS — 11/11이다. |
| 20 | fixture module 경계 결과를 과장했는가 | PASS — 33/33으로 기록했다. |
| 21 | broad layout의 기존 실패를 숨겼는가 | PASS — 후보 449/450과 baseline 447/448의 동일 C15 assertion을 적었다. |
| 22 | runtime suite의 기존 실패를 새 회귀처럼 보이게 했는가 | PASS — 후보 276/8/3, 기준 275/8/3 및 동일 실패 집합을 분리했다. |
| 23 | Android AVD 결과를 실기기 GPU로 확대 해석했는가 | PASS — ARM64 AVD/SwiftShader라고 제한했다. |
| 24 | iOS 표본을 compositor 이후 결과로 확대했는가 | PASS — Metal drawable readback을 하지 않았다고 명시했다. |
| 25 | simulator 화면을 pixel parity 증거라고 불렀는가 | PASS — visual smoke 한계를 표시했다. |
| 26 | WPT나 일반 CSS admission 지원을 주장했는가 | PASS — 둘 다 미완료로 남겼다. |
| 27 | roadmap의 plan/evidence 링크가 선택 publication에 존재하는가 | PASS — build manifest에서 #1176의 local route와 SHA를 확인했다. |
| 28 | 선택 빌드를 전체 문서 링크 검증으로 오해할 수 있는가 | PASS — pinned RSPress `2.0.22`, 선택 route 범위, dead-link 검사 비활성화를 README에 밝혔다. |
| 29 | 복사 전 대상의 상태와 rollback 경로를 확인했는가 | PASS — 교체 route 열린 handle 0, 변경 전 hash 및 backup hash를 기록했다. |
| 30 | Tailnet 게시 후 화면·해시·프로세스 경계를 확인했는가 | PASS — local/HTTPS route hash, desktop/mobile 화면, 가로 넘침 없음, browser 종료, preview PID 유지와 no-delete를 기록했다. |

**결론:** 이 문서 변경은 #1176의 병합 사실과 검증 한계를 반영하며 C11 완료 상태나 공식 집계를 바꾸지 않는다. 선택한 Tailnet route 동기화는 SHA-256 검증을 통과했다. 발견한 차단 결함은 없다.
