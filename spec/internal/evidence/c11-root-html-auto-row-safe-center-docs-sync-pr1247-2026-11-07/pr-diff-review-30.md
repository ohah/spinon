# PR #1247 변경분 실패 관점 검토 30개

대상은 PR #1247의 상태 대장, 공개 roadmap, C11.4 계획, safe/unsafe 후속 계획 및 Chrome 사전 비교 자료다. 다음 30개 관점을 서로 독립적으로 대조했다. Blocking finding은 없다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | bounded slice 때문에 C11 parent를 완료 표시 | `C11 CSS Grid` checkbox가 계속 미완료다. |
| 2 | 최근 C11.4 작업 때문에 C11.4 parent를 완료 표시 | `C11.4 intrinsic track sizing` checkbox가 계속 미완료다. |
| 3 | 공식 완료 수 재계산 누락 | `78/214`가 STATUS·roadmap·계획에 일치한다. |
| 4 | 내부 계약 숫자 버전 상승 | 계약 `0.1.0`을 그대로 유지한다. |
| 5 | PR #1246 merge SHA 오기 | `ea86bd1ceea03df21ed1a5ba89b8b2aaad965948`를 기준으로 대조했다. |
| 6 | PR #1246 시각 또는 merge method 오기 | UTC `2026-11-07T01:48:53Z`, Rebase and merge 기록이 일치한다. |
| 7 | calc 지원 폭을 더 넓게 기술 | definite 40px basis의 `<length> + <percentage>` 두 항만 적었다. |
| 8 | 사전 비교·focused gate 수치 변형 | 120 scenes/1,500 coordinates/최대 오차와 12/12·4/4·6/6이 계획 근거와 맞는다. |
| 9 | simulator OS screenshot을 surface readback으로 과장 | PixelCopy/Metal drawable readback이 아니라고 STATUS와 공개 카드에 표시했다. |
| 10 | 비어 있는 GitHub check를 CI green으로 기록 | PR check rollup 부재를 명시하고 CI 성공 주장은 없다. |
| 11 | 이전 percentage row-gap 기록 삭제/대체 | PR #1243 이력을 남기고 PR #1246 기록을 앞에 추가했다. |
| 12 | STATUS의 상대 링크 기준 오류 | `spec/STATUS.md`에서 `../plan` 및 `internal/evidence` 경로가 올바르다. |
| 13 | 공개 roadmap 최신 카드 순서 오류 | PR #1246 카드를 PR #1243 카드 앞에 뒀다. |
| 14 | 공개 HTML에서 `<length>`를 tag로 파싱 | `<code>` 안에서 `&lt;length&gt;`와 `&lt;percentage&gt;`를 escape했다. |
| 15 | public roadmap 경로 접두사 오류 | 계획·근거 링크에 기존 `/spinon/...html` 형식을 썼다. |
| 16 | 후보 계획을 구현 완료로 표시 | STATUS/roadmap에서 후보를 계획 검토 완료, 구현 전으로 구분했다. |
| 17 | W3C 문서를 안정된 최종 규범으로 소개 | 2026-10-08 Working Draft 및 at-risk 상태를 명시했다. |
| 18 | Chrome 결과를 독립 expected로 재사용 | 자유 공간과 offset을 별도 계산 JSON에 기록했다. |
| 19 | positive control에서 center 기하 계산 오류 | tracks `[10,10,10]`, free space 10px, offsets `[5,15,25]`를 대조했다. |
| 20 | calc percentage basis 오류 | 40px × 120% - 2px = 46px을 사용했다. |
| 21 | overflow extent에서 gap 하나 누락 | 세 track과 두 gutter의 합 122px을 확인했다. |
| 22 | safe fallback의 y 위치 오류 | start fallback offsets `[0,56,112]`를 관측·계산과 대조했다. |
| 23 | unsafe center의 overflow 부호 오류 | -82px free space의 half offset -41px, offsets `[-41,15,71]`을 대조했다. |
| 24 | 브라우저·viewport·fixture provenance 누락 | Chrome 157, Agent Browser 0.31.1, viewport/DPR, fixture SHA와 raw rectangles를 JSON에 묶었다. |
| 25 | PR 본문 이미지가 실제로 열리지 않음 | 잘못된 SHA 링크를 고친 뒤 Agent Browser 새로고침에서 complete와 1280×633 natural size를 확인했다. |
| 26 | Chrome oracle screenshot을 Spinon app 화면으로 오인 | PR 본문 alt와 설명을 Chrome oracle screenshot으로 제한했다. |
| 27 | safe/unsafe 확장으로 인접 값을 허용 | 고정 allowlist 밖 modifier·baseline·distributed 조합은 계획에서 계속 거부한다. |
| 28 | fixture가 다른 Root HTML topology를 사용 | 40×40 Grid, fixed empty item, direct-text A/B 세 개를 고정했다. |
| 29 | 후보 플랫폼 검증 상태 혼동 | 후보의 Android/iOS simulator, real-device, WPT 검증이 아직 없음을 기록했다. |
| 30 | merge/배포 상태를 앞서 주장 | PR #1247은 문서 작업이며 Tailnet 동기화와 public Pages 배포를 완료로 주장하지 않는다. |

## 판정

문서의 merge 사실, official counter, 후속 후보 상태와 Chrome evidence가 서로 모순되지 않는다. 이미지 링크 오류는 병합 전 고쳤고 Agent Browser에서 다시 확인했다. PR #1247 변경분에 blocking finding이 없어 Rebase and merge 가능하다.
