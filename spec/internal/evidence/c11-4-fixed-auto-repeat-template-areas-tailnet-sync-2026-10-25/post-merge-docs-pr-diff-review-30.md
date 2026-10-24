# C11.4 PR #604 병합 후 문서·Tailnet 변경 적대 리뷰 30개

## 범위와 판정

범위는 `plan/c11-4-fixed-auto-repeat-template-areas.md`, `spec/STATUS.md`의 #604 병합 후속 행, `spec/public/roadmap.html`의 #602 역사 기록과 #604 병합 기록, `spec/internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-24/README.md`, 그리고 같은 날짜의 `c11-4-fixed-auto-repeat-template-areas-tailnet-sync-2026-10-25/` 디렉터리 전체다. 무관한 untracked 파일은 조사하거나 바꾸지 않았다.

현재 diff와 모든 scoped script, JSON, 화면 증거를 읽고 GitHub PR 상태를 조회했다. 읽기 전용 해시 대조에서 source/build manifest 52개, 선택 build의 전체 `static/` 파일 19개, 현재 preview 경로 30개, 기존 교체 backup 4개가 모두 일치했다. roadmap PNG도 저장된 SHA-256과 1280×633 크기가 일치한다. 테스트·문서 build·앱 build는 실행하지 않았고, 이번 리뷰에서는 브라우저를 시작하지 않았다. Agent Browser 결과와 세션 종료는 저장된 Agent Browser evidence를 확인했다. 검토 대상 파일은 수정하지 않았으며, 이 리뷰 파일만 새로 작성했다. 기존 변경 파일은 검토 전부터 staged 상태였고 stage/unstage하지 않았다.

**결론:** blocking finding은 없다. 앞서 확인한 누락된 RSPress static assets와 선택 build dead-image 검사 설명은 수정됐다. 남은 증거 한계는 전체 문서 build 실패 원문 로그가 scoped evidence에 없고, 최초 통합 결과 파일이 verifier에 origin 필드를 넣기 전에 생성됐다는 점이다. 두 번째 한계는 origin을 포함해 새로 실행한 `tailnet-final-verification-reproduced.json`으로 보완됐으며, 이전 파일은 역사적 첫 결과로만 취급한다.

## 실패 관점 30개

1. **PR 상태를 open/closed만으로 오판하는 경로** — `gh pr view 604`에서 `state=MERGED`, `mergedAt=2026-10-24T15:51:45Z`를 확인했다. plan, STATUS, roadmap의 #604 상태는 병합 사실과 맞는다.

2. **head SHA를 merge SHA로 잘못 적는 경로** — GitHub의 `mergeCommit.oid`는 `912c10bee4e2d88374a2a32689ac37c824b1207d`이고, plan·STATUS·roadmap·runtime README도 이를 쓴다. `headRefOid`의 `dfce60a4…`와 혼동하지 않았다.

3. **빈 check rollup을 CI 통과로 확대하는 경로** — PR API의 `statusCheckRollup`은 빈 배열이고 main required-status-check API는 404 `Required status checks not enabled`를 반환했다. 네 문서 모두 CI 통과를 주장하지 않는다.

4. **#602 계획 PR을 구현 PR로 오인하는 경로** — `roadmap.html`의 #602 문단은 계획 당시 기록이라고 밝히고 후속 구현은 아래 #604 문단을 따르도록 연결한다. #602 SHA `6a70d3e…`와 #604 SHA가 분리돼 있다.

5. **계획 PR #602의 날짜·SHA가 병합 증거를 덮는 경로** — roadmap의 #602 기록은 계획 전용이며 Chrome oracle과 계획 산출물만 링크한다. 이를 앱 runtime이나 simulator 결과라고 표현하지 않는다.

6. **공식 완료 수가 제한 slice 때문에 증가하는 경로** — plan, STATUS, roadmap, runtime README가 `72/213`을 유지한다. Agent Browser evidence도 roadmap·STATUS·plan에서 같은 값을 확인했다.

7. **C11.4 상위 checkbox를 완료 처리하는 경로** — STATUS의 C11.4 행은 `[ ]`이고 Agent Browser 기록은 `c114_checked=false`다. #604 병합 후속 행도 상위 완료로 승격하지 않는다.

8. **fixture 결과를 일반 CSS 지원으로 확대하는 경로** — plan과 runtime README는 opt-in 제한 범위를 명시하고 일반 CSS admission, WPT, 전체 C11.4를 미검증으로 남긴다. roadmap/STATUS도 같은 한계를 표시한다.

9. **시뮬레이터 수치를 실기기 결과로 확대하는 경로** — STATUS는 Android API 37 AVD와 iOS 26.2 Simulator를 구분하고, Samsung SM-S731N 시도는 Dozing/keyguard 때문에 draw/present와 pixel을 얻지 못했다고 기록한다. 이를 실기기 통과로 세지 않는다.

10. **C11.4의 다른 미지원 영역이 사라지는 경로** — STATUS의 병합 후속 행은 일반 admission, WPT, Android hardware GPU, iOS compositor 이후 pixels, 최종 APK/app bundle fixture exclusion을 계속 미검증으로 둔다. C11.4 상위 미완료 표시와 충돌하지 않는다.

11. **병합 전 candidate와 병합 후 상태를 혼동하는 경로** — STATUS의 앞선 제한-slice 후보 설명 뒤에 #604 병합 행이 별도로 이어진다. candidate는 opt-in 제한 profile 설명으로 읽히며 PR이 pending이라고 주장하는 문장은 없다.

12. **plan의 현재 상태와 병합 링크가 어긋나는 경로** — plan 첫 상태 문단에 PR #604, merge 시각, SHA, 빈 check rollup 및 `72/213` 유지가 함께 기록돼 있다. 계획 본문이 여전히 계획 단계의 acceptance를 설명하는 점도 현재 구현 상태로 오인하지 않는다.

13. **roadmap 화면 링크가 잘못된 source PNG를 가리키는 경로** — #604 roadmap 카드가 Android/iOS의 `fixed-auto-area-columns-area-larger.png` 경로를 지정하고, `selected-build-verification.json`의 11개 직접 route에 두 파일이 들어 있다. source/build SHA가 각각 77,795 및 133,225 bytes와 일치한다.

14. **선택 문서 root가 다른 문서 내용을 섞거나 바꾸는 경로** — `prepare-selected-root.mjs`는 11개 고정 source→route 매핑, symlink 거부, manifest 및 파일의 exclusive 생성으로 제한한다. `selected-root.json`의 source와 selected root 해시 22개가 현재 source와 일치했다.

15. **선택 RSPress build가 산출하지 않은 HTML을 sync하는 경로** — `prepare-selected-site.mjs`는 고정 selected root와 전용 output 경로를 지정하고 `selected-build-verification.json`은 11개 산출 route의 크기/SHA를 기록한다. 현재 selected output에서 11개 해시가 모두 일치했다.

16. **생성된 정적 자산을 문서 route와 혼동하거나 누락하는 경로** — `prepare-static-assets.mjs`는 `static/` 재귀 파일을 manifest에 담고 symlink/특수 파일을 거부한다. 실제 selected build `static/` 파일 집합과 manifest가 19/19 일치해 누락·추가 route가 없다.

17. **pass01처럼 hashed image 또는 lazy JS chunk를 빠뜨리는 경로** — 첫 11-route 동기화에서 이미지 `naturalWidth=0`였던 실패가 기록돼 있고, pass02는 선택 build `static/` 전체 19개를 포함한다. CSS, 두 hashed image, async route chunks, shared JS, license, search index가 모두 manifest 및 final route union에 있다.

18. **dead-image 검사를 끈 사실을 숨기는 경로** — `rspress.config.ts`가 `checkDeadLinks:false`와 `checkDeadImages:false` 모두 설정한다. README 9행은 이제 둘 다 비활성화하며 완전성을 보장하지 않는다고 정확히 설명한다.

19. **선택 build 결과를 전체 RSPress build 성공으로 표시하는 경로** — README는 선택 root/hash 확인만 기록하고 전체 `bun run docs:build`가 성공했다고 주장하지 않는다. 기존 내부 `.json`·`.log` dead-route 실패 원문 log는 scoped evidence에 없어 원인 문구를 본 리뷰에서 재생성·독립 확인할 수는 없다.

20. **문서 base path와 hashed asset URL이 어긋나는 경로** — 재현 verifier는 `remote_origin=https://macstudio.tailed42f2.ts.net`, `remote_path_prefix=/spinon/`을 기록한다. Agent Browser의 Tailnet runtime README 결과에서 Android PNG `naturalWidth=1080`, iOS PNG `naturalWidth=1206`이다.

21. **이미지 파일은 복사됐지만 잘못된 URL/404로 표시되는 경로** — Agent Browser evidence는 수정 전 두 이미지 폭 0과 수정 후 폭 1080/1206을 모두 기록한다. 이는 pass02가 실제 Tailnet 문서에서 두 URL을 해석한 결과다.

22. **문서 11개와 static 19개 사이 route 충돌이나 중복이 생기는 경로** — `verify-tailnet-final.mjs`는 합친 route 집합의 중복을 assertion으로 거부한다. `tailnet-final-verification-reproduced.json`은 서로 다른 route 30개를 기록하고 manifest와의 미매핑 경로가 없다.

23. **manifest 생성 뒤 source/build 파일이 바뀌는 경로** — read-only 재대조에서 source root↔selected root, selected build, static asset source 등 52개 hash가 현재 파일과 모두 일치했다. stale artifact를 최신 source 증거로 잘못 쓰는 불일치는 발견하지 못했다.

24. **local preview가 source/build와 다른 파일을 계속 제공하는 경로** — 현재 `SpinonPreview`의 합친 30개 파일을 직접 hash 대조했고 모두 manifest와 일치했다. `tailnet-final-verification-reproduced.json`도 local 30/30을 기록한다.

25. **원격 HTTP 200이지만 내용은 stale인 경로** — origin을 포함해 새로 재실행한 통합 verifier 결과에서 Tailnet 30/30이 HTTP 200 및 source SHA와 일치한다. 이전 `tailnet-final-verification.json`은 origin 필드 추가 전의 역사적 pass 결과라 fresh 재현 파일과 구분한다.

26. **remote origin을 모른 채 임의 서버를 Tailnet으로 보고하는 경로** — 새 `tailnet-final-verification-reproduced.json`에 정확한 host와 `/spinon/` prefix가 저장돼 있고 README도 같은 주소를 적는다. verifier의 `fetch`는 redirect를 따라가지만 최종 `response.url`이나 redirect chain은 결과에 남기지 않으므로 최종 host provenance는 기록되지 않는다. pass01/pass02 개별 verify JSON보다 개선됐으며 현재 대상은 의도된 Tailnet 주소다.

27. **기존 preview 문서를 덮으면서 복구본을 잃는 경로** — `tailnet-preflight.json`은 replace 4개와 이전 SHA를 기록하고, `tailnet-final-verification-reproduced.json`은 backup 4/4를 검증한다. 네 backup은 현재 실제 파일 크기와 SHA도 다시 일치했다.

28. **pass02가 같은 자산을 불필요하게 덮거나 backup을 덮는 경로** — `tailnet-assets-preflight.json`은 생성 10, 동일 9, 교체 0을 기록하고 `tailnet-assets-copy.json`은 전체 19개 처리 및 삭제 없음으로 남는다. 따라서 pass02에서 backup 0개인 것은 기존 9개가 같은 hash였기 때문이며 누락으로 보이지 않는다.

29. **경로 조작·symlink·삭제가 미리보기의 다른 파일에 전파되는 경로** — sync script는 `static/` route의 `..`, 절대경로, 역슬래시와 symlink 경로를 거부하고, 변경 전 hash 확인·exclusive backup·임시 파일 rename을 사용한다. preview 대상 삭제 호출은 없고, 오류 정리의 unlink는 아직 rename하지 않은 임시 파일에만 적용된다. 다만 batch 전체 rollback은 없으므로 중간 복사 실패 시 일부 route만 갱신될 수 있으며 새 preflight로 복구해야 한다(실제 기록된 pass는 완결됨).

30. **화면 증거 또는 Agent Browser 프로세스를 남겨두는 경로** — 저장 screenshot은 1280×633이며 file SHA가 `d8153e60b10587296f96d466bee5600dcfb39c01ceae4b7fba944504171635b4`와 일치한다. Agent Browser 결과는 세 session close, `closeAllUsed=true`, active session/listener 각 0을 기록한다. 이번 리뷰는 브라우저를 열지 않았고 종료 상태를 증거 파일로 확인했다.

## 남은 범위

- 전체 `bun run docs:build` 실패 원문 log는 이 scoped evidence에 없다. 실패 원인과 발생 환경을 독립 재현하지 않았으며, 이 리뷰에서도 문서 build를 실행하지 않았다.
- `tailnet-final-verification.json`은 origin 필드가 verifier에 추가되기 전에 만든 최초 통합 결과다. 최종 원격 주장은 origin과 route prefix를 기록한 fresh `tailnet-final-verification-reproduced.json`을 따른다.
- HTTP hash와 이미지 `naturalWidth`는 route 접근·이미지 decoding을 증명한다. 선택 문서 전부의 링크/image 완전성, JS 상호작용/검색 hydration, 전체 RSPress build, 앱 runtime을 증명하지 않는다. 선택 RSPress config는 dead-link 및 dead-image 검사를 모두 껐다.
