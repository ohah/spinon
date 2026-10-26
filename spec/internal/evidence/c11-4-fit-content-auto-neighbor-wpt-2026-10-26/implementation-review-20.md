# C11.4 fit-content 옆 auto row 구현/runtime 실패 관점 검토 · 20개

- 최종 검토 대상 commit: `9c3ee690d72325c81d1b452e0f007d84b5fe40f9`
- 범위: 신규 순수 Rust oracle 테스트, `#[cfg(test)]` 등록, 계획 및 실행 근거
- 최종 판정: **20 PASS / 0 PARTIAL / 0 FAIL**
- 검토 방식: 독립 코드·규범·근거 검토. 구현 코드는 수정하지 않았다.

이 20-view 기록 시점의 evidence manifest에는 19개 항목이 있었다. 이후 PR diff review와 본문 이미지 확인 기록을 추가해 현재 manifest는 21개 항목이다. 현재 21개 항목은 모두 SHA 검증된다.

초기 20-view에서 동작 관련 18개는 통과했고, 두 근거 항목은 PARTIAL이었다. 구현 소스 해시와 등록 diff를 고정하고, WPT root와 Spinon unit-test root의 폭 필드를 분리했으며, `cargo fmt` 무출력 결과를 manifest에 포함했다. 아래 최종 판정은 이 수정 뒤의 관련 근거 재검토를 반영한다.

| # | 실패 관점 | 최종 확인 |
|---:|---|---|
| 1 | 선택한 WPT assertion이 다른 fit-content 조건일 수 있음 | `fit-content-track-sizing.html`의 `fit-content(40px) auto` 단일 assertion과 `[10px, 290px]` 값을 대조했다. PASS |
| 2 | WPT revision/source가 바뀌었을 수 있음 | revision `c999c58338ee1d223df5ad62e48be1202ced0d37`, 파일 SHA 및 원본 130–132행을 고정했다. PASS |
| 3 | 브라우저 사전 관찰을 WPT PASS로 오인할 수 있음 | `file://` 환경에서 testharness를 실행하지 못한 제한이 README와 계획에 남아 있다. PASS |
| 4 | 10px 최소 기여가 규범 근거 없이 설정됐을 수 있음 | visible overflow, auto minimum track, 단일 span과 10px nested content를 CSS Grid §6.6에 연결했다. PASS |
| 5 | fit-content row가 auto row처럼 stretch될 수 있음 | 첫 track의 max function은 max-content이고 40px argument에 의해 growth limit이 10px로 제한됨을 설명한다. PASS |
| 6 | 남은 free space 산술이 잘못됐을 수 있음 | `300 - 10 - 10 = 280px`; zero gap/padding/border에서 두 번째 row가 290px이 된다. PASS |
| 7 | 테스트 폭을 WPT 폭으로 오인할 수 있음 | `expected.json`은 `spinon_unit_test_root_width_px`를 사용하고, 계획은 WPT containing-block 폭과 구분한다. PASS |
| 8 | explicit row를 implicit row라고 검사할 수 있음 | 입력의 explicit rows는 비어 있고 `auto_rows`는 실제 `[FitContent(40px), Auto]`다. PASS |
| 9 | track order나 item placement가 assertion과 다를 수 있음 | row auto-flow, 두 direct item 순서, 두 auto-row pattern의 순서가 검사된다. PASS |
| 10 | item 크기와 content contribution이 섞일 수 있음 | auto-height wrapper와 각 10px nested block의 frames를 별도로 검사한다. PASS |
| 11 | 다른 grid의 trace를 읽을 수 있음 | 테스트는 유일한 trace가 root node에 귀속됨을 검사한다. PASS |
| 12 | column trace를 row 결과로 해석할 수 있음 | root trace에 row pass가 정확히 하나인지 확인한다. PASS |
| 13 | 단계가 누락되거나 순서가 바뀔 수 있음 | 네 track-sizing stage의 존재·단일성·순서를 검사한다. PASS |
| 14 | intrinsic 기여 전/후 base를 혼동할 수 있음 | `IntrinsicSizingComplete`에서 두 row bases `[10,10]`을 검사한다. PASS |
| 15 | maximize/flexible 단계가 예상 밖으로 track을 바꿀 수 있음 | 두 단계 각각의 bases가 `[10,10]`이며 stretch보다 앞인지 검사한다. PASS |
| 16 | `align-content: normal`에서 stretch가 적용되지 않을 수 있음 | 단 한 번의 `StretchAutoTracks { applied: true }`를 요구한다. PASS |
| 17 | 남은 공간이 fit-content row에 배정될 수 있음 | stretch 뒤 bases `[10,290]`으로 두 번째 auto row만 커졌음을 확인한다. PASS |
| 18 | gutter sentinel을 CSS track으로 셀 수 있음 | 두 track과 세 zero-size gutter를 구분하며 양 끝 gutter만 collapsed인지 검사한다. PASS |
| 19 | trace는 맞지만 frame이 다를 수 있음 | root, 두 wrapper, 두 nested content의 다섯 frame을 `1e-4 CSS px` 허용치로 검사한다. PASS |
| 20 | test-only 결과를 제품·플랫폼 지원으로 넓혀 쓸 수 있음 | production 코드와 API는 그대로다. stdout, fmt 결과, 19개 manifest entry와 source hashes를 확인했고 WPT/runtime/platform 미검증 경계가 명시돼 있다. PASS |

구현 관측은 [implementation-observations.md](./implementation-observations.md), 독립 기대값은 [expected.json](./expected.json), 소스 provenance는 [implementation-source.json](./implementation-source.json)에 있다. `spec/STATUS.md`는 바뀌지 않았으며 C11.4 미완료와 공식 `72/213`이 유지된다.
