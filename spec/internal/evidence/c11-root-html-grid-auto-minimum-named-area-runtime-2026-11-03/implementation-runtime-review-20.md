# C11.1 named-area 자동 최소 크기 구현·runtime 실패 관점 검토 20개

검토 대상은 현재 Rust projection·기본 Root HTML runtime 변경, Android/iOS opt-in capture adapter, 고정 HTML 입력, 최종 [platform observation](./pass-final/platform-observations.json)과 [원본 실행 로그](./pass-final/)다. 각 항목은 구현 동작 또는 런타임 경계를 별도로 확인했다. 계획 검토 20개와 실제 PR diff 검토 30개는 별도 기록이다.

| # | 검토한 실패 경로 | 판정과 조치 |
| --- | --- | --- |
| 1 | named-area가 없는 일반 Root HTML에도 새 auto-minimum `minmax()` admission이 번짐 | 불가. 새 축 분류 함수는 named-area template validator에서만 호출되고 no-area profile은 별도 regression으로 확인했다. |
| 2 | columns matrix가 rows matrix로 잘못 분류되거나 track order가 뒤바뀜 | 불가. 정확한 `[minmax(auto|0px,fr),40px]`/`[20px,20px]` 및 반대 축 `[40px,40px]`/`[minmax(auto|0px,fr),40px]` 두 형태만 양 축 테스트에 고정했다. |
| 3 | auto minimum 이웃 track이나 definite basis 없이 계산됨 | 불가. 첫 track과 fixed neighbor의 개수·순서·값, root content dimension을 함께 검사한다. |
| 4 | 음수·NaN·무한 `fr` factor가 matrix에 들어옴 | 불가. track classifier가 유한 non-negative 값만 받으며 기존 수치 rejection과 함께 검사한다. |
| 5 | `minmax(0,1fr)` control이 `minmax(auto,1fr)`처럼 넘침 | 불가. 두 축 control은 `[60,40]`/`[60,40]` used track과 root 밖 sample의 흰색을 기대한다. |
| 6 | `minmax(auto,0fr)`의 flex factor가 auto minimum을 지움 | 불가. columns·rows 0fr 경계를 각각 기대 `[80,40]`, 18px 바깥 spill 및 파란 표본으로 검사했다. |
| 7 | 0 minimum control이 CSS computed `0px`와 다른 내부 enum으로 빠짐 | 불가. typed computed breadth `LengthPx(0)` 비교와 두 축 positive matrix에서 확인했다. |
| 8 | area map의 spanning header가 minimum contribution을 유발하거나 잘못된 item frame을 숨김 | 불가. 전체 header/side/content frame을 독립 산술과 대조하며 single-track contributor와 spanning header를 분리했다. |
| 9 | content-box 크기와 border-box 크기를 혼용해 18px overflow가 잘못 계산됨 | 불가. root는 content `100×40` 또는 `80×100`, 네 변 2px border이며 root border box와 border 밖 spill을 각각 검사한다. |
| 10 | width/height·min/max·aspect ratio가 track sizing에 끼어들지만 검증하지 않음 | 불가. definite 축 크기, `min-size:auto`, `max-size:none`, `aspect-ratio` 부재를 container와 item에서 확인하고 override negatives를 둔다. |
| 11 | scroll/clip/hidden item이 automatic minimum을 0으로 바꿈 | 불가. Root와 direct item overflow는 visible만 받으며 hidden negative가 node/property 오류로 닫힌다. |
| 12 | item이 area와 다른 grid line placement를 동시에 제공해 다른 cell에 배치됨 | 불가. 네 computed edge가 같은 exact area alias를 가리키는지 확인한다. |
| 13 | DOM ID/class가 성공 허용 목록 역할을 하거나 child source order가 area를 바꿈 | 불가. ID/class를 바꾼 positive fixture에서도 typed `grid-area` 해석 결과가 같은 frame을 만든다. |
| 14 | 두 개 이상의 named-area container 또는 nested/additional participating item이 승인됨 | 불가. direct child·Grid owner 개수를 검증하며 extra item negative를 통과했다. |
| 15 | unsupported track이 일부 frame을 만든 뒤 완성 revision으로 보임 | 불가. unsupported style runtime은 `Failed`, completed layout 없음, cascade/layout request key 일치를 확인한다. |
| 16 | 같은 화면에서 JS class/style mutation 후 계산 결과를 새 revision으로 오인 | 현재 mutation 경로는 cascade revision을 게시하지 않아 이 harness에 쓰지 않는다. 여섯 개 정적 Root HTML source를 각각 새 session에 넣어 회귀를 검증하고 동적 style mutation 지원을 주장하지 않는다. |
| 17 | inline border 축약이 예상 밖 computed `border-top-color`를 만들어 fixture가 격리 실패 | 재현 후 fixture를 허용된 `border-width`와 `border-style`로 바꾸고 원래 shorthand는 쓰지 않았다. |
| 18 | auto-minimum 추가가 fixed implicit row no-area 경계를 우발적으로 넓힘 | 재현했다. fixed implicit row와 auto-minimum `minmax()` 조합을 거부하도록 guard를 추가했으며 기존 19 profile 검사와 68 runtime Grid 회귀가 통과했다. |
| 19 | Android Activity가 재사용되어 직전 scene의 launch extras를 PixelCopy가 읽음 | 최초 캡처에서 재현했고 `am start -S`로 fresh launch한다. 최종 여섯 run ID별 summary와 sample 두 개가 각 PNG와 묶였다. |
| 20 | iOS surface가 다른 generation·scene의 표본을 읽거나 최종 화면을 compositor 이후로 오해 | run ID, surface generation, scene, drawable size, sample RGB를 로그와 연결했다. Metal readback은 compositor 전임을 명시했고 iOS simulator screenshot은 별도 시각 자료다. |

최종 재빌드 행렬은 Android API 37 AVD PixelCopy 12/12와 iOS 26.2 Simulator Metal drawable 12/12 exact다. Android는 SwiftShader software 경로이고 iOS는 compositor 전 surface라 실기기/hardware 및 최종 display color 보장은 하지 않는다. 기본 feature-off app artifact 대조는 [결과](./artifact-boundary/result.txt)에서 확인했다.
