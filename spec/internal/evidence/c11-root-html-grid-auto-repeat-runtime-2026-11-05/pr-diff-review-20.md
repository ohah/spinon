# C11.1 Root HTML Grid auto-repeat PR diff 적대 검토 20개

검토 대상은 PR 생성 전 최종 stage한 candidate의 구현·fixture·문서·실행 evidence diff다. 앞서 기록한 계획 및 구현/runtime 검토와 별개로 profile 경계, 계산, 실패 원자성, artifact provenance와 상태 기록을 다시 확인했다. 각 행은 별도의 결함 경로다. PR 생성 후 GitHub diff가 이 검토본과 같은지 확인한다.

| # | 공격 관점 | 최종 diff 판정 |
|---:|---|---|
| 1 | 기본 Root HTML 외 profile이 실수로 auto-repeat을 받는가? | Cascade 옵션은 `RuntimeRootHtmlTextPaintV1`에만 켜고 projection도 해당 profile에서만 전용 mode를 선택한다. 통과. |
| 2 | C11.4 `TrackSizing`의 기존 admission이 넓어지거나 좁아지는가? | 기존 `allow_auto_repeat` 분기는 유지하고 Root HTML admission은 별도 옵션·mode로 연결했다. scoped tests와 Clippy 통과. |
| 3 | unsupported shorthand/longhand가 CSS cascade fallback 대신 typed admission을 우회하는가? | computed typed grid 값에서 complete track-list 및 repeat metadata를 재검사한다. 기존 cascade invalid/fallback 처리는 변경하지 않는다. |
| 4 | 외부 track, integer repeat, 두 repeat group이 fragment 결합 순서를 바꾸는가? | cascade에서 단일 전체-list repeat만 허용하며 named/fixed neighbors와 integer repeat은 거부한다. negative matrix 통과. |
| 5 | 잘못된 start/end 또는 metadata 길이가 slice panic이나 잘못된 line mapping을 일으키는가? | checked range와 N+1 line metadata를 확인하고 pattern을 1–2개로 제한한다. style/runtime tests 통과. |
| 6 | pattern의 breadth 변형이 track count와 Taffy sizing을 다르게 만드는가? | finite positive px minimum + exact `1fr` maximum만 admission한다. 0/non-finite/2fr 및 3-track inputs 거부. |
| 7 | row count가 width/gap-column 또는 column count가 height/gap-row를 읽는가? | axis별 size/gap property 매핑을 검토하고 native fixture의 columns와 rows frame을 각각 검증했다. 통과. |
| 8 | repeat count 산식에서 첫 pattern의 내부 gutter 또는 반복 사이 gutter가 빠지는가? | independent count tests와 Chrome 156 13-scene expected/runtime 대조에서 two-track/gap 경계를 확인했다. |
| 9 | UA 1px floor가 authored track sizing에 섞이는가? | count calculation에만 floor를 적용하고 typed track minimum은 authored 값 그대로 Taffy에 전달한다. subpixel test 통과. |
| 10 | arithmetic overflow 또는 large input이 과다 allocation으로 이어지는가? | finite/checked calculations와 `GRID_MAX_TRACKS` cap을 projection/layout에서 확인했다. overflow, cap tests 통과. |
| 11 | projection cell capacity와 Taffy expansion이 달라 implicit item이나 false reject를 만드는가? | 양 구현의 수식·pattern/gap 조건을 대조했고 Chrome 13-scene frames와 expanded-capacity negative case가 통과했다. 계산이 두 계층에 존재하므로 추후 변경 때 같은 경계 비교가 필요하다. |
| 12 | hidden item이 capacity/auto-fit occupancy를 소비하는가? | hidden node를 item/placement validator에서 제외하며 13-scene hidden/empty cases와 runtime no-partial case를 통과했다. |
| 13 | order, explicit placement, position, aspect ratio, self alignment가 auto-placement를 바꾸는가? | typed cascade와 layout validator가 모든 visible direct item을 검사한다. 26-case negative matrix 통과. |
| 14 | margin/padding/border/gap/alignment/direction/overflow 차이로 basis나 frame이 달라지는가? | Root profile의 엄격한 container contract를 확인했다. 해당 negative cases fail closed. |
| 15 | item 수 초과 또는 allocation/layout 실패 뒤 일부 frame이 publish되는가? | expanded cell capacity를 checked multiplication하고 runtime negative matrix에서 render snapshot 부재를 확인했다. 통과. |
| 16 | auto-repeat normalizer가 새 mode에서 implicit track fallback을 열어두는가? | 새 mode의 implicit auto-track expansion은 unsupported이며 item capacity 전에 전용 validator를 통과해야 한다. 통과. |
| 17 | native HTML fixture가 row repeat를 생략하거나 simulator가 다른 test asset을 실행하는가? | native fixture에 1-track columns, 2-track columns, rows repeat가 모두 있고 Rust runtime expected frames와 대조했다. |
| 18 | 실제 앱 번들이 다른 bootstrap HTML을 실행했는데 screenshot을 후보 근거로 잘못 연결하는가? | `root-html-default-rendering.html`은 다른 보조 asset임을 발견했다. Android APK `assets/index.html`과 iOS app `index.html`을 추출/해시해 native fixture SHA와 일치시켰고 default route 로그도 `source=index.html`이다. README에 구분을 추가했다. |
| 19 | simulator 결과를 실기기·하드웨어 GPU·pixel equality로 과장하는가? | 증거는 Android 17/API 37 ARM64 AVD 및 iOS 26.2 Simulator로만 표기한다. OS screenshots를 visual smoke로 한정하고 readback 미수행을 기록했다. |
| 20 | 제한 후보가 공식 C11 완료/집계 또는 계약 버전 상승으로 잘못 반영되는가? | STATUS activity에만 candidate 기록을 추가하고 checkbox와 `78/214`를 유지했다. 내부 계약 버전 `0.1.0` 및 일반 CSS 지원 범위도 그대로다. |

## 수정 및 남은 관찰

- Review 중 3,700줄 `grid.rs`에 새 validator가 집중된 점을 확인해 `grid/root_html_auto_repeat.rs`로 분리했다. 분리 후 focused tests, Clippy와 rustfmt를 다시 통과했다.
- Simulator asset 확인 과정에서 보조 HTML과 실제 default route `index.html`을 구분하고 두 플랫폼 bundle input hash를 추가했다.
- iOS Simulator app link log에는 `RawWindowMetalLayer` duplicate-symbol warnings가 남지만 build 및 default-route layout/draw/present는 통과했다. WGPU/FFI archive의 이 별도 warning은 이번 Grid 변경 범위 밖이며 해결 주장하지 않는다.
- 이 검토는 PR diff 검토다. 계획 검토 20개와 구현/runtime 검토 20개는 별도 파일에 보존한다.
