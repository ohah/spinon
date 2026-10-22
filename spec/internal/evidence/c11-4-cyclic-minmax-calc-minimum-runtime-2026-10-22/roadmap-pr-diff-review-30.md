# C11.4 cyclic `minmax(calc())` 병합 로드맵 PR diff 검토 30개

검토 대상은 구현 PR #491 병합 메타데이터를 추가하는 계획·상태 대장과 public roadmap HTML이다. fixed runtime evidence와 기존 C11.4 완료 수를 대조하고, 문서 route 및 Tailnet 반영은 별도 sync evidence에서 확인한다.

| # | 별도 실패 관점 | diff 대조 및 결과 |
| ---: | --- | --- |
| 1 | PR head SHA를 merge SHA로 오기하는가 | GitHub PR #491의 `mergeCommit.oid` `5210435a43ce92d4309636dc0deeae1657cf7c06`과 문서 표기가 일치한다. |
| 2 | 병합 시각의 UTC 표기가 틀리는가 | GitHub의 `2026-10-22T01:58:08Z`와 계획·상태·roadmap의 UTC 시각을 대조했다. |
| 3 | Rebase and merge가 아닌 전략으로 기록되는가 | PR #491의 merge 결과와 기록된 Rebase and merge가 일치한다. |
| 4 | 빈 check rollup을 green CI로 오해하는가 | 세 문서는 rollup이 비었음을 적고 성공한 체크를 주장하지 않는다. |
| 5 | 미설정 required check를 통과한 체크로 오해하는가 | GitHub 보호 API의 404 응답과 문서 문구가 main required check 미설정을 구분한다. |
| 6 | 제한된 runtime slice가 C11.4 전체 완료로 읽히는가 | 문서는 fixed scene 네 개만 기록하고 일반 CSS admission과 전체 C11.4는 미완료로 둔다. |
| 7 | 공식 완료 수가 잘못 증가하는가 | 계획, STATUS, roadmap 모두 `72/213`을 유지한다. |
| 8 | C11.4 checkbox가 체크되는가 | STATUS diff는 checkbox 줄을 수정하지 않는다. |
| 9 | column minimum expected value가 틀리는가 | plan/runtime evidence와 `calc(200% + 4px)`의 24px track을 대조했다. |
| 10 | row minimum expected value가 틀리는가 | plan/runtime evidence와 `calc(200% + 2px)`의 22px track을 대조했다. |
| 11 | `max-content` maximum이 사라지는가 | roadmap은 두 minmax 표현 모두 `max-content` maximum을 보존한다. |
| 12 | cyclic과 definite control을 같은 조건으로 설명하는가 | 문서들은 cyclic axes와 definite control 네 장면을 구분한다. |
| 13 | root border-box 크기가 틀리는가 | 10px content basis와 실제 2px border 네 변에 따른 14×14px 값을 evidence와 대조했다. |
| 14 | item overflow가 root 안으로 잘못 축소되는가 | roadmap은 item이 root 바깥까지 그려지는 결과를 기록한다. |
| 15 | Android AVD를 실기기로 표기하는가 | Android는 API 37 ARM64 SwiftShader AVD로 기록한다. |
| 16 | iOS Simulator를 실제 iPhone 검증으로 표기하는가 | iPhone 17 Pro / iOS 26.2 Simulator 문구를 유지한다. |
| 17 | Metal 표본을 compositor 이후 결과로 부르는가 | 계획·상태·roadmap은 Metal 표본이 compositor 제출 전임을 명시한다. |
| 18 | PixelCopy 표본 수가 달라지는가 | Android evidence와 20/20 표기 및 다섯 지점×네 장면 수를 대조했다. |
| 19 | release archive 검사를 앱 전체 exclusion으로 과장하는가 | 문구는 `spinon-ffi` archive와 fixture marker에만 한정한다. |
| 20 | 공개 API가 추가된 것처럼 보이는가 | 변경은 fixture 경계이며 API/버전 계약 변경을 주장하지 않는다. |
| 21 | plan 링크가 다른 C11.4 계획을 가리키는가 | public roadmap의 링크는 `c11-4-cyclic-minmax-calc-minimum.html`이다. |
| 22 | precomparison 링크가 잘못된 증거 디렉터리를 가리키는가 | 경로 stem이 구현 전 비교 evidence 디렉터리와 일치한다. |
| 23 | runtime README 링크가 다른 fixture를 가리키는가 | 경로 stem과 run ID가 PR #491 runtime evidence README와 일치한다. |
| 24 | Android screenshot이 중간 실패 run을 가리키는가 | 링크는 수정 후 최종 `015105529Z-ccced9ed` run의 cyclic column PNG다. |
| 25 | iOS screenshot이 중간 실패 run을 가리키는가 | 링크는 최종 `015105529Z-3111cd63` Simulator run의 cyclic column PNG다. |
| 26 | 이미지 링크가 HTML에서 escaping 없이 손상되는가 | `/spinon/` 기반 absolute route와 HTML attribute quoting을 점검했다. |
| 27 | 문서 card가 시간 순서를 깨뜨리는가 | PR #491 card는 같은 날 앞선 #487 다음, #480 이전에 배치했다. |
| 28 | 기존 #485·#487·#480 roadmap 문구가 변형되는가 | 삽입은 #487과 #480 card 사이에 한 건이며 기존 card 본문은 보존됐다. |
| 29 | 계획과 상태 대장의 merge provenance가 서로 다른가 | 양쪽 모두 같은 merge 시각, SHA, check rollup 및 required check 상태를 사용한다. |
| 30 | Tailnet 갱신을 GitHub Pages 배포로 혼동하는가 | 이 변경은 source 문서와 별도 Tailnet 미리보기만 갱신하며 Pages workflow는 실행하지 않는다. |

**결과:** source diff에서 남은 불일치를 찾지 못했다. Tailnet 선택 build, 제공 파일 hash, Agent Browser route와 이미지 결과는 별도 sync evidence에 기록한다.
