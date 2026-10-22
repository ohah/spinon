# C11.4 auto-repeat line name 계획 PR diff 검토 30개

**검토 대상:** 계획 PR의 diff 전체 — C11.4 parent plan 링크, fixed Chrome oracle, 계획 입력, 독립 계산, scope, simulator gate와 미지원 경계. 각 관점은 서로 다른 문서·규범·구현 실패 가능성을 확인한다.

| # | 실패 관점 | 대조와 판정 |
| --- | --- | --- |
| 1 | 계획 PR에 runtime 구현 코드가 섞이는가 | diff는 계획/사전 관측 문서와 이미지뿐이며 Rust·앱·fixture ABI 변경은 없다. 통과. |
| 2 | 미구현 계획이 공식 완료 checkbox를 바꾸는가 | `spec/STATUS.md`는 수정하지 않았고 상위 C11.4 체크는 미완료다. 통과. |
| 3 | 공식 완료 수를 이전 시점 숫자로 되돌리는가 | parent/신규 계획 모두 `77/213`을 유지한다. 통과. |
| 4 | C11.4 parent plan에서 새 계획으로 가는 링크가 빠지거나 잘못되는가 | parent에 새 계획, Chrome evidence, 20점 계획 검토 링크를 함께 추가했다. 통과. |
| 5 | 계획 전용 20점 검토를 구현 검토와 혼동하는가 | 별도 파일이 20개 관점을 기록하고 구현/runtime 검토는 구현 후 새로 작성한다고 명시한다. 통과. |
| 6 | GitHub PR 유형/영역 라벨이 저장소 기준에 없는 값인가 | `planning` 및 `area: layout`은 기존 PR #444에서 사용한 기존 라벨이다. 통과. |
| 7 | 저장소 언어 규칙에 맞지 않는 사용자 문서가 있는가 | 새 계획, review, README의 서술은 한국어이고 CSS/타입/명령 이름만 원어로 유지한다. 통과. |
| 8 | 계획 문서가 공개 API 추가/내부 계약 완료를 주장하는가 | public API 변경은 계획하지 않으며 계약 0059 완료·버전 변경도 주장하지 않는다. 통과. |
| 9 | auto-repeat 규칙을 1차 규범 대신 블로그/구현 관측에 의존하는가 | CSS Grid Level 1 2025 CRD의 auto-repeat과 line placement 조항을 직접 연결했다. 통과. |
| 10 | line-placement 링크가 문서의 다른 절이나 잘못된 anchor로 이동하는가 | Agent Browser에서 `#line-placement`가 §8.3 제목 H3로 이동하는 것을 확인했다. 통과. |
| 11 | Taffy capability claim이 lockfile과 다르게 적혀 있는가 | workspace Cargo manifest/lock의 vendored Taffy가 정확히 `0.15.0`이다. 통과. |
| 12 | named line/repeat support가 근거 없이 가능하다고 가정하는가 | vendor source의 `GridTemplateRepetition.line_names`, `NamedLine`, `NamedSpan`, `NamedLineResolver`를 확인했다. 통과. |
| 13 | Taffy가 repeat count 전 placement를 해석한다고 잘못 가정하는가 | grid compute 경로에서 axis auto-repeat count를 산정한 뒤 그 count로 `NamedLineResolver`를 만든다. 통과. |
| 14 | 기존 count 계산을 Spinon projection에서 중복 구현하도록 유도하는가 | 계획은 Taffy count 재구현 없이 symbolic placement/line groups 전달을 요구한다. 통과. |
| 15 | top-level component 경계 이름과 반복 내부 line name을 한 배열로 혼합하는가 | 계획은 외부 component group과 repeat fragment의 N+1 group을 별도 보존하도록 적었다. 통과. |
| 16 | 기존 no-repeat line-name 배열 의미가 회귀하는가 | 비-auto-repeat track list의 기존 track N+1 group을 유지한다고 제한한다. 통과. |
| 17 | 생략 occurrence가 첫 이름 line을 고르는 동작이 빠지는가 | `bare` fixture와 `(0,0) 40×40px`을 고정했다. 통과. |
| 18 | positive occurrence가 반복 track count와 어긋나는가 | `2 cell / 3 cell`은 세 repeated track 중 가운데 track `(50,0)`으로 정했다. 통과. |
| 19 | negative occurrence가 authored pattern 끝에서 계산되는가 | `-2/-1`은 expanded explicit grid의 마지막 track `(100,0)`으로 정했다. 통과. |
| 20 | named span frame에서 inter-track gutter가 누락되는가 | 두 40px track 사이 10px gutter를 포함해 90px로 계산한다. 통과. |
| 21 | auto-fit collapse가 line occurrence 자체를 다시 매기는가 | 3개 count를 먼저 정하고 empty first/last track collapse 후 line identity 유지와 item x=0을 분리했다. 통과. |
| 22 | two-track pattern count가 prefix/suffix/gap을 빠뜨리는가 | 260px case의 count 2/3 총 사용량 `220/310px`와 second-pattern item 폭 `80px`을 적었다. 통과. |
| 23 | repeat 경계와 외부 group 병합을 관측하지 않는가 | `merge`는 repeat 시작·끝 양쪽에 동일 이름을 둔 실제 fixture이며 `same 1/same 4` placement를 비교한다. 통과. |
| 24 | row-axis 이름 처리 없이 columns만 지원 범위처럼 표시되는가 | 별도의 180px row auto-fill scene과 root-relative y=50px item을 둔다. 통과. |
| 25 | 화면 절대 좌표를 root-relative expected로 오인하는가 | viewport `1280×633`, DPR 1을 고정하고 JSON에 screen rect와 root-relative frame을 함께 기록했다. 통과. |
| 26 | 수정된 fixture와 observation/screenshot이 다른 입력을 가리키는가 | HTML/JSON/screenshot 각각 SHA-256을 기록하고 JSON을 parse했다. 통과. |
| 27 | reference screenshot을 Spinon이나 simulator 결과로 소개하는가 | README와 파일명은 Chrome 156 oracle라고 구분하고 Spinon/runtime 증거가 아니라고 명시한다. 통과. |
| 28 | 지원 계획이 areas, missing-name implicit line 또는 임의 grammar까지 번지는가 | 해당 조합을 오류 경계로 유지하고 acceptance를 8개 fixed input으로 한정했다. 통과. |
| 29 | named input data model이 기존 Copy/API·allocation·ownership을 무시하는가 | 구현 단계에서 `GridPlacementInput` 표현과 fallible bridge allocation을 결정하도록 위험과 gate를 기록했다. 통과. |
| 30 | simulator/화면 근거가 실제 수행 전 성공으로 적히거나 cleanup을 생략하는가 | 문서는 simulator 8/8과 pixel checks를 미래 gate로만 둔다. Chrome 세션은 닫았고 active session/process가 없다. 통과. |

**결론:** 계획 diff에서 수정할 결함을 찾지 못했다. 이 기록은 계획 PR 검토이며 구현/runtime 실패 관점 검토 20개를 대체하지 않는다.
