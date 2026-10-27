# C11.4 intrinsic auto-repeat 구현/runtime 실패 관점 검토 20개

검토 대상은 고정 WPT 8-case를 위한 Rust fixture·style projection·layout admission, Android/iOS adapter, capture runner와 2026-10-27 최종 simulator 실행이다. 각 항목은 서로 다른 실패 경로를 대상으로 삼았다. 계획 검토 항목은 재사용하지 않았다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 기준 WPT나 기대값이 다른 revision에서 조용히 바뀜 | source manifest의 pinned WPT revision·원본/helper hash와 독립 `expected.json` 8개 ID를 대조했다. |
| 2 | fixture scene ID의 누락·중복·순서가 oracle과 어긋남 | capture boundary test에서 8개 ID와 catalog 선택 순서·516 전체 ID 수를 확인했다. |
| 3 | fixture 코드가 기본 제품 runtime ABI로 들어감 | 기본·opt-in `.rlib` 비교에서 신규 `runtime_source` 심볼은 기본에 없고 opt-in positive control에만 있었다. |
| 4 | 알 수 없는 scene이 가장 비슷한 fixture로 잘못 매핑됨 | Rust adapter는 정확한 ID 검색 후 `None`; FFI dispatcher는 다른 route로 넘기며 unknown ID negative test를 가진다. |
| 5 | layout matcher가 다른 root나 여분 Grid를 수용 | root identity, 전체 node/container/item cardinality, root child 수, child leaf topology를 exact 검사한다. |
| 6 | width keyword·box edge·direction 등 다른 CSS 입력이 WPT로 오인됨 | root의 min/max-content, edges, border, padding, direction, overflow, gap 및 alignment를 typed value로 검사한다. |
| 7 | `justify-items:start`가 CSS cascade 뒤 projection에서 사라짐 | opt-in track-sizing profile에서만 typed container 값으로 읽고 Taffy의 `justify_items`로 전달한다. 기본값은 `Normal`이다. |
| 8 | 미지원 정렬값이 start처럼 축약됨 | runtime alignment parser와 profile rejection을 확인했다. regression projection test와 simulator trace/frame이 start 동작을 확인한다. |
| 9 | repeat/suffix의 다른 순서나 track 폭이 WPT match로 허용됨 | 반복 track·suffix·repeat fragment가 두 고정 track 배열 중 하나와 byte-for-byte typed shape로 일치해야 한다. |
| 10 | prefix 또는 두 개 이상 suffix에 intrinsic neighbor가 확장됨 | style projection과 layout test가 trailing single column에만 제한하며 prefix·multiple suffix·row-axis를 거부한다. |
| 11 | 음수·무한·NaN `minmax(px,auto)` 최소값이 Taffy에 전달됨 | projection/layout 경계가 finite nonnegative 값만 허용한다. 세 invalid numeric 경우의 negative test가 통과했다. |
| 12 | auto-fill과 auto-fit count가 neighbor contribution을 다르게 적용 | 양쪽 mode에서 같은 expected count·columns를 검사한다. 8/8 scene마다 반복 count와 final columns가 oracle과 일치했다. |
| 13 | auto-fit이 spanning item의 repeated track을 빈 track으로 접음 | in-flow item은 explicit repeat와 suffix 전체를 span한다. final CSS track은 모두 non-collapsed로 확인했다. |
| 14 | absolute probe가 intrinsic contribution 또는 auto-repeat count를 바꿈 | probe 4개는 exact placement·absolute inset profile이어야 하며 track trace contribution에는 in-flow item ID 2만 허용한다. |
| 15 | 0-width probe가 painted box count 및 surface request에 포함됨 | node frame에는 0-width probe를 보존하고 box count와 Pixel/Metal sample 집합에서는 제외한다. 두 플랫폼 실행과 node frame이 일치했다. |
| 16 | track trace의 pass 순서·stage·contribution 종류가 틀려도 앱이 통과 | Android·iOS matcher와 capture verifier가 Columns/Rows, pass sequence, stage, item/span/value/kind를 검사한다. |
| 17 | auto-track stretch 이후 값이 달라져도 pre-stretch 값으로 성공 처리 | 검토에서 발견한 결함이다. Android/iOS matcher를 마지막 `StretchAutoTracks` stage의 track으로 바꾸고 Android/iOS 8/8을 다시 실행했다. |
| 18 | malformed Android trace stage 배열이 null dereference를 일으킴 | 같은 검토에서 발견했다. 각 stage object를 먼저 검사해 null이면 `false`로 거부하도록 보완했다. 최종 Android build·8/8 실행이 통과했다. |
| 19 | overflow sample이 border, item 내부, item 외부와 다른 위치를 읽음 | Oracle의 root-relative x=73/84/103,y=20과 target node를 native preflight에서 exact 확인하고, Android·Metal RGBA 결과가 각 3/3 exact였다. |
| 20 | 오래된 앱 설치·부분 trace·미제출 draw를 새 실행 성공으로 오판 | 최종 실행은 local/installed APK·app hash 일치, complete trace chunk, `drawPresented:true`, retry 0, 고유 run prefix와 장면별 log/PNG를 남겼다. |

두 발견은 최종 플랫폼 빌드와 전체 8-scene 재실행 전에 수정했다. 최종 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator가 각 8/8 통과했다. PR diff 검토는 별도 30개 관점 문서로 남긴다. 일반 CSS admission, 실기기, WPT runner의 Spinon 적합성 및 전체 C11.4는 이 검토로 확정하지 않는다.
