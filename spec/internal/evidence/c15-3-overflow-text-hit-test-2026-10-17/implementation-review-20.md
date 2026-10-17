# C15.3 overflow text-owner hit-test 구현 실패 관점 20개

이 검토는 renderer API를 바꾸지 않고 고정 fixture route가 owner, clip과 순서를 올바른 지점에서 관찰하는지 확인한다. 각 행은 다른 실패 조건을 대상으로 한다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | sample CSS 좌표를 PixelCopy physical 좌표로 잘못 넘겨 hit-test한다 | Rust hit-test는 CSS pixel center를 받고 PixelCopy는 독립적으로 DPR 배율을 적용한다. 통과. |
| 2 | 정수 좌표 경계가 반열린 box edge 의미를 바꾼다 | fixture는 고정 픽셀의 중심 `+0.5 CSS px`에서 검사한다. 통과. |
| 3 | visible 점이 owner frame 밖이라 row/root fallback만 반환한다 | geometry가 `(13,81) 40×10`이며 sample은 `x=29.5`다. visible owner NodeId가 선택됐다. 통과. |
| 4 | visible parent의 overflow clip이 owner에 잘못 전파된다 | 기본 fixture에서 visible 점은 owner다. visible owner에 clip을 넣은 negative control은 admission assertion에서 실패한다. 통과. |
| 5 | hidden 안쪽 점까지 조상 clip으로 잘못 제외한다 | `(13.5,121.5)` sample이 hidden owner로 돌아왔다. 통과. |
| 6 | hidden 밖 점에서 clip이 누락되어 hidden owner를 반환한다 | `(29.5,121.5)`는 hidden owner를 반환하지 않는다. underlying target은 허용한다. 통과. |
| 7 | hidden 밖 결과를 반드시 `None`으로 강제해 ancestor fallback 계약을 바꾼다 | assertion은 hidden owner 배제만 확인한다. 통과. |
| 8 | sample이 glyph ink가 아니라 빈 곳에 놓인다 | 기존 Ahem sampler가 alpha `255` bitmap pixel만 선택한다. 통과. |
| 9 | simulator 표면의 동일 위치에는 glyph가 그려지지 않았다 | Android/iOS sample은 고정 ink RGBA로 각각 3/3 exact다. 통과. |
| 10 | hidden 외부를 hidden owner의 배경으로 잘못 통과시킨다 | Android/iOS PixelCopy·Metal drawable이 stage 배경 RGBA를 반환한다. 통과. |
| 11 | Ahem font 누락으로 fallback 글꼴이 oracle가 된다 | 고정 font asset bytes를 opt-in fixture가 직접 등록한다. 통과. |
| 12 | text owner가 render snapshot의 box에 바인딩되지 않는다 | `RuntimeRenderSnapshot` 생성이 동일 NodeId owner 존재를 검사하고 sample가 owner를 반환한다. 통과. |
| 13 | scene revision이 다르지만 text hit만 우연히 성공한다 | 기존 stale text source 거부 검사는 유지되고 전체 fixture attach가 동일 key로 성공한다. 통과. |
| 14 | hidden clip이 paint-only이며 hit-test clip에는 반영되지 않는다 | hidden 밖 sample의 owner 배제로 hit-test clip 경계를 확인한다. 통과. |
| 15 | paint order가 아닌 text command가 overlay를 덮는다 | 더 높은 순서 overlay를 같은 visible sample에 둔 negative control이 `Some(overlay NodeId)`를 반환하지 않으면 통과하지 못한다. 통과. |
| 16 | 동일 위치의 unrelated hidden box가 visible sample을 가린다 | 두 owner는 `y=81`과 `y=121`의 별도 line에 있고 fixture point가 분리되어 있다. 통과. |
| 17 | viewport 바깥 sample이 box hit-test로 허용된다 | 고정 세 점은 모두 360×240 CSS viewport 안이며 기존 viewport guard를 통과한다. 통과. |
| 18 | test route가 기본/제품 feature에 공개된다 | 추가 로직은 기존 `c15-3-wgpu-glyph-paint-fixture-test` opt-in module에 있다. 통과. |
| 19 | simulator screenshot을 event dispatch 증거로 잘못 사용한다 | report assertion과 image pixel readback을 별도 증거로 분리했다. 통과. |
| 20 | 제한된 owner-box probe를 glyph hit, DOM event 또는 C15 완료로 넓혀 쓴다 | contract·evidence·status에서 해당 기능과 상위 완료 체크를 제외한다. 통과. |
