# PR #721 병합 로드맵 변경 검토 30개

검토 대상은 `spec/STATUS.md`, C11.4 §11.7 후속 계획, `spec/public/roadmap.html`의 새 병합 항목과 링크다. 각 항목은 별도의 사실 오류·경계 혼동·링크 실패를 확인한다.

| # | 실패 가설 | 판정과 근거 |
| ---: | --- | --- |
| 1 | 새 항목을 C11.4의 전체 구현 완료처럼 읽을 수 있다. | **PASS** — 문구는 §11.7의 definite `ExpandFlexibleTracks` 앱 runtime slice로 한정하고 C11.4 미완료를 명시한다. |
| 2 | roadmap이 잘못된 PR에 연결될 수 있다. | **PASS** — #721은 flexible-track app fixture PR이며 GitHub에서 `MERGED` 상태를 확인했다. |
| 3 | 병합 시각을 현지 시각 또는 PR 생성 시각과 혼동할 수 있다. | **PASS** — `2026-10-27T23:01:07Z`는 PR `mergedAt` 값과 일치한다. |
| 4 | SHA가 branch head SHA나 원래 커밋 SHA로 기록될 수 있다. | **PASS** — Rebase and merge 결과의 merge commit `022ea292b0490920d84d0a4fec84f90da7549c7f`를 링크했다. |
| 5 | 병합 방식을 squash 또는 merge commit이라고 잘못 표시할 수 있다. | **PASS** — PR merge 결과는 Rebase and merge이며 roadmap 문구와 일치한다. |
| 6 | 기존 C11.4 checkbox가 이 slice 때문에 완료 처리될 수 있다. | **PASS** — `spec/STATUS.md`와 roadmap 모두 checkbox 미완료 상태를 유지한다. |
| 7 | 공식 완료 집계가 이번 실행 때문에 증가할 수 있다. | **PASS** — 공식 `72/213`을 새 항목과 상태 문서에서 그대로 유지한다. |
| 8 | four-scene 결과를 전체 §11.7 장면 집합으로 오해할 수 있다. | **PASS** — 네 fixed definite-space 입력과 남은 broader CSS Grid 범위를 문장으로 제한한다. |
| 9 | 200px definite axis와 120px 반대축이 뒤바뀔 수 있다. | **PASS** — README expected JSON, manifests, status 기록은 축별 크기가 일치한다. |
| 10 | fixture 결과가 일반 제품 CSS 지원 범위로 게시될 수 있다. | **PASS** — opt-in fixture라고 명시하고 일반 CSS admission을 검증하지 않았다고 밝힌다. |
| 11 | Android 결과가 실기기 결과로 표시될 수 있다. | **PASS** — Android API 37 ARM64 16 KB AVD라고 표시하며 실기기 확인을 제외한다. |
| 12 | iOS 기기명이나 runtime이 실행 manifest와 다를 수 있다. | **PASS** — iPhone 17 Pro / iOS 26.2 Simulator 표기가 최종 iOS manifest와 일치한다. |
| 13 | 한 플랫폼만 성공했는데 두 플랫폼 모두 통과했다고 쓸 수 있다. | **PASS** — Android와 iOS 각각 4/4를 따로 적고 독립 verifier는 두 run을 별도 열로 기록한다. |
| 14 | 결과가 final frame만 비교하고 stage 순서를 확인하지 않은 것으로 과장될 수 있다. | **PASS** — 실행 근거가 target axis의 `Initialize`, `MaximizeTracks`, `ExpandFlexibleTracks`, `StretchAutoTracks` 순서를 기록한다. |
| 15 | root는 확인했지만 marker frame을 확인하지 않은 것처럼 보일 수 있다. | **PASS** — status와 roadmap은 root·item frame을 명시하고 README는 전체 node frame과 verifier를 연결한다. |
| 16 | 10px gap을 track base로 세어 track topology를 잘못 설명할 수 있다. | **PASS** — 문구는 stage track/gutter topology를 구별하고 README는 CSS track 둘과 내부 gutter를 분리한다. |
| 17 | screenshot 링크가 화면 pixel equality를 보장하는 것처럼 읽힐 수 있다. | **PASS** — README는 screenshot을 배치 증거로 제한하고 pixel equality와 GPU readback을 부인한다. |
| 18 | PR CI가 비어 있는데 통과했다고 기록할 수 있다. | **PASS** — #721의 `gh pr checks` 결과는 no checks reported이며 roadmap과 status는 CI 통과를 주장하지 않는다. |
| 19 | workspace Clippy의 기존 실패를 숨겨 모두 통과했다고 쓸 수 있다. | **PASS** — 변경 밖 validation fixture의 `type_complexity` 한계를 함께 기록한다. |
| 20 | Android/iOS 8개 실행을 한 플랫폼의 8/8로 오해할 수 있다. | **PASS** — verifier 결과와 roadmap은 플랫폼별 4/4를 분리한다. |
| 21 | verifier의 negative control 수가 실제 결과와 다를 수 있다. | **PASS** — `verification.json`의 7개 항목과 문서의 개수가 일치한다. |
| 22 | 새 Android stage-order guard가 다시 빠져도 문서가 그대로 통과라고 남을 수 있다. | **PASS** — Android 최종 AVD run은 해당 guard가 포함된 source hash로 캡처됐고 역순 stage negative control이 verifier에서 거부된다. |
| 23 | 전체 WPT 또는 normative conformance를 완료했다고 오해할 수 있다. | **PASS** — roadmap은 전체 WPT를 미검증으로 열거한다. |
| 24 | 일반 CSS admission과 고정 fixture 입력을 혼동할 수 있다. | **PASS** — 현재 결과를 opt-in 앱 fixture로 설명하고 author CSS admission은 제외한다. |
| 25 | simulator 성공을 physical-device 결과로 확대할 수 있다. | **PASS** — 실기기를 명시적으로 남은 범위에 두었다. |
| 26 | 새 public API 또는 contract version이 생겼다고 잘못 기록할 수 있다. | **PASS** — PR 본문·계획은 public API를 추가하지 않았고 기존 내부 계약 0059를 사용한다고 밝힌다. |
| 27 | plan 링크가 pre-merge의 불완전 계획 또는 잘못된 route를 가리킬 수 있다. | **PASS** — `/spinon/plan/c11-4-flex-stage-runtime.html`은 현재 plan source 경로와 맞는다. |
| 28 | evidence 링크가 다른 run 또는 오래된 Android screenshot으로 연결될 수 있다. | **PASS** — Android는 matcher 수정 뒤 재실행한 `225654572Z-2745705b`, iOS는 최종 `224932984Z-8e6b2435`를 사용한다. |
| 29 | 화면 링크가 깨지거나 alt text가 빠질 수 있다. | **PASS** — 두 run에 PNG가 있으며 PR #721의 Agent Browser 확인에서 Android 1080×2400과 iOS 1206×2622 이미지가 모두 로드됐다. |
| 30 | 긴 진행 항목이 `<section>` 경계를 깨거나 overview 본문으로 들어갈 수 있다. | **PASS** — 새 `<p>`를 C11.4 progress 항목 끝과 `overview` section 시작 사이에 삽입했고 기존 section 닫힘은 보존했다. |

## 결론과 경계

로드맵은 병합된 app-runtime slice와 실제 검증 범위를 반영한다. C11.4 전체, 실기기, 일반 CSS, 전체 WPT와 공식 완료 집계의 상태는 바꾸지 않는다.
