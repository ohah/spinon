# C11.4 양축 cyclic `minmax(calc())` spanning text PR diff 검토 20개

PR #506의 구현 연결부, 고정 fixture, platform verifier, 실행 근거와 상태 문서를 별개의 실패 경계 20개로 검토했다. 계획 검토와 구현/runtime 검토의 항목을 재사용하지 않고, branch diff의 wiring·입력 거부·기대값 비교·외부 판정 가능성을 확인했다. 검토 기준은 `origin/main`의 `79c28ebe3dea92a25cdbb9839242f8ed81959fa1`부터 feature branch의 구현·문서 커밋과 이 검토 파일까지다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 신규 scene이 fixture-test 밖의 제품 FFI surface에 노출됨 | 기존 C11.4 fixture registry 경로에만 모듈을 추가했다. 기본/opt-in static archive 비교는 새 scene·error·font marker의 부재/positive control 포함을 나눠 기록하며 앱 번들 전체 검사로 확대하지 않는다. |
| 2 | FFI allowlist 등록과 scene source dispatch가 불일치함 | 네 ID가 `c11_4_grid_track_sizing_runtime_source`에서 새 source 함수로 연결되고 exact ID 외 입력은 `None`으로 남는다. |
| 3 | runtime adapter가 다른 root owner/profile이나 ID만 같은 문서를 수락함 | root tag, ID, root ownership, 빈 `data-node`, 고정 computed profile과 direct-child 개수를 함께 검사한다. 다른 owner·profile은 구체적인 unsupported fixture 오류로 fail closed한다. |
| 4 | grid item 구조가 direct single text라는 전제와 달라짐 | item tag·ID·parent·marker, 단일 direct text child와 정확한 `aaaaaa` 내용을 확인한다. nested child나 text 변형은 해당 adapter의 정상 입력으로 처리하지 않는다. |
| 5 | root track definition·placement 중 하나만 맞아 잘못된 fixture가 통과함 | typed Columns/Rows 각각의 2개 track, cyclic `minmax(calc(10% + 2px), max-content)`, line-name/auto-repeat/areas 부재와 item의 1/3 양축 placement를 검증한다. |
| 6 | 스타일 값이 주변 화면에서 상속되어 intrinsic 기대값을 우연히 만족함 | adapter는 root/item의 layout dimensions, spacing, border, font, wrapping 관련 값과 Ahem size를 고정해 확인한다. 지원 집합 밖 값은 명시적으로 거부한다. |
| 7 | 신규 renderer fixture가 시스템 font에 의존해 호스트마다 다른 줄 수를 냄 | runtime 입력은 system-font fallback이 없는 `TextLayoutEngine`에 저장소 Ahem을 등록한다. CSS source와 Rust paragraph input 모두 Ahem 10px·10px line-height·break-all을 지정한다. |
| 8 | min-content/max-content 장면과 definite control이 ID·width에서 뒤바뀜 | FFI source와 layout oracle은 cyclic `min-content`/`max-content` 및 definite 10/60px를 서로 다른 네 ID로 정의하고 expected root/item frame과 두 축 track을 각 case에 연결한다. |
| 9 | root `min-content`가 page의 일반 available width처럼 처리되어 Grid track sizing에 전달되지 않음 | Taffy root available-space 경로는 `display:grid`와 width `min-content`를 함께 확인하며, grid feature가 꺼진 구성은 기존 계산값을 반환한다. 다른 display의 intrinsic sizing을 전역 변경하지 않는다. |
| 10 | cyclic percentage 재실행이 span contribution으로 이미 키운 minimum base를 다시 낮춤 | Taffy 재초기화는 해당 rerun에서만, percentage를 쓰는 minimum과 intrinsic maximum인 track의 이전 base/해결 minimum 중 큰 값을 유지한다. 첫 pass와 다른 track class에는 preservation을 켜지 않는다. |
| 11 | 첫 sizing pass 또는 무관한 axis pass에 preservation 인자가 잘못 전달됨 | 최초 Columns/Rows 호출은 `false`; percentage 재실행 상태를 구분해 후속 cross-axis pass 인자로 넘긴다. 모든 `track_sizing_algorithm` call site에 새 인자가 들어갔다. |
| 12 | Grid 판별이 borrowed style·compact runtime style에서 기본값 `false`로 남음 | `CoreStyle` 기본 구현, 참조형 forwarding, vendored `Style`의 grid-feature 구현과 Spinon `CompactTaffyStyle` 구현이 각각 연결되어 있다. |
| 13 | Style→layout projection이 새 root intrinsic input을 넓게 허용해 임의 node까지 수용함 | 예외는 opt-in C11.4 경로, root handle, Grid display, `min-content` width와 `max-content` height가 동시에 맞을 때만 켜진다. 다른 intrinsic/image 검증 조건을 바꾸지 않았다. |
| 14 | Rust oracle이 frame은 맞지만 intrinsic source나 axis contribution은 빠뜨림 | focused test는 네 scene 각각에서 Text intrinsic source의 10/60px inline sizes, root/item frame, Columns/Rows track과 두 axis의 span-2 MinContent/MaxContent contribution을 검사한다. |
| 15 | Grid trace에서 Columns/Rows 값은 존재하지만 row contribution이 이전 column pass보다 먼저 계산됨 | Rust test와 simulator verifier가 마지막 Columns pass가 마지막 Rows pass보다 먼저임을 확인하고 `columnSizingFeedsRows=true` 장면에서 순서를 fail 조건으로 둔다. |
| 16 | Java/Swift에 scene 목록은 들어갔지만 frame·node 수·sample expectations가 다른 장면 규칙을 적용함 | Android와 iOS 모두 네 ID, root 14×64/64×14, item 10×60/60×10, 두-node box count 및 장면별 sample mapping을 연결했다. 최종 run은 각 플랫폼 네 장면을 모두 기록한다. |
| 17 | native pixel matrix가 border·item 내부·root 바깥·아래 경계를 혼동하거나 sampling을 누락함 | 각 장면은 6개 좌표에서 left/right/bottom border, item fill, root 옆·아래 clear를 분리한다. AVD PixelCopy 24/24와 pre-compositor Metal drawable 24/24 exact가 근거에 기록됐다. |
| 18 | JS verifier가 예상 track/frame만 비교하고 핵심 span contribution이나 scene uniqueness를 확인하지 않음 | 네 장면을 전체 scene registry에 추가하고 count를 251→255로 갱신하며 ID uniqueness를 유지한다. verifier는 axis별 span-2 MinContent/MaxContent 기여, final frame/track과 Columns→Rows pass 순서를 검사한다. |
| 19 | 결과 이미지와 빌드 증거가 다른 checkout 또는 존재하지 않는 PR URL을 가리킴 | simulator manifests는 source commit·tracked dirty path·app hash를 기록한다. Agent Browser에서 PR #506의 Android/iOS 이미지가 raw branch URL에서 실제 로드되고 `naturalWidth`/`naturalHeight`가 0이 아님을 확인했다. |
| 20 | 문서·PR 설명이 제한 fixture를 일반 CSS/Grid 완료로 부풀리거나 공식 진행 수를 바꿈 | plan·`spec/STATUS.md`·runtime README·PR body가 고정 slice와 glyph/WPT/physical/hardware/compositor 한계를 밝힌다. C11.4 checkbox는 unchecked, `77/213`은 유지하며 구현/runtime 검토와 PR diff 검토를 별도 링크한다. |

## 검토 결론

검토한 최종 diff에서 별도 수정을 요구할 blocker는 찾지 못했다. PR 화면에 올린 두 Simulator 이미지는 정상 표시된다. 결과는 고정 fixture, Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 근거에 한정된다. 일반 CSS admission, WPT, 실기기·hardware GPU, text glyph raster, iOS compositor 이후 pixels와 전체 C11.4는 이 PR로 증명하지 않는다.
