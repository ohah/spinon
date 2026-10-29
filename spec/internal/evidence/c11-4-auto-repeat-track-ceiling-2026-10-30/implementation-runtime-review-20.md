# Auto-repeat track ceiling implementation review · 20 failure angles

검토 대상: `grid_track_sizing_tests/auto_repeat_track_ceiling.rs`와 `grid_track_sizing_tests.rs` 모듈 연결. 고정 Taffy 0.15.0 입력 및 최종 검증 로그를 대조했다.

| # | 독립 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 9,999px case가 off-by-one을 내는가 | Columns/Rows의 `used_* .explicit_track_count`와 Initialize non-gutter 수가 모두 9,999다. |
| 2 | 10,000px case가 상한 전에 멈추는가 | 두 축 모두 두 관측값이 정확히 10,000이다. |
| 3 | 10,001px basis에서 Taffy의 clamp가 누락되는가 | 두 축 모두 layout이 성공하고 10,000으로 clamp된다. |
| 4 | Columns가 고정 inline basis를 받지 못하는가 | helper가 root width와 viewport width를 요청한 basis에 고정한다. |
| 5 | Rows가 고정 block basis를 받지 못하는가 | helper가 root height와 viewport height를 요청한 basis에 고정한다. |
| 6 | Gap이 반복 횟수에 섞이는가 | 모든 입력이 `LayoutGap::default()`의 zero gap이다. |
| 7 | Auto-fill 빈 track이 잘못 collapse되는가 | 6개의 축·basis 입력 모두 non-gutter track의 `is_collapsed=false`다. |
| 8 | Auto-fit 빈 track이 collapse되지 않는가 | 6개의 축·basis 입력 모두 non-gutter track의 `is_collapsed=true`다. |
| 9 | auto-fit 결과가 item placement에 의존하는가 | root children과 grid item map이 비어 있는 helper 입력에서 검사한다. |
| 10 | trace budget 실패의 partial output을 성공으로 취급하는가 | 계산 성공과 trace 존재를 요구하며, 각 케이스에서 총 trace state가 100,000 미만인지 검사한다. |
| 11 | 축의 이전 pass를 최종 결과처럼 읽는가 | target axis의 마지막 pass에서 Initialize stage를 고르고 그 pass 안에 정확히 하나인지 확인한다. |
| 12 | wrong-axis trace를 count하는가 | trace pass의 axis와 requested axis가 일치하는 마지막 pass만 선택한다. |
| 13 | gutter를 CSS track으로 count하는가 | stage 목록에서 `is_gutter` 상태를 제외하고 resolved explicit count와 비교한다. |
| 14 | trace와 resolved output이 서로 다른 결과를 내는가 | `used_* .explicit_track_count`와 Initialize stage의 non-gutter 수를 독립 비교한다. |
| 15 | 두 축을 동시에 maximum으로 만들며 budget을 소진하는가 | 선택 축만 10,000px이고 orthogonal 축은 고정 1px이다. |
| 16 | prefix neighbor가 상한 슬롯 계산에서 사라지는가 | 두 축 모두 첫 track이 2px로 남는다. |
| 17 | cap 이하에서 suffix가 너무 일찍 잘리는가 | 10,003px fitting control은 2px prefix, 9,998 × 1px repeat, 3px suffix로 10,000 tracks다. |
| 18 | cap 초과에서 suffix가 유지되거나 prefix까지 잘리는가 | 10,004px control은 prefix 뒤 9,999 × 1px repeat로 채우고 3px suffix를 제거한다. |
| 19 | production surface 또는 fixture ABI에 변경이 들어가는가 | diff는 unit-test 모듈 연결과 test 전용 파일이며 runtime/FFI source는 수정하지 않는다. |
| 20 | 이 검사를 CSS/WPT·모바일 runtime 지원으로 확대 해석하는가 | 검토 범위는 고정 layout input test뿐이며 공식 C11.4/C11 상태는 그대로 열린다. |

**판정:** 20개 서로 다른 실패 관점에서 코드와 실행 결과를 대조했다. 첫 시도에서 빠진 `LayoutEngine` trait import와 auto-fit의 collapsed base size를 1px로 가정한 assertion을 수정했다. 마지막 focused run은 2/2, 전체 `spinon-layout`은 312/312 통과했다.
