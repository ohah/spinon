# C18.2 계획 적대 검토 — 2026-10-16

구현 전 계획을 20개의 서로 다른 실패 관점으로 검사했다. 검토는 Chrome 155.0.8059.85 Agent Browser 캡처, 24×16 PNG 바이트, 기존 C18.1 WGPU image pipeline, Android/iOS C18.1 opt-in 경로에 대조했다. 이 문서는 계획 검토 기록이며 구현 검토에 재사용하지 않는다.

## 검토 대상

- 계획: [C18.2 정적 PNG alpha source-over](../../../plan/c18-2-static-alpha-image-compositing.md)
- 독립 oracle: [Chrome screenshot·10개 sample·input hash](./c18-2-alpha-precomparison-2026-10-16/README.md)
- 구현 전 renderer 기준: `crates/spinon-render-wgpu/src/renderer/images.rs`는 alpha가 255가 아닌 texel을 거부하고 image fragment는 alpha를 1로 내보낸다. image pipeline은 blending을 비활성화했다.
- 기존 화면 경로: C18.1은 별도 V8 source·FFI·Android adapter·iOS route를 사용하며 AVD PixelCopy와 Metal drawable readback을 전체 screenshot과 분리한다.

## 실패 관점 20개

1. **0 alpha에 숨겨진 RGB가 번짐** — 투명 texel RGB를 magenta로 고정해 alpha 0이 backdrop만 남기는지 확인한다. oracle sample `(12,20)`과 `(15,20)`이 `[17,34,51,255]`라 source RGB 누출을 식별한다.
2. **투명 sample이 배경을 덮어씀** — color blend에 `One` 또는 alpha ignore가 들어가면 투명 영역이 바뀐다. 결과 색만이 아니라 이미지 밖과 투명 영역이 모두 exact 배경인지 확인한다.
3. **부분 alpha가 불투명으로 취급됨** — alpha 128 source pixel을 고정하고 sample `[118,37,65,255]`로 비교한다. alpha를 255로 치환하는 잘못된 경로는 불투명 green positive control과 다른 결과를 낸다.
4. **source-over 대신 destination-over/덧셈 사용** — source RGB 및 배경을 서로 다른 색으로 골랐다. 방향이 뒤집히거나 채널이 더해지면 반투명 표본이 exact oracle과 달라진다.
5. **색상 blend factor는 맞지만 alpha factor가 틀림** — backdrop은 불투명하므로 source-over 출력 alpha가 255여야 한다. RGB와 A의 blend state를 별도로 검토하고 app capture의 alpha byte도 비교한다.
6. **straight-alpha를 premultiplied로 오해함** — PNG texel은 `(220,40,80,128)`의 straight RGBA8이다. shader에서 RGB를 미리 alpha 곱하고 `SrcAlpha`도 적용하는 이중 곱을 negative control로 잡는다.
7. **색 공간 때문에 플랫폼 간 결과가 다름** — Chrome sample `[118,37,65]`를 명시적 기준으로 삼는다. Android `Rgba8Unorm`, iOS `Bgra8UnormSrgb` 각 결과를 exact 비교하고 surface 조정이 필요하면 opaque C18.1과 색상 경로도 재실행한다.
8. **색 profile/metadata를 임의로 적용함** — PNG generator는 ancillary color-profile chunk 없이 RGBA8을 만든다. PNG hash와 RGBA source value가 다르면 oracle을 사용하지 않는다.
9. **alpha 0/255 경계만 검사해 128 누락** — 10개 표본 중 반투명 band의 양쪽 경계와 내부 3점이 있다. 세 점 중 일부만 일치하면 필터 또는 좌표 오류로 거부한다.
10. **띠 경계에서 선형 필터링** — x=15/16 및 23/24를 양쪽에서 표본화한다. nearest/clamp natural-size paint에서 인접 source band가 섞이면 실패한다.
11. **natural-size 약속을 DPR 확대와 혼동** — CSS frame은 항상 `(8,12) 24×16`이다. 2.625×와 3× physical surface 배율은 view mapping에만 적용하고 CSS geometry를 확대하지 않는다.
12. **고정 backdrop이 실제 image 아래 있지 않음** — root와 body 모두 `#112233`으로 채우고 이미지 밖 `(2,2)`, `(34,20)`를 확인한다. 투명 sample이 다른 surface clear color를 보는 실행은 fixture가 아니다.
13. **이미지 draw가 box 순서를 깨뜨림** — 한 root background 다음 단일 image를 그린다. C18.1 border/overflow 별도 회귀도 실행해 image pipeline 변경이 기존 paint ordering을 깨뜨리지 않는지 확인한다.
14. **fixture가 정확한 V8 node를 고정하지 않음** — C18.2 전용 JS source는 예상 node ID와 `spinon-fixture:c18-2/...` src를 하나씩 요구하고 추가·누락·중복 노드는 scene attachment 전에 실패시킨다.
15. **요청·문서·render snapshot revision이 어긋남** — source tuple, current `src`, resource revision, render key를 연결한다. 재요청 없이 이전 document image bytes를 재사용하는 상태는 성공 report가 될 수 없다.
16. **async loader 범위를 슬쩍 포함함** — byte 공급은 opt-in fixture adapter에서 동기적으로 고정한다. network, auth, redirect, cancellation, source replacement를 구현하거나 완료로 기록하지 않는다.
17. **기본 빌드에 C18.2 input이 포함됨** — Cargo feature, FFI define, Android/iOS source selector가 모두 opt-in일 때만 route와 PNG payload가 있어야 한다. default artifact scan과 opt-in positive control을 계획에 넣었다.
18. **Metal/PixelCopy 표본만 통과하고 화면에 틀림** — 내부 surface sample과 OS compositor 뒤 전체 화면 sample을 각각 검사하며 view origin, size, scale과 두 anchor가 맞지 않으면 pixel 결과를 무효화한다.
19. **픽셀 결과와 run identity가 분리됨** — source/image hash, document/style/render/surface/frame ID를 캡처 report와 연결한다. 다른 실행 screenshot을 끼워 넣으면 comparator가 거부해야 한다.
20. **부분 검증을 C18 완료로 표시함** — 상태 대장에서 C18.2만 자식 slice로 체크하고 C18, C14, U05, 공개 image/CSS 범위는 그대로 unchecked로 둔다. simulator·contract·artifact 경계 evidence가 없으면 C18.2도 완료가 아니다.

## 계획 수정 후 재확인

검토 뒤 계획에 RGB/A source-over 수식과 두 output channel의 책임, straight-alpha 입력, color-space별 exact 비교 및 opaque C18.1 회귀, fixture node/src 고정, revision 일치, comparator mutation control과 10개 sample/2개 anchor를 명시했다. 20개 관점 모두 계획의 재현 입력, 판정 기준, 경계 또는 evidence에 연결했다. 다음 구현 검토는 별도 관점으로 새로 수행한다.
