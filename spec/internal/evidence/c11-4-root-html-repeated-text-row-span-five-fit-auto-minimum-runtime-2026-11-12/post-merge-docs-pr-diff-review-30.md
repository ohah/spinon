# C11.4 five-fit row-span 후속 문서 PR diff 실패 관점 검토 30개

범위는 PR #1360 병합 뒤의 계획, 내부 계약, 공식 상태 대장, 실행/병합 기록, 공개 로드맵 동기화다. 기능 구현 자체는 이미 병합된 PR #1360과 별도의 구현/runtime 20개·구현 PR diff 30개 검토 대상이다. 아래 관점은 문서 변경분의 서로 다른 사실·상태·배포 실패 경로를 확인한다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 병합 SHA 오기 | GitHub PR의 merge SHA `6b3475a0ee48741b7f2aaa30e8ee62b19618ef8d`와 일치한다. |
| 2 | PR head 오기 | 병합 전 최종 head `71d2226ddbf8cfd5b5a43fe53880eee8b9b9f24e`를 기록했다. |
| 3 | 병합 시각·방법 오기 | `2026-11-11T16:13:47Z`, Rebase and merge로 일치한다. |
| 4 | 병합 전 base 오기 | `1f63c3148e851f6d00d9163a0fcb9b22486ec79e`로 기록했다. |
| 5 | CI 미설정을 green으로 과장 | workflow run 0, status check 0을 분리해 적고 CI green이 아니라고 했다. |
| 6 | 로컬 통과와 GitHub check 혼동 | local test/build 결과와 원격 check 부재를 다른 문장으로 기록했다. |
| 7 | 병합 후 검토를 병합 전 검토로 둔갑 | 실제 PR diff 실패 관점 검토와 local/remote diff 일치 확인은 병합 전 완료된 사실만 기록한다. |
| 8 | 공식 상태 집계 상승 | 공식 `78/214`를 유지하고 새 완료 표시를 넣지 않았다. |
| 9 | 제한 slice를 C11.4/C11 전체 완료로 오인 | 상위 C11.4/C11 미완료와 unchecked 상태를 유지한다. |
| 10 | 상태 대장 기준일 누락 | 문서 변경일인 `2026-11-12`로 갱신했다. |
| 11 | 계획서 구현 중 라벨 잔존 | PR #1360 병합, SHA와 제한 범위를 상태에 표시한다. |
| 12 | 계획 검증 근거 유실 | Chrome matrix, implementation/runtime 20개 및 PR diff 30개 검토 링크를 보존한다. |
| 13 | 완료된 계획 slice가 여전히 계획 상태라고 표시 | 이 계획의 bounded profile은 구현됐고 남은 범위는 별도 추적한다고 바로잡았다. |
| 14 | 구현 전 결손을 현재 상태로 오해 | 이전 결손이라고 구분하고 PR #1360이 고정 profile을 추가했다고 썼다. |
| 15 | typed cap 값의 순서·종류 손실 | `[0px, 10px, 20%, 30px, 40px]`를 정확히 보존한다. |
| 16 | tail cardinality 범위 확대 | `min-content`, `max-content`, `auto` 중 정확히 하나라고 한정한다. |
| 17 | row-span topology 범위 확대 | 두 고정 topology, source-order pair, alignment를 제한 조건으로 기록한다. |
| 18 | auto-placement와 row-span profile 혼합 | 계획에서 별도 placement 규칙으로 구분한다. |
| 19 | 34,560 matrix 분모 오류 | 네 chunk와 34,560/34,560 scene을 같은 근거와 대조한다. |
| 20 | baseline을 candidate 결과에 합산 | baseline under-floor 건수와 candidate 결과를 별도 cohort로 기록한다. |
| 21 | candidate payload-floor 실패 누락 | candidate 위반 0건으로 기록한다. |
| 22 | 전체 suite·Clippy 통과 과장 | 집중 통과와 runtime `--all-targets`의 기존 lint 5건 실패를 분리한다. |
| 23 | Simulator smoke를 fixture 실행으로 오인 | 앱 화면은 기본 Root HTML route이며 five-fit fixture는 싣지 않는다고 밝혔다. |
| 24 | Android AVD를 실기기·hardware GPU로 표현 | API 37 ARM64 AVD/SwiftShader software 경로로 기록한다. |
| 25 | 화면 캡처를 pixel parity로 확대 | PixelCopy·Metal readback과 compositor 이후 비교는 검증하지 않았다고 썼다. |
| 26 | 실기기·WPT 검증을 암시 | 실기기, hardware GPU와 full WPT는 범위 밖이라고 명시한다. |
| 27 | 화면 이미지 링크 실패·브라우저 잔존 | 네 PR 이미지가 Agent Browser에서 로드됐고 세션이 종료된 사실을 기록한다. |
| 28 | 공개 API/FFI·버전·PR labels 혼동 | 공개 API/FFI 추가 없음, internal `0.1.0` 고정과 세 PR labels를 기록한다. |
| 29 | 로드맵 카드·최근 이력 불일치 | 완료 slice는 진행 칸에서 제거하고 최신 병합 history에 기록하며 다음 후보를 만들지 않는다. 기존 history는 유지한다. |
| 30 | 문서 링크·Tailnet 배포·정리 범위 오류 | 새 상대 링크를 검증하고 Tailnet은 문서 PR 병합 뒤 source/target hash를 대조한다. 공유 simulator·AVD는 보존하고 task-owned 프로세스와 확인된 비활성 temp만 정리한다. |

**결과:** 문서 상태·실행 근거·로드맵 사이의 사실 불일치를 찾지 못했다. 새 후보, 지원 범위 또는 공식 완료를 만들어내지 않는다. Tailnet publication의 실제 source/target hash와 화면 검증은 별도 post-merge 동기화 근거에 기록한다.
