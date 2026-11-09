# PR #1314 병합 후 상태·로드맵·Tailnet diff 검토 30개

이 검토는 PR #1314 병합 뒤의 상태 문서·명세·계획·공개 로드맵과 선택 publication, 서버 복사·browser evidence를 대상으로 한다. 구현 PR의 코드 review 20개와 diff review 30개를 재사용하지 않았다.

| # | 독립 실패 경로 | 판정 |
|---:|---|---|
| 1 | 잘못된 구현 PR 번호를 병합 근거로 기록 | PR #1314 링크와 제목이 GitHub PR 정보와 일치한다. |
| 2 | 잘못된 UTC 병합 시각 기록 | `2026-11-09T17:54:36Z`가 PR merged timestamp와 일치한다. |
| 3 | rebase 이후 source SHA를 merge SHA로 혼동 | `352e6c0b...`를 merge SHA로 표기했다. |
| 4 | main HEAD가 병합을 포함하지 않는데 반영 완료로 표기 | post-merge `origin/main`이 merge SHA를 가리키는 것을 확인했다. |
| 5 | status rollup이 비었는데 CI 성공으로 기록 | combined status에 check가 없어 CI 통과 문구를 두지 않았다. |
| 6 | 제출 review가 없는데 승인됐다고 기록 | PR reviews가 비어 있음을 확인하고 승인 주장을 하지 않았다. |
| 7 | bounded slice 병합을 공식 C11 완료로 확대 | C11과 C11.4 공식 checkbox를 미완료로 유지했다. |
| 8 | 제한 구현 하나를 공식 완료 숫자에 더함 | STATUS·계획·로드맵 모두 `78/214`를 유지한다. |
| 9 | 내부 숫자 계약 버전이 구현 범위와 함께 증가 | 계약 `0.1.0`을 유지했다. |
| 10 | plan의 상태가 계속 merge 전으로 남음 | plan 상단·결과 단락에 PR #1314와 merge SHA를 반영했다. |
| 11 | plan이 검증하지 않은 일반 CSS를 지원한다고 암시 | fixed Root HTML direct-text profile 경계를 유지했다. |
| 12 | 두 fit 위치 cardinality를 잘못 기록 | 서로 다른 위치 1–6의 15쌍과 12 tail order가 일치한다. |
| 13 | 나머지 track multiplicity를 부정확하게 요약 | `min-content ×2`, `max-content ×1`, `auto ×1`로 기록했다. |
| 14 | 두 cap의 typed grammar를 확장 해석 | finite non-negative direct px/%만으로 범위를 고정했다. |
| 15 | row-span profile이 auto-placement와 합쳐졌다고 암시 | plan과 계약에서 row-span topology를 별도 profile로 남겼다. |
| 16 | Chrome 사전 비교를 RuntimeSession 증거로 대체 | Chrome과 RuntimeSession의 2,880/2,880을 별도 근거로 표기했다. |
| 17 | trace failure guard를 검증 없이 통과 처리 | trace unit tests 4/4와 실패 경계를 연결했다. |
| 18 | 과거 회귀 denominator를 줄이거나 합산 | 810, 1,440, 3,840, 1,920, 4,320 scene 수를 각각 보존했다. |
| 19 | 시뮬레이터 화면을 native pixel parity로 확대 | visual smoke 및 PixelCopy/Metal 한계를 명시했다. |
| 20 | 물리 기기·hardware GPU·WPT 결과를 암시 | 남은 범위에 세 항목을 제외한다고 기록했다. |
| 21 | implementation README의 diff-check 주장이 raw log와 모순 | source/docs 검사와 raw platform log의 trailing space를 구분해 정정했다. |
| 22 | 내부 계약 색인 링크가 구현 artifact를 가리키지 않음 | 계획·사전 비교·실행 README·20/30 review 경로를 확인했다. |
| 23 | 공개 로드맵 최신 카드가 “PR 준비 중”으로 남음 | heading과 progress card를 PR #1314 merged로 갱신했다. |
| 24 | 공개 카드의 local route가 선택 publication에서 빠짐 | plan·precomparison·implementation README를 RSPress route manifest에서 확인했다. |
| 25 | publication에서 기존 시작/404 페이지를 삭제 | 기존 `index.html`과 `404.html`을 제외하고 route 삭제를 하지 않았다. |
| 26 | preview 서버가 꺼져 있거나 예상 외 listener로 교체 | Tailscale Running, 예상 server script와 `127.0.0.1:8768` listener를 확인했다. |
| 27 | 교체 전에 다른 프로세스가 route를 읽는 중인데 덮어씀 | 실패한 첫 preflight에서 멈췄고 재시작 후 open-handle 0개를 검사했다. |
| 28 | 이전 제공 파일이 백업 없이 교체 | 교체 20 route 모두 지정 backup root에 저장하고 hash를 재검증했다. |
| 29 | HTTPS 경로가 새 hash를 제공하지 않음 | 93/93 route의 HTTPS 200과 SHA-256을 검증했다. |
| 30 | browser·simulator 임시 리소스가 작업 뒤 남음 | task Agent Browser 세션을 닫고 `session list`가 비었음을 확인했다. 기존 Terminal Browser sidecar는 해당 사용자 세션 소속이라 보존했으며 S03/booted iOS/ADB 기기는 없었다. |

### 남은 경계

이 검토는 병합 후 문서와 사용자가 지정한 preview의 제한된 route/hash 상태를 확인한다. PR check rollup, 일반 CSS Grid 적합성, simulator compositor 이후 pixels 또는 공개 Pages 배포를 주장하지 않는다.
