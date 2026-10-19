# C11.4 Tailnet roadmap PR diff 검토 30개

이 검토는 동기화 실행의 20개 검토와 별도로, PR에 들어가는 public roadmap·내부 색인·동기화 근거가 실제 구현 상태와 일치하는지 확인한다.

| # | 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | 새 roadmap 문장이 기존 C11.4 text 결과를 지우거나 대체 | image 9-scene note를 앞에 추가했고 PR #349의 Ahem text 10-scene note는 그대로 남았다. |
| 2 | 새 note가 C11.4 전체를 완료로 표현 | 제목과 본문이 후속 제한 fixture로 표현하고 C11.4/C11은 미완료라고 적는다. |
| 3 | 공식 STATUS checkbox 또는 완료 수를 roadmap summary가 임의 변경 | `STATUS.md` checkbox와 공식 완료 개수에는 변경이 없다. 새 note는 9개 장면 결과만 요약한다. |
| 4 | 이미지 scene 수가 runtime evidence와 다름 | roadmap의 9/9 scene은 `cross-platform-comparison.json`의 9개 scene table과 같다. |
| 5 | PNG의 실제 natural dimensions가 화면 CSS 크기와 혼동 | note는 decoded 24×16 intrinsic input을 말하고 screenshot을 CSS background box로 제한한다. |
| 6 | Chrome oracle 결과를 모든 브라우저 적합성으로 확대 | 고정 Chrome 155 observation을 명시하며 WPT/일반 적합성은 남은 범위로 둔다. |
| 7 | 시뮬레이터 matrix가 실기기나 hardware GPU 결과로 읽힘 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 정확히 적고 실기기/hardware GPU는 미검증으로 둔다. |
| 8 | CSS pixel 표본을 PNG texel paint로 과장 | red item을 CSS `background` box라고 명시하고 decoded texel paint 주장은 부정한다. |
| 9 | fixed-track item이 grid box 안에서만 그려진다고 오해 | fixed 12px track 밖 24px item 배경 표본을 설명하고 `(20,8)` 표본·3/3 근거를 링크된 README에 둔다. |
| 10 | geometry tolerance가 생략되어 exact equality처럼 오해 | roadmap에 0.05 CSS px tolerance를 밝힌다. |
| 11 | `auto`, `minmax`, `fit-content`, span 및 row scene 범위를 빠뜨림 | 새 note가 해당 feature families를 명시하고 상세 matrix를 README로 연결한다. |
| 12 | PixelCopy와 Metal drawable 표본 시점이 compositor 이후로 오해 | note는 플랫폼별 3/3 exact 결과를 요약하며 상세 README·PR에는 iOS compositor 제출 전임을 남긴다. |
| 13 | image/text intrinsic 일반 조합이 구현된 듯한 표현 | README/PR 한계에는 mixed item, nested/anonymous, 여러 item, aspect-ratio 전달을 남긴다. |
| 14 | dynamic image replacement/cache invalidation을 구현으로 추론 | roadmap note와 evidence README가 dynamic replacement를 미완료로 분리한다. |
| 15 | public roadmap의 PR 링크가 다른 C11.4 PR을 가리킴 | 새 note는 병합된 PR #351에 연결되고 기존 text note는 #349를 유지한다. |
| 16 | evidence 페이지가 main의 선택 빌드에서 빠져 broken route가 됨 | selected root에 C11.4 image evidence를 포함하고 Tailnet `/spinon/` 경로로 실제 HTTP 200/hash를 확인했다. |
| 17 | internal index에서 sync 보고서 링크를 찾을 수 없음 | `spec/internal/README.md`에 PR #351 뒤 Tailnet sync 항목을 추가하고 README·20·30 검토 파일을 연결한다. |
| 18 | RSPress upgrade가 문서 sync와 묶임 | config/log가 pinned RSPress `2.0.22`를 사용하며 package/lock version upgrade는 없다. |
| 19 | 전체 docs build/Pages 배포를 실행한 것으로 오해 | README는 선택 build만을 기록하고 Pages workflow 변경이나 수동 Pages deployment는 없다. |
| 20 | 선택 root에서 public asset이 없어 icon/roadmap 미표시 | icon·logo·roadmap asset을 staging public root에 넣고 Agent Browser에서 로드맵 화면을 확인했다. |
| 21 | 내부 PR/증거 페이지의 code asset이 Rspress route compiler로 실행 | `.mjs`, `.ts`, `.sh` evidence asset은 staging `public/` 경로에 둔다. 사용자 문서 본문은 Markdown route로 렌더한다. |
| 22 | HTML/JS에 작업자 로컬 절대 경로가 남아 Tailnet 브라우저로 노출 | 선택 root text와 Rspress worker bundle을 sanitize하고 결과에 host path가 남지 않았는지 다시 찾았다. |
| 23 | sanitization이 roadmap text나 CSS image route를 훼손 | redaction 대상은 host/worktree absolute path이고, Agent Browser에서 note·roadmap SVG·screenshots route를 확인했다. |
| 24 | Tailnet preview note에서 API/status의 원본과 미리보기를 혼동 | 문구가 Tailnet sync를 정적 미리보기로 정의하고 official status를 read-only 링크로 둔다. |
| 25 | build log가 새로운 최종 pass5 build라고 잘못 표시 | final 선택 build 로그를 별도 이름으로 생성하고 config는 `pass5` root/outdir을 사용한다. |
| 26 | 이전 preview에 있던 1GB 자료가 선택 build 결과로 줄어듦 | copy는 선택 출력만 overlay하며 target의 그 외 22,014개 기존 파일을 지우지 않는다. |
| 27 | 17개 기존 파일을 백업 없이 교체 | pass-4는 replacement 17개 모두 backup SHA를 검증했다. pass-5도 고유 backup 경로로 기록한다. |
| 28 | public screenshot alt가 실제 화면과 다른 simulator를 지칭 | Agent Browser 파일 캡처를 각각 roadmap, STATUS C11.4 image 항목, C11.4 계획에서 만들고 화면을 확인했다. |
| 29 | PR body에 작업 브랜치에만 존재하는 screenshot asset 링크를 둠 | PR 설명의 이미지 링크는 병합 후 main commit SHA로 고정하고 Agent Browser에서 실제 render/hash를 확인한다. |
| 30 | 로드맵 갱신을 제품 검증 완료 수에 더함 | 이 PR은 문서 preview sync만 다루며 app 실행·기능 상태·전체 CSS 완료 수를 변경하지 않는다. |

## 판정

새 roadmap note, 내부 색인, 선택 렌더링, Tailnet 화면은 서로 같은 C11.4 미완료 범위를 보여준다. pass-5 PR body에는 Agent Browser가 캡처한 Tailnet 화면을 commit-pinned 이미지로 연결하고, 병합 뒤 main commit permalink로 바꿔 다시 확인한다.
