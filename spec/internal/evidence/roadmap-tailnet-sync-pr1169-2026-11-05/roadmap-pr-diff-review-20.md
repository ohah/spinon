# C11.1 PR #1167 공개 로드맵 및 Tailnet 동기화 PR diff 적대 검토 20개

검토 대상은 `spec/public/roadmap.html`의 #1167 카드와 선택 publication evidence다. PR #1167 구현 diff 검토 및 count correction 검토는 재사용하지 않고 공개 상태·route·호스트 검증 실패 경계를 따로 대조했다.

1. **카드의 PR 링크 오기** — 새 카드가 PR #1167을 가리키는지 확인했다.
2. **원본 head와 merge SHA 혼동** — 카드에는 merge SHA `18782b58c5edf2e2cc00aeb71928c523c752a7c3`가 기록되어 있다.
3. **병합 시각 오기** — `2026-11-05T01:31:53Z`를 GitHub merge metadata와 대조했다.
4. **이전 카드 순서와 중복** — #1167 카드를 #1164 뒤에 한 번만 추가했다.
5. **C11.1과 C11.4 분류를 완료 선언으로 혼합** — Root HTML route 제목과 C11.4 bounded scope를 분리해 적고 두 체크를 미완료로 둔다.
6. **지원 범위 과장** — `auto`, min/max-content 및 지원된 `minmax()` explicit tracks로 한정한다.
7. **Chrome oracle 장면과 앱 runtime 장면 혼합** — Chrome 19 scene과 positive definite-axis runtime 17 scene을 구분한다.
8. **zero-axis oracle-only scene의 앱 실행 과장** — runtime scene 수는 positive definite-axis 17개만 기록한다.
9. **브라우저 oracle 버전 누락** — Chrome 156을 명시한다.
10. **Android simulator를 실기기로 확대** — Android API 37 ARM64 AVD라고 명시한다.
11. **iOS simulator를 실기기로 확대** — iPhone 17 Pro/iOS 26.2 Simulator라고 명시한다.
12. **draw/present를 pixel readback으로 확대** — PixelCopy와 Metal drawable readback을 하지 않았다고 기록한다.
13. **focused tests를 전체 suite 성공으로 표현** — 394/404 및 443/444를 함께 적고 전체 green이 아니라고 명시한다.
14. **체크가 없는 PR을 CI green으로 계산** — GitHub check rollup이 비었다고 명시한다.
15. **공식 집계 오기 재발** — 현재 `78/214`를 사용하고 역사적인 `72/213` 문구는 사용하지 않는다.
16. **공식 상위 C11 체크의 조기 완료** — status 원본과 live page에서 C11/C11.4가 미완료인지 대조한다.
17. **새 카드 내부 링크 단절** — README·Android/iOS PNG·20-review route를 selected publication manifest와 대조한다.
18. **서버 상태 및 손상 경계** — Tailscale Running, PID 28040 유지, 341 HTTPS hashes, backup 30/30, no deletes를 확인했다.
19. **교체 중 사용 중 파일 덮기** — preflight가 replacement route의 열린 handle 0개를 확인한 뒤 atomic rename을 수행했다.
20. **브라우저 프로세스 잔류** — Agent Browser의 desktop/mobile 검증 뒤 session을 닫고 process 조회를 확인했다.

## 판정

선택된 341 route를 정확한 hash로 Tailnet에 반영했고 live page에서 #1167과 `78/214`를 확인했다. 전체 docs 링크 검사는 통과했다고 주장하지 않는다. C11은 미완료다.
