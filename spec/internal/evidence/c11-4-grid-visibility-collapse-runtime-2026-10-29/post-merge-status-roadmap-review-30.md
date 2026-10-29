# C11.4 Grid `visibility:collapse` 병합 후 상태·로드맵 동기화 검토 30개

구현 PR #804 병합 뒤 공식 상태 대장, 계획, 내부 계약, runtime 근거와 공개 roadmap의 문구를 대조한다. 구현/runtime 검토 20개와 PR diff 검토 30개를 재사용하지 않는다.

| # | 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 구현 PR이 실제로 병합되지 않았는데 병합 완료로 기록 | GitHub PR #804의 `state=MERGED`를 확인했다. 통과. |
| 2 | merge timestamp가 GitHub 값과 다름 | PR `mergedAt`은 `2026-10-29T11:08:12Z`; 상태·계획·로드맵에 같은 UTC 값을 적었다. 통과. |
| 3 | merge SHA 오기 | GitHub merge SHA `85b41e75aaa4452355a8cb38d557a702604fa0cf`를 세 문서에서 대조했다. 통과. |
| 4 | merge SHA가 main에 도달하지 않음 | PR 직후 `origin/main`을 fetch했고 tip commit과 PR merge SHA가 일치했다. 통과. |
| 5 | 제한 구현 merge 때문에 C11 parent를 완료 처리 | `spec/STATUS.md`의 C11 checkbox는 unchecked다. 통과. |
| 6 | 이번 slice 때문에 C11.4 parent가 완료 처리 | C11.4 checkbox는 unchecked이며 plan과 roadmap도 전체 게이트 미완료를 표시한다. 통과. |
| 7 | 기존 완료 C11 하위 항목이 잘못 변경 | 공식 checked 하위 항목 C11.2.2와 C11.2.3만 유지한다. 통과. |
| 8 | 전체 공식 완료 수가 부당하게 증가 | 상태 대장, 계획과 roadmap에서 `72/213`을 유지한다. 통과. |
| 9 | status의 PR URL이 잘못 연결 | 링크는 ohah/spinon PR #804를 가리킨다. 통과. |
| 10 | 계획에 다른 PR의 merge SHA가 들어감 | 계획 SHA를 GitHub PR #804의 `mergeCommit.oid`와 대조했다. 통과. |
| 11 | CI가 실행되지 않았는데 성공했다고 표현 | PR check query가 비어 있고 required status check이 없어 성공 주장을 하지 않는다. 통과. |
| 12 | 보호 설정의 required review와 checks가 과장 | branch protection은 required approval 0개, required status check 미설정, linear history 필수다. 승인이나 CI 결과를 주장하지 않는다. 통과. |
| 13 | fixture 결과가 일반 CSS 지원으로 승격 | status, plan, runtime README와 roadmap에서 고정 opt-in fixture로 범위를 제한한다. 통과. |
| 14 | 모든 Grid collapse 동작을 검증했다고 읽힘 | 직접 배치하는 10개 장면과 Columns/Rows 양축만 설명하고 다른 profile 및 Flex/Table를 제외한다. 통과. |
| 15 | Android 장면 수가 틀림 | final API 37 ARM64 software AVD matrix는 10/10이다. 통과. |
| 16 | iOS 장면 수가 틀림 | final iPhone 17 Pro/iOS 26.2 Simulator matrix는 10/10이다. 통과. |
| 17 | surface sample이 장면 수와 혼동 | Android PixelCopy와 iOS Metal drawable 표본은 각 14/14 exact로 따로 표기한다. 통과. |
| 18 | simulator 결과를 실기기나 hardware GPU 증거로 과장 | Android software AVD, iOS Simulator, iOS compositor 전 drawable 경계를 명시한다. 통과. |
| 19 | hidden-equivalence 추론을 직접 규범 문구로 제시 | CSS Display fallback과 Grid에 별도 collapse sizing이 없다는 근거로부터 얻은 해석임을 표시한다. 통과. |
| 20 | computed `collapse` 보존 여부를 누락 | status와 plan이 computed value 보존 및 hidden-equivalent Grid sizing을 기록한다. 통과. |
| 21 | item frame 보존과 paint/hit 제외가 섞임 | 기록은 frame/contribution 보존과 item paint/hit 제외를 구분한다. 통과. |
| 22 | `display:none`을 hidden/collapse와 동일시 | runtime README는 `display:none`의 zero tracks/frame 대조를 별도로 기술한다. 통과. |
| 23 | visible descendant control 누락 | runtime evidence의 blue child case 및 PR image 확인 근거를 연결한다. 통과. |
| 24 | visible overflow positive control 누락 | Columns/Rows visible control의 root 바깥 표면 및 hit 검증이 runtime README에 연결된다. 통과. |
| 25 | 내부 계약 version을 구현 병합 때문에 올림 | 내부 계약 0059는 `0.1.0`이다. 통과. |
| 26 | 계획과 공식 상태가 서로 다른 범위를 말함 | 둘 다 제한 fixture 완료와 전체 C11.4 미완료를 구분한다. 통과. |
| 27 | status에서 실행 근거 링크 누락 | plan, contract 0059, runtime README, 구현 검토 20과 PR diff 검토 30 링크를 점검했다. 통과. |
| 28 | 공개 roadmap에서 merge proof 또는 제한이 빠짐 | 새 roadmap 카드에 PR #804, timestamp, merge SHA, 고정 scene/sample 수, 미완료 범위를 넣었다. 통과. |
| 29 | 공개 roadmap의 이전 `72/213` 표시가 달라짐 | 새 카드와 주변 C11 카드가 `72/213`을 동일하게 유지한다. 통과. |
| 30 | main merge를 Tailnet live preview 배포 완료로 오해 | 문서는 Tailnet sync를 별도 작업으로 둔다. 실제 제공본은 HTTP/hash 확인 뒤에만 배포 완료로 기록한다. 통과. |

**결과:** 구현 PR #804의 merge metadata와 제한된 동작 근거가 plan, status 및 공개 roadmap에서 일치한다. 공식 C11/C11.4 체크 상태와 `72/213`은 변하지 않는다. Tailnet 미리보기 제공본의 hash 및 Agent Browser 확인은 별도 배포 검증이다.
