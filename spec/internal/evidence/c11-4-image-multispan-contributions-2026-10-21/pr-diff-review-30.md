# C11.4 direct image multi-span PR diff review 30개

**검토 대상:** 제한 image multi-span adapter/runtime, native verifiers, four-scene capture runner, Chrome fixture, roadmap/evidence/docs, screenshots and logs.

**결과:** 30개 관점을 확인했다. 차단 결함은 남지 않았다. 첫 실기기 PixelCopy에서 발견한 border-paint 제한은 fixture를 1px padding으로 바꾸어 동일 content geometry를 보존하고 결과에 기록했다. C11.4 전체 완료나 PNG texel paint 주장은 하지 않는다.

| # | 검토 관점 | 결과 |
| ---: | --- | --- |
| 1 | scene allowlist 외 ID가 image route에 들어오는가 | 네 ID만 고정 enum처럼 처리하고 unknown은 기존 bounded path로 간다. |
| 2 | 새 module이 fixture module boundary 밖에 놓였는가 | Rust source는 기존 `runtime_layout/fixtures/c11_4_image/` 및 FFI `fixtures/c11_4/` 아래다. |
| 3 | fixture route가 제품 ABI로 노출되는가 | runtime GPU fixture-test feature gate 안에 있으며 기본 FFI rlib/staticlib에서 scene ID가 없다. |
| 4 | generic image adapter에 영향을 주는가 | 네 root ID가 맞을 때만 새 adapter를 선택하며 일반 image scenes의 경로는 유지된다. |
| 5 | root namespace/tag/parent 검사가 누락됐는가 | HTML `div`, 문서 root의 direct child를 검사한다. |
| 6 | child topology가 과도하게 허용되는가 | 정확한 2/3 direct child count, HTML `img`, leaf topology를 검사한다. |
| 7 | DOM ID와 item order가 충돌하는가 | 각 scene별 고정 ID와 순서를 하나씩 대조한다. |
| 8 | 임의/원격 image source가 layout input을 바꾸는가 | fixture URL을 고정하고 network가 아닌 bounded PNG decode 경로에서 실패를 전파한다. |
| 9 | 여러 resource 완료의 dimensions가 섞이는가 | 한 snapshot에서 모든 item의 24×16 natural size와 intrinsic input을 확인한다. |
| 10 | item별 intrinsic revision이 달라 stale data가 되는가 | 한 `ImageResourceSnapshot`의 revision 1을 사용하고 세 플랫폼 trace에서 확인한다. |
| 11 | root border-box와 content area가 oracle과 다른가 | 40×60 border-box/1px padding은 Chrome과 앱의 38×58 content area 및 (1,1) inset을 만든다. |
| 12 | root padding/border paint를 scope보다 과장하는가 | Android border color 미출력을 근거로 border sample을 제거했고 padding geometry와 limitation을 기록했다. |
| 13 | track/row/gap grammar가 넓게 열렸는가 | 네 `min-content` column, 세 fixed row, zero gap만 accept한다. |
| 14 | automatic minimum 또는 flexible stage를 잘못 해석하는가 | Minimum 0과 span별 min-content values를 trace contributions로 대조한다. |
| 15 | span order가 DOM 순서에 의존하는가 | span-2 pair와 span-2/3 pair를 모두 forward/reversed로 실행했다. |
| 16 | span grouping stage가 누락되거나 재정렬되는가 | Chrome reference와 runtime trace에서 `[2]` 또는 `[2,3]` stage order를 확인한다. |
| 17 | rows가 columns trace와 섞이는가 | 20px 세 row의 separate Rows pass를 검사한다. |
| 18 | computed alignment의 unsupported `start`가 다시 들어오는가 | `justify-self:auto`/`align-self:auto`를 양쪽 입력에 고정하고 실제 런타임에서 통과했다. |
| 19 | image dimensions가 implicit stretching으로 커지는가 | 24×16, 48×32, 72×48 frame을 Chrome 및 네 scene runtime에서 확인한다. |
| 20 | placement grammar가 다른 span으로 오인되는가 | numeric row/column starts, span, automatic end line을 exact-match 검사한다. |
| 21 | expected track values가 runtime trace와 따로 노는가 | span-2 `[12,24,24,0]`, span-3 `[16,28,28,0]`을 final non-gutter tracks로 비교한다. |
| 22 | trace chunk 누락/절단을 JSON으로 잘못 처리하는가 | part index/count를 확인해 재조립한 뒤 JSON과 input/contribution/track fields를 검증한다. |
| 23 | frame check가 root 또는 역순 item을 빠뜨리는가 | 모든 node의 ID/order/x/y/width/height를 `1/64 CSS px` 이내로 비교한다. |
| 24 | sample 좌표가 root/item edge 안쪽에 머무는가 | root exterior, right overflow, below-root image, after-item 빈 좌표를 각각 둔다. |
| 25 | PixelCopy가 예상 target/color와 무관하게 성공하는가 | native adapter가 scene-specific node IDs, CSS positions, exact RGBA와 sample count를 대조한다. |
| 26 | Android 실기기 APK와 설치 파일이 다를 수 있는가 | SM-S731N local/installed APK SHA-256이 동일하고 device/GPU/API 정보가 결과에 남는다. |
| 27 | AVD 근거가 실제 기기 근거로 혼동되는가 | API 37 AVD 결과를 별도 폴더와 별도 device manifest로 둔다. |
| 28 | iOS Metal 표본이 draw log만으로 통과하는가 | Metal pixel surface opt-in을 켜 build하고 네 scene sample summary를 요구했다. |
| 29 | 브라우저 기준이 잘못된 revision과 연결되는가 | Chrome `155.0.8059.85` Agent Browser 관찰 JSON, 입력 hash, screenshot hash를 고정했다. |
| 30 | roadmap/status와 screenshot이 범위를 과장하거나 경로가 깨지는가 | 공개 roadmap/계획/evidence는 제한 slice로 적고 공식 status/count를 그대로 뒀다. 물리기기·AVD/iOS 화면 링크가 존재한다. |

## 상태

Rust formatting, focused runtime/FFI tests, default FFI artifact boundary, Android and iOS Simulator fixture builds, Android physical device, Android AVD, iOS Metal surface run, Agent Browser Chrome capture, and `git diff --cached --check` passed. WPT, generic CSS Grid support, image PNG texel paint, Android border paint, and full C11.4 remain outside this diff's claims.
