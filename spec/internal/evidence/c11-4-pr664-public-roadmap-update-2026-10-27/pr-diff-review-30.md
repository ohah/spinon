# C11.4 PR #664 공개 로드맵 변경 적대 검토 · 30개 관점

- 검토 대상: 브랜치 docs/c11-4-pr664-roadmap-source-20261027, 기준 origin/main / HEAD bc681ad039081c6bce6dd59594a2e8353d1300eb
- 변경 범위: spec/public/roadmap.html:187의 단일 문단 추가
- 대조 자료: GitHub PR #664 metadata/body, spec/STATUS.md, C11.4 계획·사전 비교, pass6 README/result.json/runtime log/simulator.json, 양 검토 보고서와 캡처 manifest
- 방식: read-only 소스·증거 대조와 diff check. 새 테스트·빌드·브라우저 실행은 하지 않았다.

## 판정

**PASS 30 / PARTIAL 0 / FAIL 0.** 추가 문단의 PR 병합 정보, 실행 결과·범위, 두 검토 집계와 공식 상태가 연결된 원본과 일치한다. mutation별 Android/iOS 앱 negative matrix는 여전히 실행되지 않았으며 문단과 실행 README가 이를 명시한다.

새 계획 링크의 source는 repo 최상위 plan/에 있다. Tailnet selected-root 준비 스크립트가 그 source를 site root의 plan/ 및 spec/plan/으로 overlay하는 기존 규칙을 확인했다. 기본 Rspress root=spec가 repo 최상위 plan/을 직접 publish한다고 판단하지 않았고, 이 검토는 새 Tailnet URL을 실시간으로 열어 확인한 것이 아니다.

## 적대적 검토 30개

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 변경이 다른 roadmap row나 주변 문서를 같이 바꾸거나 삭제한다. | **PASS** — 현재 branch의 working-tree diff는 spec/public/roadmap.html의 한 줄 추가뿐이다. |
| 2 | 새 row가 C11.4 업데이트 묶음 밖에 놓이거나 다음 섹션 구조를 깨뜨린다. | **PASS** — 187행에서 기존 #656 문단 다음, 188행 섹션 닫힘 직전에 형제 p로 배치됐다. |
| 3 | HTML 태그가 닫히지 않아 다음 콘텐츠까지 감싼다. | **PASS** — 새 문단의 p, strong, code 5개, a 8개가 각각 열림/닫힘 수가 맞고 git diff --check도 통과한다. HTML5 전체 문서 검증을 했다는 주장은 하지 않는다. |
| 4 | PR 링크가 다른 repository나 PR을 가리킨다. | **PASS** — href는 https://github.com/ohah/spinon/pull/664이며 GitHub metadata의 번호·저장소와 일치한다. |
| 5 | PR이 열려 있거나 merge되지 않았는데 병합 완료라고 쓴다. | **PASS** — PR metadata는 state=closed, merged=true다. |
| 6 | 표시한 merge SHA가 PR의 merge commit이나 현재 기준과 다르다. | **PASS** — 문단의 bc681ad039081c6bce6dd59594a2e8353d1300eb은 PR merge_commit_sha 및 origin/main/HEAD와 같다. |
| 7 | 날짜·시간이 UTC 변환 오류로 병합 시각을 잘못 나타낸다. | **PASS** — PR merged_at은 2026-10-26T19:22:09Z다. commit committer time 2026-10-27T04:22:09+09:00과 같은 시각이다. |
| 8 | C11.4 행이 다른 작업을 요약하거나 PR의 범위를 넓혀 말한다. | **PASS** — PR 제목·본문은 fit-content(40px) 이웃 auto implicit row의 고정 app fixture slice를 다루며 row의 요약과 일치한다. |
| 9 | 계획 href의 파일이 없거나 site 경로를 기본 Rspress build가 만든다고 잘못 전제한다. | **PASS** — plan/c11-4-fit-content-auto-neighbor-runtime.md가 존재한다. 기존 Tailnet selected-root 준비 스크립트의 alias 규칙은 top-level plan/ source를 plan/ 및 spec/plan/ route에 overlay한다. 기본 docs root=spec의 직접 publish로 간주하지 않는다. |
| 10 | 실행 근거 href가 잘못된 경로 또는 다른 runtime slice를 가리킨다. | **PASS** — /spinon/internal/evidence/.../README.html에 대응하는 spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-2026-10-27/README.md가 존재하고 문단의 run·범위와 일치한다. |
| 11 | 구현/runtime review href가 빠졌거나 20-view 자료가 아닌 곳으로 간다. | **PASS** — 연결된 implementation-review-20.md가 존재하고 제목에서 20관점 검토임을 확인했다. |
| 12 | 구현/runtime 검토의 결과 집계가 로드맵에서 바뀐다. | **PASS** — 해당 보고서의 최종 판정은 19 PASS / 1 PARTIAL / 0 FAIL이며 문단의 19/1과 맞는다. |
| 13 | PR diff review href가 잘못되거나 30-view 검토가 아니다. | **PASS** — 연결된 pr-diff-review-30.md가 존재하고 제목상 30개 관점 자료다. |
| 14 | PR diff 검토 집계가 로드맵에서 바뀐다. | **PASS** — 연결 보고서의 판정은 PASS 29 / PARTIAL 1 / FAIL 0으로 문단의 29/1과 일치한다. |
| 15 | Android 화면 링크가 없거나 run manifest와 다른 PNG를 가리킨다. | **PASS** — 링크 대상 PNG가 있고 실제 SHA-256은 source-revision.json의 Android screenshot hash 35b3d269a54e01f7019215f74c4aa6ccbec6f62c6afaf83527c30e1366f5ad7c와 같다. |
| 16 | iOS 화면 링크가 없거나 run manifest와 다른 PNG를 가리킨다. | **PASS** — 링크 대상 PNG가 있고 실제 SHA-256은 manifest의 iOS screenshot hash d4d94e106634755cabd3dd204de3e241ec47a336f96127cf15ef36dee19fb51f와 같다. |
| 17 | 공식 상태 href의 파일 또는 fragment가 존재하지 않는다. | **PASS** — /spinon/STATUS.html에 대응하는 spec/STATUS.md가 있고 CSS 구현 체크리스트 제목이 fragment css-구현-체크리스트의 source다. |
| 18 | 이번 row가 상위 C11.4 checkbox를 완료로 승격한 것처럼 읽힌다. | **PASS** — spec/STATUS.md:273의 C11.4 checkbox는 미체크이고 문단도 C11.4 미완료를 그대로 쓴다. |
| 19 | 완료 집계 72/213이 최신 상태와 다르게 표시된다. | **PASS** — 현재 상태 대장과 인접 C11.4 기록의 공식 완료 집계는 72/213이며 새 row가 이를 유지한다고 쓴다. |
| 20 | 한 장면 실행을 여러 scene 또는 전체 행렬로 과장한다. | **PASS** — Android/iOS result.json 각각 sceneCount=1이고 scene ID도 fit-content-auto-neighbor-wpt 하나다. |
| 21 | opt-in fixture 검증을 일반 제품 입력 admission이라고 잘못 표현한다. | **PASS** — plan과 README는 기존 opt-in C11.4 app fixture라고 한정하며 row도 일반 CSS admission 미검증을 분리한다. |
| 22 | [10,290] 기대값이 규범 근거 없이 Chrome 결과만 베껴 만든 값이다. | **PASS** — expected.json은 CSS Grid 2025 CRD 조항에서 독립 산출한 기대 rows [10,290]을 기록하고 auto track만 남은 공간을 받도록 정의한다. |
| 23 | Chrome 156 upstream 비교를 Spinon WPT 통과로 확대한다. | **PASS** — precomparison README는 upstream WPT의 Chrome 관찰을 명시하고 Spinon WPT 결과가 아님을 분명히 한다. 새 row도 Spinon WPT를 미검증으로 열거한다. |
| 24 | Android API 또는 CPU ABI가 evidence와 다르다. | **PASS** — simulator.json은 Android API 37, arm64-v8a AVD를 기록한다. |
| 25 | Android를 실기기나 hardware GPU로 오인하거나 renderer 조건을 숨긴다. | **PASS** — runtime log는 SwiftShader Device를 기록하고 row는 software renderer, 실기기와 hardware GPU가 미검증이라고 쓴다. |
| 26 | iOS Simulator 모델 또는 OS 버전이 캡션과 다르다. | **PASS** — simulator.json은 iPhone 17 Pro와 iOS 26.2 runtime을 기록한다. |
| 27 | root/item frame이 목표 CSS geometry와 맞지 않는다. | **PASS** — 양쪽 result.json의 ordered frames는 root 60×300, 첫 item/child 60×10, auto item 60×290 at y=10, 두 번째 child 60×10이다. |
| 28 | 결과 rows가 [10,290]이어도 실제 Rows trace와 다르거나 다른 축/scene을 읽는다. | **PASS** — 두 result.json의 대상 scene tracks는 10과 290이고, runtime logs의 대상 Rows trace와 frame은 동일 run ID에 기록돼 있다. |
| 29 | frame/trace만 있고 draw/presentation이 없거나 다른 revision을 잘못 결합한다. | **PASS** — 양 runtime log의 PRESENTATION_MATCH는 matched=true이며 expected/actual identity가 1/25/3/0/1로 같다. 각 result.json도 drawPresented=true를 기록한다. |
| 30 | 5/5 표본이나 지원 범위를 과장하고 남은 negative matrix 한계를 감춘다. | **PASS** — Android PixelCopy와 iOS Metal log 각각 expected/observed/matched 5/5를 기록한다. README는 iOS drawable readback이 compositor 전이고, mutation별 앱 negative matrix·일반 CSS admission·Spinon WPT·실기기/hardware GPU·iOS compositor 이후 pixels가 미검증이라고 명시한다. |

## 결론과 남은 범위

- 이 단일 public roadmap 문단에서 수정이 필요한 사실 오류나 링크 source 오류를 찾지 못했다.
- Tailnet selected-root의 plan/ overlay mapping은 확인했지만, 기본 root=spec docs build가 그 경로를 publish한다고 주장하지 않았다. 실제 Tailnet 링크 확인은 별도 selected-root sync에서 수행해야 한다.
- mutation별 변형 입력을 두 플랫폼 앱 launch/draw까지 연결하는 negative matrix는 아직 없다. 이 제한은 실행 README와 로드맵 문단 양쪽에 명시돼 있다.
- C11.4는 미완료이며 공식 완료 집계 72/213을 유지한다.
