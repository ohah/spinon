# C15.3 non-zero scroll text transform 구현 실패 관점 20개

구현 코드와 runtime snapshot, fixture, iOS/Android surface 관측을 계획 검토와 독립적으로 점검했다. 모든 항목은 본 slice가 지원하는 고정 Ahem fixture 범위만 판정한다.

| # | 실패 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | 수평 scroll을 glyph origin에 적용하지 않거나 반대 방향으로 적용 | `3→6→0`을 base origin 기준 `-3→-6→0`으로 확인했다. renderer 95/95와 fixture report가 통과했다. |
| 2 | 수직 scroll을 x축에 적용하거나 수직 이동을 누락 | `18→7→0`에서 y origin만 이동하고 PixelCopy/Metal 모두 scroll ink를 읽었다. |
| 3 | nested offset 둘 중 하나만 합산 | `4+5→2+3→0+0` 결과가 x 이동 `9→5→0`이다. |
| 4 | 새 offset을 이미 움직인 origin에 누적해 반복 변경 때 drift | base origin을 보존하고 각 snapshot에서 현재 offset 합으로 다시 계산한다. 세 sequence의 reset이 원점과 일치했다. |
| 5 | 한 축 scroll이 다른 축 좌표를 바꿈 | 수평은 y를, 수직은 x를 보존하는 unit assertion과 runtime origin report를 통과했다. |
| 6 | scroll owner의 scrollport가 자기 offset 때문에 함께 움직임 | owner box는 base frame으로 재계산하고 자식만 scroll offset을 적용한다. 기존 owner clip 고정 회귀와 nested clip unit test를 통과했다. |
| 7 | outer scroll이 nested inner scrollport 위치를 갱신하지 않음 | nested test에서 inner owner clip은 outer offset만 따라 이동하고 inner 자기 offset에는 고정됨을 확인했다. |
| 8 | text가 inner clip만 사용하고 outer clip을 누락 | active owner paint clip과 base text clip의 교차를 사용한다. Android와 iOS outer boundary ink 및 nested sample이 기대 색과 맞았다. |
| 9 | text origin 변환에서 base text clip을 잃음 | `base_clip_css_px`를 보존해 active owner clip과 매번 재교차한다. 기존 visible/hidden clip 사례와 simulator 3개 경계 표본을 통과했다. |
| 10 | 같은 scroll 상태의 새 revision에서 text source가 stale로 승인 | 기존 `RuntimeRenderText::source` key 검사를 유지했다. stale source 회귀 테스트가 통과했다. |
| 11 | metadata가 등록되지 않은 scroll owner를 참조 | `scroll_offset_sum`은 미등록 owner에서 `InvalidScrollState`를 반환한다. missing-container test가 통과했다. |
| 12 | ancestor metadata에 동일 owner가 두 번 있어 offset을 이중 적용 | 중복 owner를 fail-closed 처리하고 전용 negative test를 추가했다. |
| 13 | scroll offset이 NaN·무한대·허용 범위 밖 | container 생성/offset 교체 검사가 기존 validation 오류로 거부한다. |
| 14 | 서로 유한한 offset 합이 f32 범위를 넘음 | offset 합의 finite 검사를 유지하며 `InvalidScrollState`를 반환한다. |
| 15 | base origin과 offset 연산 결과가 유한하지 않음 | `apply_scroll_transform`가 `InvalidTextGeometry`를 반환하도록 했다. |
| 16 | text owner 자신이 scroll container인 미지원 구조를 실수로 허용 | 기존 `UnsupportedTextScroll` 경계를 유지했고 전용 회귀가 통과했다. |
| 17 | scroll metadata가 없거나 box 수와 맞지 않음 | snapshot 생성의 기존 길이·frame 검사에서 `InvalidScrollState`로 거부한다. |
| 18 | moved glyph가 paint command 순서 또는 glyph bitmap을 바꿈 | text clone은 bitmap과 key를 보존하고 마지막 `DrawText` order를 유지한다. 기존 paint command test와 WGPU glyph suite가 통과했다. |
| 19 | glyph가 clip 밖에 paint되거나 scroll 후 owner hit-test가 다름 | runtime sample마다 text mask와 effective clip을 함께 검사하고 owner hit-test를 확인한다. overflow 및 scroll 표본 6개가 Android/iOS에서 일치했다. |
| 20 | 플랫폼 표본 개수·좌표·drawable scale이 잘못되어 거짓 pass | FFI는 6개만 받고, 앱 parser는 알려진 이름·서로 다른 CSS/pixel 좌표·viewport 범위를 확인한다. Android PixelCopy와 iOS Metal drawable이 각각 6/6 exact였다. |

경계: 이는 고정 runtime fixture의 paint와 box-owner hit-test 결과다. 일반 DOM text admission, glyph별 DOM hit target, scrollbar와 실기기 GPU를 검증하지 않는다. C15.3과 상위 C15 체크는 미완료로 남는다.
