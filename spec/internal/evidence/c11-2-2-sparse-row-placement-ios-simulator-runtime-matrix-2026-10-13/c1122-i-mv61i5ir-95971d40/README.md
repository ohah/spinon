# C11.2.2 모바일 runtime geometry 행렬

- run ID: `c1122-i-mv61i5ir-95971d40`
- 대상: iPhone 17 Pro · com.apple.CoreSimulator.SimRuntime.iOS-26-2 · ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D
- 앱 SHA-256: `39dc89c473868062c4be85a576d22b3af2be442a8b01f171f7f0301909e2354d`
- 기기 정보: [device.json](./device.json)
- Chrome 154 기준 SHA-256: `aa56820f4b80b87ff314547398ff4910e4eec93db65be85c73599c4455db32b8`
- 결과: 성공 11/11 · 기대 거부 5/5
- geometry 최대 절대 오차: 0 CSS px (허용 0.5 CSS px)
- 장면마다 새 앱 process와 HostDocument를 사용했다.
- 거부 장면은 오류 code·NodeId·CSS property, frame 부재, 양수 draw 부재를 확인했다.
- 성공 장면은 source-order NodeId·geometry·paint order·양수 WGPU draw를 확인했다.
- 화면 PNG는 실행 상태 보조 자료다. 표면 pixel 비교: matched (11/11 성공 장면).
- [세부 결과](./comparison.json): 16개 장면의 결과·오류 진단과 51개 표면 표본 RGBA를 포함한다. 장면별 화면 PNG를 보존했다. 빌드·설치 로그도 함께 둔다.
- 이 행렬은 고정 fixture 결과이며 WPT 적합성 또는 전체 CSS Grid 지원을 뜻하지 않는다.
- Pixel matrix fixture SHA-256: `3ff07215493c0e3e13dec4ae399a9e9348aa1871269075a74a9b471a2528b7f6`; 최대 RGBA 채널 오차: 0.
- 표면 표본은 OS compositor 최종 출력이 아닌 renderer surface 결과다.

대표 장면: [definite-first-mixed 화면](./definite-first-mixed-ios-simulator.png).

보존한 console log는 판독성과 저장소 공백 검사를 위해 줄 끝 공백만 제거했다.
