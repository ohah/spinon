# Row-span trace 회귀 발견과 수정

최초 최종 회귀 실행에서 기존 `parsed_root_html_later_fit_content_positions_match_all_1920_chrome_scenes` 테스트가 scene 292, `position 2 span2-forward 20px stretch`에서 실패했다. RuntimeSession은 `C11.4 repeated text row-span track trace` 오류로 layout을 fail-closed했다.

원인은 새 9-item auto-placement 정규화가 1/64px row-size distribution helper를 공유한 뒤, 그 helper가 `MaximizeTracks` 및 `StretchAutoTracks` 단계를 요구하게 된 점이었다. 기존 3/4-item row-span profile은 이전 Taffy trace 단계 모양을 사용하므로 공통 helper가 기존 지원 장면을 거부했다.

보정 산술은 공통으로 유지하고 trace extraction을 분리했다. Row-span은 기존 `IntrinsicSizingComplete` 및 마지막 stage 선택을 유지한다. 신규 repeated-text auto-placement는 unique final row trace, `MaximizeTracks`, `StretchAutoTracks`, used-row serialization 및 auto-only growth 검증을 계속 요구한다.

수정 뒤 같은 Chrome 157 RuntimeSession matrix를 다시 실행해 1,920/1,920 scene을 통과했다. 실행 로그는 [`rust-verification/runtime-span-comparison.log`](./rust-verification/runtime-span-comparison.log)에 있다.
