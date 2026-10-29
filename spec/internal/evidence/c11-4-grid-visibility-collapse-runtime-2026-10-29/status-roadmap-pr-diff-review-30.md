# C11.4 `visibility:collapse` 병합 상태·공개 로드맵 PR diff 검토 30개

검토 대상은 구현 PR #804 병합 뒤 상태 대장, slice 계획, runtime README와 공개 roadmap에 추가된 동기화 diff다. 구현/runtime 20개, 구현 PR diff 30개, 병합 후 상태·로드맵 내용 대조 30개와 별개다.

| # | 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | PR #804의 merge 증거가 아니라 head commit을 merge SHA로 기록 | GitHub `mergeCommit.oid` `85b41e75aaa4452355a8cb38d557a702604fa0cf`와 일치한다. 통과. |
| 2 | merge 시간대가 local time과 뒤섞임 | GitHub `mergedAt` UTC `2026-10-29T11:08:12Z`를 그대로 썼다. 통과. |
| 3 | 문서화 전에 merge 완료를 가정 | PR #804 상태가 `MERGED`이고 `origin/main` tip이 merge SHA다. 통과. |
| 4 | PR #805 diff에 roadmap 이외 기능 변경이 섞임 | 변경 파일은 STATUS, slice plan, runtime README, review 문서, public roadmap뿐이다. 제품 코드는 없다. 통과. |
| 5 | C11 parent checkbox가 체크됨 | `spec/STATUS.md` C11 parent는 unchecked다. 통과. |
| 6 | C11.4 parent checkbox가 체크됨 | C11.4 intrinsic track sizing은 unchecked다. 통과. |
| 7 | 기존 C11.2.2 공식 상태가 변경됨 | C11.2.2는 checked를 유지한다. 통과. |
| 8 | 기존 C11.2.3 공식 상태가 변경됨 | C11.2.3는 checked를 유지한다. 통과. |
| 9 | 완료 집계가 새 fixture 때문에 증가 | 전체 완료 `72/213`을 유지한다. 통과. |
| 10 | fixture 동작을 일반 author CSS 지원으로 승격 | 모든 새 요약은 고정 opt-in fixture 범위를 명시한다. 통과. |
| 11 | Chrome/WPT 전체 적합성으로 잘못 설명 | PR 본문과 roadmap은 Spinon WPT 미완료를 명시한다. 통과. |
| 12 | `visibility:collapse` computed value가 사라짐 | status, plan, roadmap이 computed `collapse` 보존을 명시한다. 통과. |
| 13 | Grid layout contribution 설명이 모호함 | hidden-equivalent track/frame 보존을 item paint/hit 제외와 분리한다. 통과. |
| 14 | `display:none`이 hidden 대조군과 섞임 | roadmap은 `display:none` control을 별도로 열거한다. 통과. |
| 15 | visible descendant 양성 대조가 빠짐 | roadmap은 visible descendant control을 명시하고 runtime README에 증거 링크가 있다. 통과. |
| 16 | root 바깥 visible overflow control이 빠짐 | roadmap은 root overflow 대조를 명시한다. 통과. |
| 17 | Android scene 수가 모든 테스트 개수를 뜻함 | Android API 37 ARM64 software AVD에서 10/10 scene으로 한정한다. 통과. |
| 18 | iOS scene 수가 device 결과로 잘못 읽힘 | iPhone 17 Pro/iOS 26.2 Simulator 10/10으로 한정한다. 통과. |
| 19 | scene count와 surface sample count가 혼동 | PixelCopy/Metal 표본 14/14를 scene 수와 별도로 표기한다. 통과. |
| 20 | Android AVD를 실기기 또는 hardware GPU로 과장 | software AVD라는 한계를 roadmap에서 유지한다. 통과. |
| 21 | iOS drawable을 compositor 이후 색으로 오해 | compositor 제출 전 Metal 표본이라고 표기한다. 통과. |
| 22 | 두 Simulator 결과를 일반 플랫폼 지원으로 일반화 | 실기기, hardware GPU와 전체 C11.4 미검증 경계를 적었다. 통과. |
| 23 | PR #804 check가 성공한 것처럼 표현 | PR checks가 보고되지 않아 CI 성공 주장을 하지 않는다. 통과. |
| 24 | main branch protection을 잘못 요약 | 조회한 설정은 required status check 없음, approval 0, linear history 필수다. 통과. |
| 25 | 내부 계약의 version을 올림 | 계약 0059는 `0.1.0`으로 유지한다. 통과. |
| 26 | 계획의 완료 상태가 전체 C11.4 완료를 암시 | plan 첫 상태와 본문 모두 상위 C11.4/C11 미완료를 유지한다. 통과. |
| 27 | 새 HTML 링크가 선택한 문서 경로와 맞지 않음 | plan, contract, runtime, 세 검토 문서의 대응 Markdown source 경로가 존재한다. 통과. |
| 28 | roadmap HTML에서 `<code>`/`<a>` markup이 닫히지 않음 | 삽입된 카드 태그를 대조했고 전체 `git diff --check`가 통과한다. 통과. |
| 29 | 기존 roadmap 카드나 작업을 교체/삭제 | diff는 새 `<p>` 한 개 삽입이며 기존 C11.3.3 및 C11.4 카드는 보존된다. 통과. |
| 30 | source 문서 merge와 Tailnet live deployment를 같은 상태로 기록 | review와 PR 본문은 preview sync를 별도 단계로 두고, HTTP/hash/Agent Browser 확인 후에만 live 완료를 보고한다. 통과. |

**결과:** PR #804 merge 기록, 좁은 simulator fixture 범위와 공식 미완료 상태가 문서 변경에 일치한다. 이 PR diff는 live Tailnet preview가 갱신됐다고 주장하지 않는다.
