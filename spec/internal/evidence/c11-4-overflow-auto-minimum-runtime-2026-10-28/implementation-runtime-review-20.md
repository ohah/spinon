# C11.4 overflow automatic minimum 구현/runtime 실패 관점 20개

최종 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 결과, cascade·projection·layout 검증 경로를 대조했다. 각 항목은 별도 실패 조건이다.

1. **Feature-off 누출:** 새 cascade API와 fixture module은 `c11-4-grid-track-sizing-fixture-test`/runtime fixture cfg에 묶인다. 기본 FFI archive에서 scene marker가 없어 통과.
2. **유사 ID가 새 route 선택:** route는 고정 prefix를 쓰지만 알려진 다섯 root ID만 `prepare`가 받아들인다. 알 수 없는 suffix는 `Unsupported`로 끝나며 일반 layout을 통과시키지 않아 통과.
3. **root identity 위조:** XHTML namespace, `div`, 문서 root parent 및 track-sizing profile을 함께 검사한다. 불일치 입력을 거부해 통과.
4. **Grid child 누락·추가:** root direct child가 하나가 아니거나 부모 관계가 다르면 입력 준비를 거부해 통과.
5. **item identity 변경:** 고정 item ID/tag가 다르면 고정 scene 처리 대신 오류로 종료해 통과.
6. **text 입력 변조:** 자식 text node 수, parent 및 정확한 Ahem text를 검사한다. 글자 수가 바뀐 intrinsic contribution을 고정 기대값에 섞지 않아 통과.
7. **font/layout 환경 변화:** horizontal writing mode, Ahem 10px, root/item dimension·margin·padding·border·gap·placement를 고정값과 비교해 통과.
8. **overflow axis computed-value 누락:** authored x와 실제 computed x/y를 별도로 확인한다. hidden/scroll의 y=`auto` 변환 및 clip의 y=`visible`을 양 플랫폼에서 확인해 통과.
9. **일반 Grid profile 확대:** 기본 track-sizing cascade는 overflow를 허용하지 않고, 전용 fixture cascade만 x/y를 허용한다. base profile 거부 test 통과.
10. **새 허용 속성으로 인한 inline CSS 우회:** fixture property allowlist가 overflow 외의 미지원 선언을 허용하지 않고 text-orientation 별도 preflight를 유지해 통과.
11. **author stylesheet 우회:** author property override는 fixture cascade 호출에서만 사용하며 origin/stylesheet preflight를 유지한다. 기본 profile에는 전달되지 않아 통과.
12. **stale style 재사용:** marker root는 previous style snapshot 재사용에서 빠진다. 전용 route가 이전 overflow computed 값에 의존하지 않아 통과.
13. **typed track 입력 변조:** columns/rows, `TrackSizing`, auto-repeat, template areas를 고정 계약과 비교한다. 새 검증은 auto-flow/implicit track과 실제 비어 있지 않은 line name도 거부해 통과.
14. **trace 누락·다중 Grid:** root trace 한 개와 Columns/Rows pass 두 개를 요구한다. 누락·추가·다른 node trace를 거부해 통과.
15. **item Minimum 누락·중복:** item/span의 Minimum 기여 하나와 고정 value를 요구한다. 별도 FFI matcher에서도 item/span을 확인해 통과.
16. **minimum source 혼동:** visible/clip은 span·auto-min track·content-based 조건을 담은 `AutomaticMinimum`, scrollable 값은 `OverflowAutomaticMinimum`이어야 한다. 다섯 scene에서 확인해 통과.
17. **MaxContent과 Minimum 혼동:** 모든 scene에서 MaxContent contribution 하나, 100px 유지 여부를 검사한다. Android/iOS trace에서 통과.
18. **gutter/collapsed track 오인:** final Columns stage에서 gutter·collapsed track을 제외한 base size 두 개를 expected 값과 비교한다. 50/50 또는 40/40으로 통과.
19. **frame 중복·추가 누락:** 초기 검증은 map으로 중복 frame을 덮어쓸 수 있었다. vector 길이와 고유 node 길이가 모두 2인지 확인하도록 고친 후 AVD/iOS final run 5/5 통과.
20. **pixel 증거 과장:** Android PixelCopy와 iOS 제출 전 Metal 표본의 좌표·색·scene 연결을 검증한다. iOS compositor 이후, 실기기, clipping semantics라고 해석하지 않도록 결과 범위를 제한해 통과.

## 실행 결과

최종 코드의 focused style/FFI tests, `cargo check`, fixture-feature library Clippy, rustfmt가 모두 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator가 각각 5/5 scene의 frame·tracks·trace·draw/present를 통과했고, 각 scene의 두 boundary pixel 표본도 exact match했다. 초기 retry에서 발견한 빈 line-name group 검증 오류와 frame cardinality 누락은 수정 후 다시 확인했다.
