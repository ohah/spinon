# C11.4 두 anonymous text run + 세 image bordered Grid PR diff 실패 관점 검토 30개

**검토 대상:** PR에 포함할 최종 runtime/FFI adapter, Android/iOS fixture gates, simulator capture tool, plan 및 evidence 변경. 구현 review 20개와 별도로 파일 간 결합과 PR 범위에서 재검토했다.

| # | 실패 관점 | diff 확인과 판정 |
| --- | --- | --- |
| 1 | 계획의 scene ID와 runtime allowlist가 다른가? | 두 새 ID가 Rust runtime, FFI source, Android Java, iOS Swift 및 capture tool에 동일하게 있다. 통과. |
| 2 | source-order reversed scene이 실제 자식 순서도 뒤집는가? | Runtime/FFI fixture와 CSS capture scene의 child sequence를 대조했다. placement·resource·size는 그대로고 child order만 반전한다. 통과. |
| 3 | legacy two-image IDs가 새 IDs로 바뀌어 기존 caller가 끊기는가? | 기존 두 ID와 frame/track expectations를 보존했다. 최종 simulator matrix에서 기존 장면 4/4가 회귀 통과했다. 통과. |
| 4 | 예기치 않은 three/four image count를 부분 성공으로 받아들이는가? | Runtime은 2 또는 3만 허용하고 FFI child count와 ordered image IDs를 확인한다. 앱 draw count도 exact한다. 통과. |
| 5 | third image가 source order마다 잘못 연결될 수 있는가? | A/B/C image IDs를 scene별 순서로 고정하고 document child order 및 static PNG attachment를 검증한다. 통과. |
| 6 | PNG payload가 default product artifact에 노출되는가? | 경계 script가 새 ID/source marker와 exact payload를 기본/opt-in archive에서 대조한다. 기본 누락, positive control 존재다. 통과. |
| 7 | third image span 값이 특정 adapter에서만 2로 남는가? | Runtime style, FFI HTML source, Java/Swift diagnostic spans, capture expected가 `[1,2,3]`으로 합치한다. 통과. |
| 8 | expected columns가 Chrome observation을 복사했는가? | 독립 `[18,34,20,20]`과 Chrome 156 `[16,32,24,20]`을 plan/evidence에 나눠 기록한다. 통과. |
| 9 | track trace에서 fourth track이 누락되는가? | Android/iOS validators가 네 columns와 세 rows를 검사하고 simulator JSON에도 full arrays를 보존한다. 통과. |
| 10 | anonymous text intrinsic inputs가 scene child order를 반영하지 않는가? | 각 scene의 node ID, `sourceOrder`, text source, 20×10 intrinsic input과 frame 배열을 exact 순서로 확인한다. 통과. |
| 11 | layout expected와 paint node expected가 섞이는가? | Text item은 layout diagnostics에서 검사하고 root/image만 painted box count와 node-frame list에 둔다. scene summaries가 이에 맞는다. 통과. |
| 12 | root border-box 대신 content-box를 사용해 overflow를 계산하는가? | frame test는 root `40×184`, content width 36, overflow 계산은 outer right edge x=40을 사용한다. 통과. |
| 13 | image C frame 자체가 34px overflow를 만들지 않는가? | `x=2,width=72`에서 right edge 74이므로 root edge 40보다 34px 크다. 두 source order 모두 이 값을 확인했다. 통과. |
| 14 | root-local PixelCopy sample을 surface coordinate로 직접 혼동하는가? | Capture tool이 CSS coordinates와 scale을 전달하고 AVD log에 transformed surface points를 보존한다. 통과. |
| 15 | 새 Android sample ID의 RGB 기대값이 없는가? | 초회 mismatch를 재현했고 `image-c-overflow-green-png-texel`을 `#00ff40` lookup에 추가했다. 최종 PixelCopy 8/8 exact다. |
| 16 | Android scene allowlist가 새 ID를 거부하는가? | 초회 `SPINON_C114_GRID_ARGUMENT_ERROR` 원인을 `C114_GRID_TRACK_SIZING_SCENE_IDS`에서 확인하고 두 ID를 추가했다. 최종 AVD 네 장면 통과다. |
| 17 | iOS scene allowlist는 Android와 다른가? | Swift allowlist와 track/frame/sample branches에 같은 두 ID를 추가하고 iOS 26.2 Simulator 네 장면에서 통과했다. |
| 18 | iOS drawable pixel result를 OS compositor output이라고 잘못 부르는가? | Evidence와 plan에서 compositor 전 readback으로 범위를 제한한다. 통과. |
| 19 | Surface pixel sample 수가 일부 scene에서 줄어드는가? | Capture verifier는 각 scene에서 border와 overflow 두 sample을 요구한다. 네 scene 각각 matched 2/2다. 통과. |
| 20 | capture JSON의 draw summary가 실제 세 image draw를 확인하지 않는가? | `runtime_image_count`와 `image_draw_count`를 scene image 수로 비교하며 final logs는 각 수 2 또는 3이다. 통과. |
| 21 | 총 scene count가 추가 장면 수와 어긋나는가? | 기존 171에서 두 새 scene을 더해 173으로 수정했고 unique-ID assertion을 유지한다. 통과. |
| 22 | row axis trace 또는 source revision이 누락되는가? | 두 axis의 track values/stages와 layout revision이 Android/iOS runtime report에서 검증된다. 통과. |
| 23 | 새 test가 기존 test를 제거하거나 weakened assertion으로 바꾸는가? | 기존 두-scene expectations를 data-driven 4-scene matrix로 넓히고 each frame/overflow/span assertions를 유지한다. 통과. |
| 24 | FFI test가 unknown ID 또는 span-3 positive control을 놓치는가? | 새 test는 네 scene IDs, image count, span-3 source, URL와 unknown ID rejection을 검사한다. 통과. |
| 25 | iOS/Android screenshots가 검증 scene과 다른 binary에서 나왔는가? | 각 run metadata에 빌드 hash를 기록하고 Android installed APK hash가 local APK와 일치한다. 통과. |
| 26 | PR에 넣을 screenshot이 빠졌거나 path가 잘못됐는가? | 최종 four-scene run에서 forward/reversed Android·iOS PNG가 모두 생성됐고 각각 1080×2400 및 1206×2622다. PR에는 commit-pinned raw image URLs를 쓴다. 통과. |
| 27 | test/build 결과가 출력이 아닌 exit code만으로 기록됐는가? | Runtime/FFI tests, both Clippy runs, app builds, module-boundary test와 archive scan의 로그·JSON을 evidence에 보존했다. 통과. |
| 28 | fixture-only style limitation이 성공 설명에서 숨겨지는가? | Explicit `overflow:visible` declaration의 unsupported profile 경계를 적고 CSS initial visible 및 실제 surface evidence와 분리한다. 통과. |
| 29 | WPT, physical device, 전체 PNG comparison 또는 full C11.4를 암시하는가? | Scope text는 네 고정 scene과 단일 pixel samples에 제한하고 physical/WPT/compositor-after/full C11.4를 완료로 표시하지 않는다. 통과. |
| 30 | 이 slice가 공식 완료 수를 올리는가? | Plan, evidence와 PR diff 모두 C11.4 checkbox 미완료 및 공식 `72/213`을 유지한다. 통과. |

최종 review에서 추가 코드 변경이 필요한 잔여 결함은 없었다. 초회 Android scene/sample expectation 누락은 이 기록에 남긴 뒤 고쳐 재빌드·재실행했다. PR에 추가할 image URL은 push 뒤 해당 commit SHA를 pin하고 Agent Browser에서 실제 로드를 확인한다. GitHub required checks 및 merge 상태는 PR 생성 후 별도 기록한다.
