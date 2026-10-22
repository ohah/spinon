# C11.4 cyclic `minmax(calc())` replaced image runtime 검토 20개

구현 뒤 runtime 실패 경로를 서로 다른 불변 조건 기준으로 검토했다. root style 제한과 Android harness의 scene allowlist, PixelCopy scene match, reversed input 기대값 누락을 찾아 보강하고 영향 범위의 build·simulator 관문을 다시 확인했다.

1. **알 수 없는 root ID가 새 fixture로 잘못 선택되는가?** Runtime adapter는 고정 scene 배열의 완전 일치만 받으며 알 수 없는 ID는 이 adapter에서 처리하지 않는다. FFI scene allowlist도 별도로 닫혀 있다.
2. **다른 namespace, tag 또는 root 밖의 같은 ID를 받는가?** HTML namespace, `div`, document root의 직접 자식 조건을 확인한다.
3. **root의 child 수가 달라도 일부만 계산하는가?** scene별로 정확히 세 child만 허용하고 나머지는 거부한다.
4. **역순 fixture가 DOM 위치와 image identity를 혼동하는가?** Runtime은 scene order의 key·expected ID·placement를 비교한다. Android 진단 입력은 고정 node ID/sourceOrder `[2,3,4]`를 유지하고 frame만 C/B/A로 뒤집는지 확인한다.
5. **이미지가 직접 leaf가 아닌 경우에도 contribution에 포함되는가?** 각 image가 root의 직접 자식이고 descendant가 없는지 검사한다.
6. **같은 크기의 다른 이미지 URL을 fixed PNG로 오인하는가?** 세 image 모두 내부 고정 URL과 일치해야 한다.
7. **깨진 PNG가 이미지 크기만 가진 것으로 통과하는가?** `ImageRequest` 생성과 PNG decode 실패를 구체적인 fixture 오류로 전파한다.
8. **한 image만 natural size가 다르게 들어오는가?** snapshot 안의 각 node decoded dimensions를 24×16px와 대조한다.
9. **natural aspect ratio 입력이 빠져 height 지정·width auto를 잘못 다루는가?** style/layout test가 세 item의 replaced natural width/height와 3:2 ratio를 검사한다.
10. **resource revision overflow나 snapshot 조립 오류를 조용히 무시하는가?** revision 증가, `ImageResourceSnapshot`, layout intrinsic snapshot 오류는 모두 fail closed다.
11. **다른 CSS style profile에서 fixture 전용 예외가 활성화되는가?** RuntimeBlockFlexGridTrackSizingV1 profile만 받는다.
12. **root min/max size 또는 aspect ratio가 예상 track sizing을 바꾸는데 통과하는가?** 초기 검토에서 검증 누락을 찾았다. Root의 `min-width/height:auto`, `max-width/height:none`, aspect ratio 없음 조건을 추가했고 runtime focused test 및 iOS app build가 통과했다.
13. **border, padding, margin, gap 또는 positioning 차이가 root frame을 바꾸는가?** root 2px border, 0 padding/margin, 2px row/column gap, static positioning을 고정한다.
14. **근사하게 비슷한 track 문법이 새 CSS admission으로 노출되는가?** 3 columns와 2 rows 각각에서 `minmax(calc(10% + 2px), max-content)` AST 구조와 term을 검사한다.
15. **auto-repeat, named lines, template areas, implicit auto tracks 또는 다른 auto-flow가 통과하는가?** 해당 속성은 모두 정확한 고정 기본값만 허용한다.
16. **image의 min/max, aspect ratio, border, padding, margin 또는 position이 크기 추론을 바꾸는가?** 세 item 각각의 width:auto, 지정 height, 기본 min/max, ratio 없음, 0 edges, static position과 exact placement를 확인한다.
17. **intrinsic 두 축 예외가 기존 C11.4 image fixture에 퍼지는가?** 새 네 scene만 flag를 켜고 기존 및 nested image adapters는 `false`를 명시한다. Android harness도 네 ID만 exact allowlist로 추가한다.
18. **replaced contribution이 A/B/C span·축·natural size와 분리되는가?** style/layout trace는 각 node identity, source order, natural size와 inline/block contribution 값을 검사한다.
19. **overlap된 span-2 suggestion이 DOM 순서에 따라 누적되거나 span-3이 먼저 적용되는가?** 네 scene에서 두 축 pass 순서, span group `[2,3]` / `[2]`, per-track maximum 및 final tracks를 검사한다.
20. **계산값만 맞고 화면 draw/pixel 확인이 빠지는가?** 최신 tracked source revision으로 iOS 26.2 Simulator와 Android API 37 ARM64 AVD 네 장면 모두 node frame/track trace와 `drawPresented`가 통과했다. Metal 및 PixelCopy 표본은 각 scene 3/3이다. 이는 compositor 이후 pixel이나 실기기 검증을 뜻하지 않는다.

**검토 결과:** root intrinsic constraints 누락과 Android harness의 scene allowlist, PixelCopy scene match, reversed input 기대값 누락을 수정했다. Style/layout 네 scene test, runtime scene matrix test, `cargo fmt --all -- --check`, Android/iOS app build와 두 simulator의 네 scene runtime/pixel gate가 통과했다. 실기기, hardware GPU, iOS compositor 이후 pixel 및 전체 C11.4 범위는 확인하지 않았다.
