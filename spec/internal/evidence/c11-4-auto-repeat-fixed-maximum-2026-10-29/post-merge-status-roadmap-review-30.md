# C11.4 auto-repeat fixed maximum 병합 후 상태·계약·로드맵 동기화 검토 30개

구현 PR #796 병합 뒤 계획, 공식 상태 대장, 내부 계약 0059, runtime 근거와 공개 roadmap 문구를 서로 대조했다. 이 검토는 구현 20개 및 PR diff 30개 검토와 구별되는 문서 동기화 확인이다.

| # | 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 문서의 PR 상태가 실제 GitHub state와 다른가? | GitHub PR #796 `state=MERGED`를 확인했다. |
| 2 | merge timestamp가 PR 실제 기록과 다른가? | `2026-10-29T07:08:47Z`로 plan, STATUS, contract, evidence, roadmap이 일치한다. |
| 3 | 문서에 기록된 merge SHA가 다른가? | 각 문서가 `48100cc93b32c00a9fb50bde68e0c12fde78f08b`로 일치한다. |
| 4 | merge SHA가 origin/main에 도달하지 않았는가? | PR merge 직후 `origin/main`을 fetch했고 HEAD가 해당 SHA다. |
| 5 | 구현 merge 때문에 C11.4 parent를 완료 처리하는가? | `spec/STATUS.md`의 C11.4 checkbox는 계속 unchecked다. |
| 6 | 공식 완료 집계가 부당하게 증가하는가? | 공식 `72/213`을 plan, status, contract 및 public roadmap에서 유지한다. |
| 7 | PR #796이 CI 통과로 잘못 표현되는가? | PR check rollup이 비었고 public 문서는 CI 성공 주장을 명시적으로 배제한다. |
| 8 | 필수 status check이 비활성인 저장소 정책을 숨기는가? | main protection API의 required-status-check endpoint가 미설정(404)임을 기록했다. |
| 9 | required review 0건 설정을 승인 완료로 오해하는가? | PR review decision을 승인으로 표현하지 않는다. |
| 10 | 내부 계약 숫자 버전이 올라가는가? | 계약 0059의 숫자 버전 `0.1.0`을 유지한다. |
| 11 | 고정 최대 track slice를 일반 CSS admission으로 넓혀 쓰는가? | plan, contract, status, public roadmap이 fixture-test opt-in으로 한정한다. |
| 12 | 무한정 calc 표현식이 지원된다고 쓰는가? | 최대 `calc(length + percentage)` typed 범위만 허용하고 arbitrary AST는 제외한다. |
| 13 | indefinite percentage basis를 지원한다고 쓰는가? | definite positive axis basis 조건과 제한을 각 근거에서 유지한다. |
| 14 | fraction/auto/intrinsic maximum까지 완료됐다고 읽히는가? | 해당 maximum branch를 unsupported/범위 밖이라고 명시한다. |
| 15 | 0px maximum repeat floor를 검증했다고 주장하는가? | 49 oracle 밖 경계로 표시하고 지원 주장에 포함하지 않는다. |
| 16 | Auto-fill/auto-fit 전체 구현으로 오해되는가? | 문서는 고정 49 scene만 설명하고 전체 auto-repeat 조합을 제외한다. |
| 17 | Chrome 관측과 Spinon WPT 통과를 혼동하는가? | Chrome 156 oracle은 기대값 기준이며 Spinon WPT harness 통과가 아니라고 구분한다. |
| 18 | Android AVD가 실기기/hardware GPU 증거로 승격되는가? | API 37 ARM64 software AVD라고 명시한다. |
| 19 | iOS Metal drawable sample이 compositor 이후 pixel로 불리는가? | compositor 제출 전 observation으로 구분한다. |
| 20 | Android 49 scene count를 다른 run 결과와 혼동하는가? | final runPrefix와 49 observation을 고정했다. |
| 21 | iOS 49 scene count를 다른 run 결과와 혼동하는가? | final runPrefix와 49 observation을 고정했다. |
| 22 | Android 104 exact samples가 재계산 불가능한가? | pixel summary 및 49 per-scene runtime logs에서 104/104, mismatch 0을 확인한다. |
| 23 | iOS 104 exact samples가 재계산 불가능한가? | Metal sample logs와 verifier 출력에서 104/104, mismatch 0을 확인한다. |
| 24 | APK/app install hash가 capture binary와 다른가? | 두 simulator metadata에서 build/install SHA equality를 확인했다. |
| 25 | 모든 49 캡처 이름을 검토할 수 없는가? | 각 platform `screenshot-manifest.json`에 49개 파일명·크기·SHA-256이 있다. |
| 26 | public roadmap 이미지가 누락되거나 경로가 오래됐는가? | Android/iOS 대표 normal 및 overflow PNG 네 개의 repo 경로가 존재한다. |
| 27 | internal docs 간 실행 근거 링크가 틀리는가? | STATUS/contract/plan의 README 및 precomparison 상대 경로를 확인했다. |
| 28 | 리뷰 기록 수 또는 링크가 빠지는가? | implementation/runtime 20 및 PR diff 30 문서가 별도 존재하고 링크된다. |
| 29 | 병합 기록 뒤 Tailnet preview가 이미 갱신됐다고 오해하는가? | plan은 Tailnet preview sync를 남은 단계로 표기한다. |
| 30 | 동일한 범위·한계가 status와 public roadmap에서 어긋나는가? | 두 문서의 49/49, 104/104, `72/213`, 미지원 경계를 항목별로 비교했다. |

**결과:** 병합 metadata, 제한 scope 및 공식 상태에 문서 간 모순은 발견되지 않았다. Tailnet preview의 live render 확인은 이 검토 다음 별도 작업으로 남긴다.
