# C11.4 mixed cross-axis span groups 계획 PR 변경 적대 검토 30개

**범위:** 다음 계획, C11.4 상태 메모, 공개 로드맵 문구와 링크, 고정 입력 및 선택 문서 렌더링 evidence. Runtime 구현 diff는 없다.

| # | 실패 경로·불변 조건 | 검토 결과 |
|---:|---|---|
| 1 | 변경이 계획 문서 범위를 넘어 제품 runtime을 몰래 바꾸는가? | tracked diff는 parent plan, 상태 대장, 공개 로드맵뿐이고 runtime 소스는 수정하지 않았다. 통과. |
| 2 | 계획이 C11.4를 완료로 승격하는가? | plan·STATUS·roadmap 모두 runtime 미시작과 C11.4 미완료를 명시한다. 통과. |
| 3 | 완료 checkbox가 실수로 바뀌었는가? | `spec/STATUS.md` diff는 미체크 C12 앞의 설명 행 하나만 추가한다. 통과. |
| 4 | 공식 집계가 바뀌었는가? | 공개 roadmap의 신규 설명은 기존 `77/213`을 그대로 보존한다. 통과. |
| 5 | 78개 상위 항목 수를 바꾸는가? | 상위 항목 행을 추가하지 않고 C11.4의 날짜별 진행 메모만 추가했다. 통과. |
| 6 | 메모가 C11.4와 관계없는 C12 아래 잘못 들어가는가? | `spec/STATUS.md`의 C11.4 진행 기록 바로 뒤, C12 시작 전에 배치했다. 통과. |
| 7 | parent plan의 다음 단계가 이전 two-item 단계를 반복하는가? | parent plan은 새 span-3 C 및 mixed span length를 다음 독립 slice로 설명한다. 통과. |
| 8 | parent plan과 새 집중 계획이 scene 수나 범위에서 충돌하는가? | 양쪽 모두 네 fixed DOM-order scene과 세 direct text item을 가리킨다. 통과. |
| 9 | C11.4 두 plan 파일 이름이 뒤바뀌는가? | 새 mixed-span 파일과 기존 two-item cross-axis 파일 링크를 각각 build route에서 구분했다. 통과. |
| 10 | 공개 roadmap 링크가 public base `/spinon/`를 빠뜨리는가? | Agent Browser에서 계획·사전비교·계획 review 20·PR diff review 30·공식 상태 다섯 링크가 모두 200이었다. 통과. |
| 11 | Markdown plan에서 새 evidence README 상대 경로가 잘못된 폴더로 가는가? | 빌드된 `spec/internal/evidence/.../README.html` route가 Agent Browser에서 열렸다. 통과. |
| 12 | 기존 runtime README 링크가 축약 build에서 raw `.md`로 남는가? | 선택 build에 실제 README와 그림 두 장을 포함한 뒤 plan 링크가 `README.html`로 정규화됐다. 통과. |
| 13 | 내부 계약 0059 상대 경로가 오타인가? | build와 Agent Browser route가 `spec/internal/0059-c11-grid-layout.html`로 연결됐다. 통과. |
| 14 | 계획 review 20점과 PR diff review 30점 route가 어긋나는가? | plan, evidence README, 공개 roadmap이 동일한 두 review 파일을 가리킨다. 통과. |
| 15 | 상태 anchor `#c11-css-grid`가 사라졌는가? | roadmap의 공식 상태 링크가 선택 build에서 HTTP 200을 반환했다. 통과. |
| 16 | HTML roadmap의 신규 행이 기존 `<p>`나 `<section>` 구조를 깨뜨리는가? | Agent Browser DOM에서 새 `.hero-progress` 문단 하나와 네 내부 링크를 추출했다. 통과. |
| 17 | HTML 텍스트에서 CSS 구문이 마크업으로 오인되는가? | `minmax(calc())`는 `<code>`로 감싸고 text-only 설명은 기존 inline HTML 흐름을 유지한다. 통과. |
| 18 | 신규 문구가 Chrome 관측을 규범 기대값처럼 표현하는가? | 독립 CRD 계산과 Chrome 156 관측을 별도 항목으로 구분한다. 통과. |
| 19 | Chrome oracle screenshot이 Spinon runtime 결과로 오해되는가? | README는 Chrome fixture 한계와 Runtime 미검증을 명시하며 screenshot alt도 Chrome 156을 적는다. 통과. |
| 20 | Chrome fixture image의 상대 경로가 RSPress module resolution을 실패시키는가? | `./chrome156-mixed-span-groups.png`를 사용하고 Agent Browser에서 natural width 1280을 확인했다. 통과. |
| 21 | 기존 Android/iOS image link가 runtime evidence page에서 끊기는가? | Agent Browser에서 AVD 1080px 및 iOS Simulator 1206px 이미지가 모두 로드됐다. 통과. |
| 22 | Ahem font revision/line metrics가 재현 불가능하게 빠졌는가? | 400 weight, `10px/10px`, 저장소 TTF 경로와 SHA-256 manifest를 기록한다. 통과. |
| 23 | Root/item box model 누락으로 예상 track 계산이 흔들리는가? | content-box root, border/gap/padding, direct block child와 zero margin/padding/border를 고정했다. 통과. |
| 24 | span-2 contribution이 합산되거나 source order에 좌우되는가? | 계획은 A/B per-track maximum과 forward/reverse invariant를 별도 gate로 둔다. 통과. |
| 25 | intrinsic track bases와 final percentage-resolved tracks를 혼동하는가? | `[9,19,19]` constraint bases와 final intrinsic columns를 분리한다. 통과. |
| 26 | row contribution과 stretched frame을 혼동하는가? | text contribution과 item frame을 plan gate에서 별도 관측하고 A/C wrapping을 고정한다. 통과. |
| 27 | root border 밖 B 표본이 다른 item에 가려지는가? | B 끝 좌표, C의 area 끝, `(50,4)`와 clear `(60,4)` 위치를 독립 계산에 남겼다. 통과. |
| 28 | 고정 fixture가 임의 CSS·임의 span 지원으로 확대되는가? | plan은 네 scene 밖 tree, item 수, span, text/font, placement, track grammar를 fail-closed로 둔다. 통과. |
| 29 | 선택 문서 빌드가 전체 docs build 성공이나 dead-link 전수 검증으로 확대 보고되는가? | 전체 빌드 실패 원인을 적고 선택 build 범위와 비활성 dead-link 검사를 따로 공개한다. 통과. |
| 30 | PR diff review가 plan 20점 검토나 향후 runtime 검토를 대체하는가? | 이 30점은 문서 PR 변경만 다루며 별도 plan 20점과 향후 implementation/runtime 20점을 유지한다. 통과. |

**결론:** 문서 범위, 계획 계약, 공식 상태, 재현 입력, 실제 RSPress route와 남은 검증 한계를 따로 확인했다. 기능 구현 또는 전체 문서 사이트 검증의 완료 증거로 사용하지 않는다.
