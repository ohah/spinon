# PR #1369 이후 상태·계약·로드맵 문서 PR diff 검토 30개

검토 범위는 `spec/STATUS.md`, 내부 계약 0059, mixed-minimum 집중 계획, `plan/c11-4-grid-track-sizing.md`, 공개 roadmap HTML, PR #1369 merge metadata와 Tailnet 미리보기 동기화 도구다. 각 관점은 별도 오류 경계를 확인한다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 구현 PR head SHA와 GitHub merge SHA를 바꾸어 기록 | PR metadata와 대조: head `95a20ea…`, merge `79cc632…`로 분리했다. |
| 2 | merge 시간을 local timezone으로 잘못 표기 | GitHub의 `2026-11-12T01:02:45Z`를 그대로 기록했다. |
| 3 | GitHub workflow가 없는데 CI 성공으로 표현 | workflow runs 0개와 commit status checks 0개를 명시했다. |
| 4 | C11.4 상위 체크를 완료로 변경 | `spec/STATUS.md`의 checkbox는 unchecked로 보존했다. |
| 5 | C11 전체 checkbox나 완료 합계가 증가 | C11 미완료와 공식 `78/214`를 모두 유지했다. |
| 6 | 숫자 버전 변경을 구현 범위 확장으로 정당화 | 내부 계약 `0.1.0`을 유지했다. |
| 7 | mixed minimum을 일반 Grid admission으로 과장 | exact six-fit repeated-text row-span 경계와 plan 링크를 명시했다. |
| 8 | six-fit auto-placement를 row-span으로 혼동 | track-sizing plan이 두 profile을 분리한다. |
| 9 | explicit-row control도 `auto`를 허용한다고 오독 | 각 계약·로드맵에서 control `0px` 고정을 명시했다. |
| 10 | 720 cap order를 scene 수와 혼동 | 720 cap order와 138,240 scene을 별도 적었다. |
| 11 | 두 topology의 mask 수를 서로 바꾸어 기록 | bounded plan의 32/16 mask 정의를 그대로 사용했다. |
| 12 | source-order 또는 alignment 조합을 matrix에서 제외 | plan/evidence의 forward·reverse 및 start·stretch 범위를 반영했다. |
| 13 | uniform six-fit control을 누락 | 11,520 scene regression을 기록했다. |
| 14 | auto-placement 또는 three-fit 회귀 수를 mixed matrix에 합산 | 두 regression을 각각 5,760 및 11,520으로 분리했다. |
| 15 | Browser oracle 관측을 Spinon 실행 결과로 표시 | Chrome 사전 비교와 RuntimeSession 실행을 별도 문서 링크로 유지했다. |
| 16 | RuntimeSession raw track trace가 Chrome used-row pixel equality를 증명한다고 주장 | 계획과 runtime evidence의 trace 해석 한계를 유지했다. |
| 17 | Android/iOS 화면을 mixed feature fixture 결과로 오해 | 기본 Root HTML boot smoke라고 명시했다. |
| 18 | 화면 smoke를 PixelCopy/Metal drawable 또는 compositor parity로 확대 | surface readback과 pixel parity는 미확인으로 기록했다. |
| 19 | Android software renderer / iOS simulator를 실기기로 표기 | AVD와 iOS Simulator 기종을 정확히 표시했다. |
| 20 | strict Clippy 실패를 숨기거나 전체 실패를 변경 탓으로 귀속 | 기존 test-helper lint 6건과 좁은 재검사를 분리했다. |
| 21 | broad crate suite가 전부 통과했다고 기술 | baseline과 동일한 broad-suite 실패를 유지했다. |
| 22 | stale revision rejection만 적고 recovery 보장을 생략 | atomic rejection과 same-session recovery를 모두 적었다. |
| 23 | unsupported input 경계가 generic CSS까지 확장됨 | cap·topology·child·text grammar 경계를 그대로 제한했다. |
| 24 | stale plan 문구가 mixed 구현 미착수로 남음 | mixed plan의 상단 상태와 결과 절을 병합 상태로 갱신했다. |
| 25 | track-sizing overview가 #1365 이후 다음 후보를 mixed로 남김 | #1369 완료와 이후 후보 미선정을 분리해 갱신했다. |
| 26 | Tailnet latest summary와 동적 C11.4 NEXT card 중 한 곳이 #1365를 계속 표시 | 첫 Agent Browser 확인에서 stale card를 발견해 원본 stage를 #1369 merged로 고치고 최종 preview에서 재확인했다. |
| 27 | roadmap 상세 note에서 PR, SHA, 검증 범위 불일치 | PR URL·merge SHA·scene 수를 같은 값으로 맞췄다. |
| 28 | roadmap route에 새로 추가된 핵심 문서 링크가 빠짐 | 계획, 계약, 사전 비교, 실행·merge evidence, 공식 상태 링크를 포함했다. |
| 29 | Rspress 전체 빌드나 전체 dead-link 검사를 수행한 것처럼 주장 | Tailnet 작업은 선택 경로 빌드로 한정하고 전체 검사는 false로 기록한다. |
| 30 | preview 복사 중 기존 사용자 파일·서버·백업을 덮거나 삭제 | sync 도구가 symlink를 거부하고 변경 파일만 고유 백업 후 atomic replace하며 삭제하지 않도록 검토했다. |

문서 수치와 실행 경계는 PR #1369 본문, merge metadata, mixed plan/runtime evidence를 교차 확인했다. 이 검토는 PR 승인·CI 결과 또는 전체 CSS Grid 적합성을 대신하지 않는다.
