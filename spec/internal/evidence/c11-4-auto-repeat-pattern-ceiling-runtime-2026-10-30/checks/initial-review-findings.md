# 초기 검증 실패와 수정

## 잘못된 필터

첫 focused command는 `--exact`에 모듈 경로를 포함했으나 Cargo test 이름과 일치하지 않아 테스트 0개를 선택했다. 이 실행은 테스트 통과로 세지 않는다. 이후 함수 이름 substring filter로 다시 실행했다.

## 첫 실제 테스트 실패

함수 이름 기준 첫 실행은 `InvalidGridTrackList { node: NodeId(1), property: "grid-template-columns" }`로 실패했다. 반복 line-name metadata가 제공될 때 `validate_auto_repeat_axis`는 outer line-name vector의 길이를 `tracks.len() - repeated_track_count + 2`로 요구한다. 4개 source track과 2개 repeated track인 이 입력은 4개 outer group을 필요로 한다.

계획 입력, precomparison 및 test helper를 수정해 해당 축에 네 개의 빈 outer group을 전달했다. 반복 패턴의 실제 세 이름 group은 `GridAutoRepeat.line_names`에만 남겨 중복 name을 만들지 않는다. 수정 후 같은 focused test를 다시 실행해 통과했다.

## 수정 뒤 focused 결과

명령:

```sh
mise exec -- cargo test --locked -p spinon-layout --lib auto_repeat_two_track_pattern_preserves_named_line_at_the_ceiling_on_both_axes
```

결과: 1 test passed, 0 failed, 313 filtered out. 이 결과는 단일 test function의 수정 뒤 rerun이다. broader suite 결과를 대신하지 않는다.
