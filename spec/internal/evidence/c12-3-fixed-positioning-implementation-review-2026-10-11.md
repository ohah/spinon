# C12.3 viewport fixed 구현 변경 실패 관점 검토

**범위:** Rust/Taffy owner 계산, Stylo profile·진단 투영, FFI와 Android/iOS V8·WGPU fixture, Chrome comparator, 계약·상태 문서. 구현 전 계획 검토와 사전 비교는 [별도 문서](./c12-3-fixed-positioning-plan-review-2026-10-11.md)에 둔다.

## 구현 중 발견해 수정한 결함

- `Fixed` 허용을 일반 absolute 허용과 한 조건으로 묶자 legacy Flex profile도 fixed를 성공 처리해 기존 C12.1 거부 경계 테스트가 깨졌다. projection에서 `supports_fixed`를 `RuntimeBlockPositioningV1`에만 허용하도록 분리했다. C12.1 Flex profile 회귀와 C12.3 Block profile 고정 fixture를 각각 다시 실행했다.
- 테스트/로그 baseline이 첫 environment revision을 1로 가정했지만 실제 초기 revision은 0이었다. 첫 eval 결과에서 초기 revision을 읽고 이후 matrix 기대값을 파생하도록 Android·iOS 비교 기준을 수정했다.
- Taffy의 out-of-flow 계산 결과 순서는 HostDocument inventory 순서와 같지 않았다. frame array를 inventory와 zip 비교하던 로직은 같은 수의 다른 node를 비교할 위험이 있었다. comparator가 안정 NodeId로 매핑하고 frame index 중복을 별도로 검사하도록 수정했다.
- C12.3 runtime subset에서 고정 Chrome inventory의 `aspect-ratio` node가 빠져 있었다. `40px × auto`, ratio 2:1 node를 성공 subset과 계획에 추가해 target profile의 adapter allowlist까지 검증했다.
- 재실행 로그의 revision negative control이 첫 `environment_revision=9` 문자열만 바꿔 matrix summary가 아닌 보조 environment report를 수정하고 있었다. 정상 로그는 여전히 통과했으므로 실패 경로를 시험하지 못했다. canonical `resize-outbound` summary를 직접 바꾸고 mutation 자체가 적용되었는지도 검사하도록 테스트를 고쳤다.

## 독립 실패 경로

| # | 검토한 실패 관점 | 확인한 경계와 판정 |
| --- | --- | --- |
| 1 | fixed를 일반 absolute로 덮어써 legacy profile이 뜻하지 않게 확장되는가 | `supports_absolute`와 `supports_fixed`를 분리했다. C12.1 RuntimeFlexLayout 거부와 C12.3 RuntimeBlockPositioning 성공 테스트를 확인한다. |
| 2 | fixed element가 일반 positioned ancestor를 containing block으로 잘못 선택하는가 | `collect_fixed_owners`가 viewport owner를 따로 만든다. relative wrapper의 offset과 무관한 viewport frame을 fixed fixture로 확인한다. |
| 3 | fixed box 아래 absolute child가 viewport에 붙는가 | fixed box는 absolute owner를 만들며 해당 child의 inset은 fixed box 기준이다. `fixed_layout_uses_viewport_and_keeps_absolute_and_fixed_owners_separate`가 양쪽 owner를 비교한다. |
| 4 | fixed descendant가 fixed ancestor를 상속하는가 | fixed descendant owner는 새 fixed ancestor를 만나도 viewport로 유지한다. 같은 Rust fixture에서 부모 offset과 다른 fixed child frame을 확인한다. |
| 5 | fixed synthetic root가 HostDocument 구조나 source order를 바꾸는가 | 계산용 graph만 재연결되고 원본 snapshot은 읽기 전용이다. NodeId·owner uniqueness 및 runtime frame NodeId mapping 검사를 확인한다. |
| 6 | display:none subtree 아래 fixed child가 viewport에 되살아나는가 | hidden 상태를 owner collection과 paint 제출에 전달한다. 28개 source frame 중 hidden node가 0×0이고 제출 box 수가 source element 수보다 작은지 comparator가 검사한다. |
| 7 | fixed layout root를 암묵적으로 viewport child로 처리하는가 | 계획한 root-fixed 제외 경계에서 오류를 유지하며 `fixed_position_tests`의 root negative test가 전체 실패를 확인한다. |
| 8 | 모든 inset이 auto라 static-position solver가 필요한 입력을 임의 배치하는가 | 성공 profile은 definite inset 축만 허용한다. 고정 inventory와 computed-style validation이 auto static-position case를 positive subset으로 올리지 않는다. |
| 9 | 두 축 stretch, auto size와 margin의 과결정 순서를 단순 위치로 대체하는가 | pinned Chrome fixed inventory와 parent-closed runtime subset에서 지원 가능한 조합만 비교한다. 지원하지 않는 intrinsic/indefinite 조합은 오류다. |
| 10 | top/bottom percentage를 viewport width basis로 계산하는가 | 축별 percentage basis와 Chrome frame 비교를 확인한다. 360×800 및 390×844에서 viewport percentage node가 각 높이 기준을 따른다. |
| 11 | DPR 변경을 CSS px geometry scaling으로 오인하는가 | 네 DPR의 각 viewport frame을 독립 비교한다. 같은 CSS viewport의 DPR-only state는 CSS frame이 같고 revision만 환경 입력에 반응한다. |
| 12 | resize no-op이 revision을 올리거나 resize 왕복에서 이전 값이 남는가 | comparator가 outbound/no-op/return state와 revision을 순서대로 검증한다. 마지막 return frame은 시작 viewport geometry에 복귀한다. |
| 13 | 오래된 environment revision의 frame을 마지막 제출로 수락하는가 | runtime comparator는 matrix별 revision과 마지막 `presented` revision을 검사한다. 잘못된 revision을 넣은 negative-control test는 실패해야 한다. 비동기 임의 지연 주입은 이 fixture 범위 밖이다. |
| 14 | transform·perspective·filter·contain 효과가 있는 ancestor를 viewport owner로 취급하는가 | computed effect typed flag와 Stylo diagnostic source preflight가 projection에서 전체 실패한다. 계산 가능한 효과와 unknown diagnostic test를 구분한다. |
| 15 | inline style 경로만 검사하고 author stylesheet effect를 놓치는가 | 동일한 fixed-CB effect 계약을 author stylesheet와 inline source 수집 경계에서 검사하고, projection에서도 snapshot effect를 재검증한다. |
| 16 | Stylo UnknownProperty·미지원 진단이 버려져 effect 선언이 무시되는가 | `RuntimeBlockPositioningV1`에서 cascade diagnostics를 거부한다. `stylo_unknown_fixed_effect_diagnostics_fail_before_layout`으로 계산 전 오류를 확인한다. |
| 17 | aspect-ratio computed style이 runtime Block adapter allowlist에서 빠지는가 | 40×20 CSS px positive case를 넣고 cascade→projection→Taffy 결과를 비교한다. 이전 `BlockFormatting` allowlist 재사용은 `BlockPositioning` allowlist로 고쳤다. |
| 18 | aspect-ratio를 min/max나 intrinsic sizing 일반 지원으로 확대 해석하는가 | 별도 조합은 C07.3 fail-closed 정책을 유지한다. 이번 positive test는 definite width, auto height 한 조합으로 제한한다. |
| 19 | 계산된 frame 출력 순서를 fixture order로 오인해 다른 node를 비교하거나 revision negative control이 잘못된 문자열을 바꿔 no-op이 되는가 | comparator가 `nodeId`로 reference entry를 찾고 frame index uniqueness·count를 별도로 확인한다. 한 frame 누락·revision 변경·이전 실패 marker negative-control이 있고 revision mutation은 canonical matrix summary에서 실제 적용을 확인한다. |
| 20 | Android/iOS 초기 로그만 통과하고 resize 이후 표시가 실패하는가 | 양 플랫폼 comparator가 13개 상태·364 frame을 검증하고 마지막 WGPU `presented`가 revision 11·26 boxes·360×800 CSS px인지 확인한다. 로그와 캡처는 [실행 근거](./c12-3-fixed-positioning/README.md)에 둔다. |

## 검증 경계

전체 Rust workspace·Clippy·rustfmt, CSS reference suite와 네이티브 앱 빌드 결과를 이 기록과 PR 검증 결과에 연결한다. Android는 SM-S731N 실기기, iOS는 iPhone 17 Pro Simulator다. WPT 전체, iOS 실기기, 전체 모바일 inventory, scroll·stacking·paint-order·성능은 검증 범위가 아니다.
