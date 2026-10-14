# C13.1 iOS 앱 hit identity grid 계획 PR 변경 관점 검토 20개 · 2026-10-15

## 변경 검토 대상

이번 계획 PR diff: 새 iOS Simulator 앱 hit-grid 계획, C13.1 계획의 진행 상태, `spec/STATUS.md`의 미완료 범위 연결, 계획 검토 기록이다. 이전 Android 구현의 리뷰를 재사용하지 않는다.

## 서로 다른 변경 실패 관점 20개

| # | PR 변경 실패 관점 | diff 검토 결과 |
| --- | --- | --- |
| 1 | 새 계획이 기존 C13.1 계획 범위를 암묵적으로 완료 처리 | 목표·제외에서 C13.1 전체와 공개 CSS 지원을 미완료로 고정했다. |
| 2 | Android #164의 증거를 iOS 통과로 합산 | AVD baseline과 향후 iOS Simulator 결과를 분리한다고 명시했다. |
| 3 | Chrome 기준 버전 또는 revision이 누락·변경 | Chrome 155.0.8059.40, 전체 revision과 agent-browser 0.31.1을 기록했다. |
| 4 | grid JSON / observation / identity SHA가 다른 기준 자료를 가리킴 | 세 해시와 fixture HTML/JS 해시를 기존 계획·Android evidence와 대조했다. |
| 5 | CSS viewport, DPR, scroll origin을 달리해 다른 좌표 grid를 비교 | `300×100`, DPR 1, scroll `(0,0)`, integer `(x,y)` 기준을 명시했다. |
| 6 | 870 RLE run이나 41 Node ID 순서를 잃어 query 일부를 검증 | run 수·row coverage·node count를 계획에 기록하고 Rust validator를 재검증한다. |
| 7 | `52 sample`과 `30,000 full grid`를 같은 숫자로 혼동 | pass criteria와 로그에서 두 결과를 각각 기록한다. |
| 8 | iOS FFI report에 grid marker가 없는데 parser가 성공 | missing/truncated marker를 parser negative control로 추가했다. |
| 9 | plan에서 실제 존재하지 않는 새로운 Rust API를 구현 요구 | 현재 FFI summary와 host를 재사용하고 API·payload·symbol 추가를 범위 밖으로 했다. |
| 10 | argument 조합 충돌을 앱이 임의로 우선순위 처리 | host 생성 전 route collision 거부를 요구한다. |
| 11 | summary count는 맞지만 digest가 변조된 결과를 성공 | expected·actual identity 및 grid·observation digest 네 필드와 zero mismatch를 동시에 pass 조건으로 고정했다. |
| 12 | mismatch preview가 큰 입력으로 report/log를 폭증 | 16개 preview 상한과 bounded error logging을 기록했다. |
| 13 | 각 run에서 generation 번호가 프로세스 재시작 뒤 재사용될 수 있음 | generation 숫자 차이를 실행 간 요구하지 않고 새 run ID로 로그를 구분하도록 정정했다. |
| 14 | app status `status=ready`가 실제 FFI report와 다른 표현 | `status=0 layout=ready`와 41 frame count를 명시했다. |
| 15 | iOS screenshot을 framebuffer exact 또는 compositor 결과로 오해 | screenshot은 화면 증거이며 Metal readback은 제외한다고 계획에 적었다. |
| 16 | 계획 PR 본문에 새 기능 screenshot이 있어 실제 실행 증거로 보일 위험 | 계획은 동작 변경을 만들지 않으며 실행 화면은 이후 기능 PR에서 올리도록 구분했다. |
| 17 | `STATUS.md`가 iOS 전체 runtime을 여전히 미검증이라고 잘못 표현 | 기존 41/41·52/52 실행을 보존하고 남은 항목을 iOS full-grid query로 한정했다. |
| 18 | C13 계획의 진행 문구가 Android full-grid도 미완료라고 남음 | PR #164 이후 상태를 갱신하고 iOS app full-grid 계획 링크를 추가했다. |
| 19 | local dirty C11/C12 작업 tree와 이 PR의 C13 docs가 섞임 | 최신 `origin/main`에서 만든 독립 worktree의 계획/status 파일만 변경했다. |
| 20 | 사용자 기기 요청으로 오해해 physical-device gate 추가 | 지정 Simulator만 실행하고 실기기는 사용하지 않는다고 명시했다. |

## 판정

계획과 공식 상태 문구는 서로 일치하며, 기존 Android 결과·고정 Chrome oracle·아직 실행하지 않은 iOS Simulator gate를 분리한다. 실제 iOS 앱 코드, build, simulator run은 이번 계획 PR diff에 포함되지 않는다. 기능 구현에는 새 구현 관점 20개와 PR delta 관점 20개가 필요하다.
