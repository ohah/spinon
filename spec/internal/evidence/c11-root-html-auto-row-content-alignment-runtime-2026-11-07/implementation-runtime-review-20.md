# C11.4 auto-row content alignment implementation/runtime review

계획 PR 검토와 다른 실패 경로 20개를 코드·테스트·시뮬레이터 근거에 대조했다.

1. **Computed allowlist 누락:** `normal|stretch`와 계획한 8개 새 keyword만 허용하고 별도 helper 테스트에서 목록을 고정한다. 통과.
2. **초기값 부재:** computed property가 없으면 helper가 거부한다. 통과.
3. **Baseline·safety modifier 누출:** `baseline`, `first/last baseline`, `safe center`, `unsafe end`와 unknown은 거부한다. 통과.
4. **Grid alias 의미 불일치:** `flex-start`/`start`, `flex-end`/`end`가 각각 같은 layout geometry를 낸다. 통과.
5. **Stretch 의미 확장:** auto-track stretch stage는 `normal|stretch`에서만 `applied=true`다. 나머지 8개는 false다. 통과.
6. **Base track 손상:** positional/distributed cases는 `[10,10,10]px` row bases를 유지한다. 통과.
7. **정렬 위치 오차:** layout matrix가 세 item 모두의 x/y/width/height를 독립 기대 frame과 비교한다. 통과; quantization 허용치는 1/32 CSS px.
8. **세 번째 track 누락·추가:** trace는 explicit 1개와 positive implicit 2개 track을 확인한다. 통과.
9. **잘못된 sizing pass의 관찰:** row pass에서 `StretchAutoTracks` 단계 하나를 찾아 적용 여부와 bases를 검사한다. 통과.
10. **Root HTML validation gate 불일치:** `grid.rs`와 root one-axis placement gate 양쪽에 같은 enum allowlist를 추가했다. 집중 layout matrix 통과.
11. **Projection/runtime gate 불일치:** style candidate, auto-row item validator 및 alignment rejection scope가 동일한 computed helper를 사용한다. RuntimeSession 5/5 통과.
12. **기존 single-text profile 확장:** 두-text-row shape 이외에서는 `start`를 계속 거부한다. 회귀 3/3 통과.
13. **세 item만 있으면 임의 text 허용:** 추가 gate는 새 positional/distributed 값에 세 visible item을 요구하며, 기존 source/item grammar가 fixed empty item과 direct `A`,`B` rows를 확인한다. rejection matrix 통과.
14. **Shorthand 첫 축 누출:** `place-content:start center`에서 `align-content`는 허용돼도 `justify-content:center`로 인해 전체 layout을 거부한다. 통과.
15. **Shorthand 두 번째 축 누출:** `place-content:center start`에서 `justify-content:start`는 거부된다. 통과.
16. **Partial publication:** unsupported baseline/safety/shorthand 요청은 completed layout을 발행하지 않는다. RuntimeSession rejection tests 통과.
17. **text painter 위치 불일치:** 두 glyph origin이 기대 text-row y와 일치하고 같은 completed style revision을 가리킨다. 통과.
18. **거부 뒤 session 영구 실패:** unsupported dark-mode request 뒤 유효한 요청이 같은 RuntimeSession에서 회복한다. 통과.
19. **Android/iOS fixture 차이:** 각 scene의 fixture SHA는 Android와 iOS 로그에서 일치한다. `start` 및 `space-between` 두 scene 모두 양 플랫폼 draw/present.
20. **검증용 코드가 기본 artifact로 유출:** unit-test positive control에는 새 test marker가 있고 default release runtime/FFI artifact에서는 없다. 시뮬레이터 실행 뒤 설치본·기존 bootstrap asset도 원래 SHA로 복원됐다.

검토 중 기존 `RootHtmlAutoRowStretchV1` alignment enum을 읽는 두 번째 layout gate가 별도로 남아 있음을 발견해 allowlist를 같이 확장했다. Shorthand는 computed `justify-content`가 새 축 값으로 전달되는 경로를 검출하도록 두 방향 거부 control을 추가했다. 두 보완 뒤 영향 테스트와 scoped Clippy/rustfmt를 다시 통과했다. 남은 한계는 [runtime README](README.md)의 scope와 non-green broad-suite 기록을 따른다.
