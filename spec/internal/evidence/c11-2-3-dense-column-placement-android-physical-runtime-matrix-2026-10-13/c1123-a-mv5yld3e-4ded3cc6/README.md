# C11.2.3 모바일 runtime geometry 행렬

- run ID: `c1123-a-mv5yld3e-4ded3cc6`
- 대상: R5KYB06SMPE · Android 16/API 36 · SM-S731N
- 앱 SHA-256: `75106de0c921b3916027e5337e6461a037112d974300edbae8dfee39c550346f`
- 결과: 성공 19/19 · 기대 거부 6/6 · 전체 scene 25개
- 전체 node: 107개
- geometry 최대 절대 오차: 0 CSS px (허용 0.5 CSS px)
- 장면마다 새 앱 process와 HostDocument를 사용했다.
- 거부 장면은 오류 code·NodeId·CSS property·frame 부재·양수 draw 부재를 확인했다.
- 성공 장면은 DOM 순서·geometry·paint order·양수 WGPU draw를 확인했다.
- 표면 pixel: 19개 성공 scene, 99개 표본, 최대 RGBA 채널 오차 0.
- [세부 결과](./comparison.json)와 각 scene의 process-filtered runtime log를 보존했다.
- 이 행렬은 고정 Chromium fixture 결과이며 WPT 적합성 또는 전체 CSS Grid 지원을 뜻하지 않는다.
