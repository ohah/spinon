# C11.4 nested Grid 행 축 계획 PR diff 적대 검토 20개

**검토 대상:** 사전 비교 fixture/evidence, nested Grid 행 축 계획, 기존 #443 병합 상태 반영, 공식 상태와 공개 로드맵 변경. 계획 실패 관점 20개와 별개로 최종 PR diff를 교차 파일 기준으로 검토했다.

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | PR scope에 구현 완료나 제품 CSS 지원 주장이 섞이는가? | 새 계획과 README가 runtime 전 상태, 단일 고정 fixture, 미완료 C11.4로 제한한다. 통과. |
| 2 | 고정 HTML hash가 계획/evidence의 입력과 다른가? | fixture SHA-256 `09b80963…f02b0d9d`를 원본 파일과 README에서 대조했다. 통과. |
| 3 | raw eval, JSON, screenshot이 같은 browser run을 가리키는가? | raw eval과 JSON을 동일 출력으로 저장했고 observation에 Chrome metadata·viewport·scene geometry가 있다. 통과. |
| 4 | font/image 입력이 다른 revision 또는 크기인가? | 저장소 Ahem/PNG hash와 observation의 font-ready, image decode, natural 24×16 크기를 대조했다. 통과. |
| 5 | screenshot이 없는 화면 증거처럼 링크되거나 잘못된 크기로 연결되는가? | 저장소 PNG를 확인했고 README는 Chrome 156 crop이라고 표기한다. PR 본문에는 commit-pinned raw PNG를 넣고 merge 전에 Agent Browser로 이미지 로드를 확인한다. 통과. |
| 6 | normative source가 Grid 단계/버전에 맞지 않는가? | CSS Grid Layout Module Level 1의 fixed 2025 CRD URL과 §§6.6, 11.1, 11.5.1을 연결했다. 통과. |
| 7 | columns-before-rows 주장과 fixture 선언이 서로 다른가? | definite 98px inline input과 inner 70/24px columns가 text wrapping을 고정한 뒤 row contribution을 관찰한다. 통과. |
| 8 | Ahem line wrapping/height가 독립 계산과 맞지 않는가? | 70px longest word, 100px unwrapped advance, 10px line-height 두 줄의 20px block을 cross-check했다. 통과. |
| 9 | inner row gap 또는 natural image height가 중복 반영되는가? | `[20,16]` + 4px inner gap = 40px이며 PNG natural height 16px를 확인했다. 통과. |
| 10 | outer span distribution에서 gap 4px을 잘못 처리하는가? | 40px item contribution에서 gap 하나를 빼 36px을 18/18로 배분하고 outer height 40px을 재계산했다. 통과. |
| 11 | one-track control이 span 분배를 재사용하는가? | `[40,0]`와 root height 44px이 별도 기대값으로 표와 README에 있다. 통과. |
| 12 | definite-flex control이 indefinite axis와 혼동되는가? | root 60px definite block-size, 4px gap, 56px flex space, 28/28 tracks를 명시했다. 통과. |
| 13 | automatic minimum control에 §6.6 조건을 잘못 적용하는가? | item이 두 flexible rows를 span하며 automatic minimum 0px이라는 전제와 한계를 적었다. 통과. |
| 14 | nested grid frame stretch와 내부 row used-size를 혼합하는가? | outer nested frame 60px과 inner rows `[20,16]`, child y=0/24를 별도로 보존한다. 통과. |
| 15 | fixture script가 font/image readiness 전에 값을 반환할 수 있는가? | `document.fonts.ready`, font check, `image.decode()`를 기다리고 readiness 값을 기록한다. 통과. |
| 16 | plan의 artifact boundary가 모호하거나 기본 산출물에 fixture가 섞이는가? | `c04-runtime-gpu-fixture-test`의 default negative와 opt-in positive control을 분리했다. 통과. |
| 17 | 새 plan/evidence 링크나 공개 roadmap asset 경로가 끊기는가? | 변경 링크를 파일 시스템과 `/spinon/` route source에 맞춰 확인했다. 통과. |
| 18 | #443 상태 또는 GitHub check 결과가 과장되는가? | 실제 merge SHA와 mergedAt을 기록하고 empty rollup/disabled required checks를 명시했다. 통과. |
| 19 | 계획 PR이 공식 상태 완료 수나 내부 버전을 올리는가? | C11.4 unchecked, 72/213 유지, 0059 numeric version 0.1.0 유지가 plan/STATUS/roadmap에 일치한다. 통과. |
| 20 | 계획 실패 검토 20개를 PR diff 검토 횟수로 대신하거나 브라우저를 남기는가? | 계획 검토와 diff 검토를 별도 파일로 기록하고 PR screenshot 확인 후 Agent Browser 종료를 요구한다. 통과. |

이 검토는 계획 PR diff 전용이다. PR 본문 screenshot URL의 실제 원격 로드는 브랜치 push 후 Agent Browser에서 확인하고 결과를 PR 본문에 기록한다.
