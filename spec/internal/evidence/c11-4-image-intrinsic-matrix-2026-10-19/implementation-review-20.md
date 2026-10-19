# C11.4 image intrinsic fixture 구현 검토 20개

검토 범위는 이번 변경의 HTML image scene allowlist, intrinsic resource 연결, layout revision, FFI scene 공급, Android/iOS assertions와 artifact 경계다. 기존 text intrinsic 구현의 이전 검토는 재사용하지 않았다. 각 항목은 별도의 실패 경로 또는 계약 경계를 확인한다.

| # | 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | 비 HTML namespace의 `div`가 같은 local name으로 fixture에 진입 | `prepare`가 root tag만 비교하던 결함을 찾았다. root와 item namespace를 모두 XHTML로 제한하고 `element_identity`가 namespace를 보존하도록 수정했다. SVG namespace root를 직접 전달하는 negative unit test가 `unsupported_c114_image_case`로 닫히는 것을 확인했다. |
| 2 | allowlist 밖 root ID나 알 수 없는 image scene이 제품 fallback으로 통과 | `IMAGE_CASES`에서 root ID를 찾지 못하면 `unsupported_c114_image_fixture_root`로 중단한다. FFI도 고정 scene ID만 받는다. |
| 3 | fixture root 아래의 중첩·복수 image가 하나의 intrinsic input으로 축약 | 전체 subtree의 HTML `img` 수와 root의 직접 자식 수를 확인하고, 단일 direct leaf 이외는 거부한다. |
| 4 | 다른 source URL 또는 무속성 `src`가 예제 PNG와 섞임 | `src`를 고정 URL과 exact 비교한 다음에만 `ImageRequest`를 만든다. 비고정 source 음성 Rust test가 오류 코드를 확인한다. |
| 5 | 테스트 중 외부 네트워크 결과에 따라 oracle이 바뀜 | PNG 바이트는 저장소 fixture를 `include_bytes!`로 공급한다. 고정 URL은 identity용이고 fetch하지 않는다. |
| 6 | 잘못된 PNG나 다른 natural size가 0×0/fallback geometry로 묵살 | decode·resource snapshot 오류를 전파하고 24×16 metadata를 exact 검사한다. failure는 layout 결과를 게시하지 않는다. |
| 7 | 다른 input revision의 natural size가 stale layout/render cache에 재사용 | `ImageResourceSnapshot`을 현재 document snapshot으로 만들고 `CurrentLayoutInputs`에 intrinsic revision을 반영한다. revision 1은 고정 단일 fixture 실행값이다. dynamic replacement/cache invalidation은 범위 밖으로 명시했다. |
| 8 | nested child, sibling 또는 텍스트가 이미지 item으로 잘못 간주 | root에 자식 하나, HTML `img` leaf, 정확한 node ID를 요구한다. Rust snapshot 검사와 9개 앱 scene에서 두 node frame을 대조한다. |
| 9 | image element의 width/min/max/padding/position/placement가 oracle과 다름 | typed computed dimension, edge, position, placement, `box-sizing`, display를 allowlist와 비교한다. 미등록 조합은 거부한다. |
| 10 | C11.4용 `align-content:start`·`justify-content:start` 예외가 다른 profile로 확장 | `reject_grid_alignment`의 새 예외는 `RuntimeBlockFlexGridTrackSizingV1` grid container로 제한된다. scene geometry와 track trace가 기대값인지 앱 양쪽 assertion이 확인한다. |
| 11 | CSS 원문 문자열만 보고 track grammar 또는 개수를 잘못 인식 | runtime validator는 `ComputedCssGridTrackSize` typed 값, 축별 track count, line names, template area, auto-flow와 auto-track을 검사한다. |
| 12 | definite percentage basis가 없는 fit-content 퍼센트를 임의 계산 | matrix의 50% limit은 100px definite grid width에 고정되어 있다. 이 장면만 24px track을 Chrome oracle과 비교하며 indefinite basis 일반 동작은 주장하지 않는다. |
| 13 | span contribution에 gutter가 중복되거나 첫 track에 전부 더해짐 | span-2 장면에서 24px item contribution, 10px·4px·10px 최종 sequence를 Android/iOS trace와 Chrome에 대조한다. |
| 14 | row min-content가 column contribution으로 잘못 기록 | `min-content-row-image`에서 Columns 100px, Rows 16px를 별도 trace로 확인하고 row min-content query가 16px인지 양 앱에서 검사한다. |
| 15 | layout이 partial frame이나 진단을 내보낸 뒤 resource error를 감춤 | prepare/decode 오류는 `RuntimeLayoutFailure`로 전파되며 결과 생성 전에 반환한다. 지원하지 않는 source의 negative test가 이 경로를 확인한다. |
| 16 | generated JavaScript scene 값이 코드 문맥으로 주입 | root/item ID와 CSS parameter는 컴파일된 고정 scene struct 값이고 JS 반환 식 문자열은 Rust debug quoting을 사용한다. 외부 scene ID는 allowlist 조회에만 쓰인다. |
| 17 | 생성된 `Cow` source의 수명이 FFI `eval` 중 끝남 | borrowed 기존 text scene과 owned image scene을 `Cow<'static, str>`로 구분하고, FFI에서 `source.as_ref()`를 동기 `host.eval`에 전달한다. FFI focused tests와 앱 실행을 통과했다. |
| 18 | Android/iOS scene allowlist 또는 expected frame mapping이 서로 다름 | 두 native harness의 9 scene ID, root/item frame, natural-size revision, axis별 final track 및 contribution assertion을 비교 실행했다. 9/9에서 양 플랫폼 값이 Chrome에 일치한다. |
| 19 | screenshot의 red item background를 decoded image texel paint로 오인 | 표면 표본은 고정 track 밖 item background와 root 배경을 읽는다. README와 PR은 screenshot을 box/overflow 증거로 한정하고 image paint를 미검증으로 명시한다. |
| 20 | fixture FFI 심볼·scene 문자열·PNG가 기본 정적 라이브러리에 유출 | 기본 release staticlib와 `c11-grid-metal-surface-pixels-test` positive control을 각각 빌드해 LLVM symbol, string, PNG payload를 검사한다. 결과는 [artifact boundary](./artifact-boundary/README.md)에 기록한다. |

## 판정

20개 관점 중 #1에서 실제 누락을 찾아 namespace 검사를 보강하고 전용 negative test를 추가했다. runtime C11.4 focused tests, FFI 5 tests, package·workspace Clippy, rustfmt, default/positive-control artifact 경계와 최종 9-scene Simulator 비교를 통과했다. 원본 로그와 결과 파일은 이 폴더에 보존한다. simulator 통과는 제품 이미지 paint, 실기기, WPT 또는 C11.4 완료를 뜻하지 않는다.
