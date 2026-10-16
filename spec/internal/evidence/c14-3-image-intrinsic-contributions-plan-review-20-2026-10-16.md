# C14.3 계획 적대 검토 20개 관점

대상은 [`C14.3 계획`](../../../plan/c14-3-image-intrinsic-contributions.md), 13-case Chrome oracle fixture와 reference다. 구현 전 계획 검토이며 구현/PR 검토를 대신하지 않는다.

## 검토 결과

수정 후 20개 관점을 다시 확인했다. 미해결 P1/P2는 없다. 초기 검토에서 발견한 네 가지 혼동(자동 높이 probe의 Grid-row 결합, border 밖 overflow의 잘못된 거리, oracle screenshot 텍스트가 runtime 레이아웃에 끼어들 가능성, 범위 밖 변형 행렬)은 계획과 fixture에서 제거하거나 정확히 제한했다. 브라우저 버전도 reduced UA와 high-entropy UA를 구분해 기록했다.

## 관점별 공격과 결과

1. **기능 범위 혼합** — 텍스트, 버튼, 자원 loader, image paint까지 C14.3 완료로 넓혀 보려 했다. 이 slice는 HTML `img` sizing keyword와 definite/cyclic percentage만 다루며 C14/C14.1/C14.2는 미완료로 둔다.
2. **기존 구현 의존성** — 계획이 새 layout input을 발명하거나 C14.1/C14.2 snapshot을 우회하는지 확인했다. `LayoutIntrinsicSizeSnapshot`과 기존 source/revision gate를 재사용하도록 고정했다.
3. **완료 표기 오염** — 이 계획/13개 oracle만으로 상위 C14 체크가 켜지는지 확인했다. 상태 대장과 계획에 상위·C14.1·C14.2 미완료가 명시되어 있다.
4. **키워드/함수 파싱 혼동** — `fit-content`와 `fit-content(<length-percentage>)`가 같은 성공 경로로 들어갈 수 있다. keyword만 대상이고 function은 node를 포함한 `LayoutError::UnsupportedIntrinsicSizing`으로 거부한다.
5. **oracle 브라우저 오기** — UA 문자열만 보고 full Chrome build를 단정할 수 있다. reduced UA, high-entropy full version `155.0.8059.85`, macOS `26.5.1`을 별도 필드로 보존했다.
6. **oracle input 변동** — 다른 PNG, 자연 크기, 로딩 실패가 결과를 바꾸는지 확인했다. 자연 크기 `24×16`, SHA-256, `loaded=true` 13/13을 고정했다.
7. **정밀도 착시** — `getComputedStyle()`의 `66.6562px` 문자열을 exact geometry로 착각할 수 있다. expected 값은 unrounded `getBoundingClientRect()` CSS px `66.65625`이며 허용 오차 `0.05 CSS px`다.
8. **content/border box 혼동** — percentage basis에 210×130 border box를 넣어 width/height를 과대 계산할 수 있다. 기대행렬은 200×120 content box를 기준으로 한다.
9. **min-content 잘못 적용** — 자연 이미지가 0이거나 12px로 축소될 수 있다. Chrome 기준은 `24×16`; natural PNG input과 width keyword case를 함께 보존했다.
10. **max-content 범위 과장** — text max-content나 max-content track sizing을 끌어들일 수 있다. `img` keyword 한 case만 검증하며 텍스트/track contribution은 미포함이다.
11. **fit-content 하한/상한 뒤집힘** — 12px available width에서 natural 24px min-content floor가 무시될 수 있다. 기본 case는 `24×16`, 별도 `max-width:12px` control은 ratio 유지 `12×8`이다.
12. **border overflow 과장** — 처음에는 content-area 초과 12px을 border 바깥 초과로 잘못 표현할 여지가 있었다. containing block의 border-box는 22px, image 시작 offset은 5px이므로 실제 오른쪽 border 밖은 7px로 고쳤다.
13. **불명확한 percentage height** — auto-height outer Grid row probe가 indefinite percentage 일반 규칙을 입증한다고 오해될 수 있었다. `75.75px` 관찰을 만든 case를 제거하고 모든 indefinite percentage axis를 비지원으로 두었다.
14. **cyclic basis 혼합** — cyclic percentage를 최종 definite basis에서 다시 계산해 natural `24px`을 반환할 수 있다. min-content containing block의 basis와 image가 둘 다 `0×0`인 별도 control을 둔다.
15. **Flex 자동 최소 크기·줄바꿈** — wrapping, grow/shrink 또는 stretch가 sizing keyword 측정에 개입할 수 있다. 한 item, `flex:0 0 auto`, row/nowrap, `align-items:flex-start`, definite 200×120 content box를 고정했다.
16. **Grid intrinsic track 우발 검사** — auto track, spanning, auto-repeat 규칙을 C14.3에서 동시에 검증하려 할 수 있다. 200×120 fixed tracks 안의 단일 item만 대상으로 하고 intrinsic track sizing은 명시 제외했다.
17. **문자/폰트 침투** — screenshot label이 Android/iOS에서 text layout을 발생시켜 C14.4 결과를 섞을 수 있다. 브라우저 oracle label은 주석이고 opt-in runtime fixture는 text node 없이 case ID를 metadata로 보고한다.
18. **layout과 paint 증거 혼동** — browser screenshot 또는 simulator report가 GPU image clipping을 증명한다고 말할 수 있다. simulator는 frame과 7px overflow만 검사하며 native image pixel paint/clip은 후속 범위라고 분리했다.
19. **실패 시 자동 대체** — 지원하지 않는 function 또는 percentage를 `auto`/0으로 조용히 바꿀 수 있다. `UnsupportedIntrinsicSizing` error와 case/property/node attribution을 기준에 넣었다.
20. **플랫폼/산출물 과장** — AVD/Simulator 결과를 실기기 또는 기본 artifact 지원으로 확대할 수 있다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 opt-in route만 검증하고 default/opt-in artifact 경계를 확인하도록 했다.

계획 검토는 구현 전 oracle·fixture·통과 기준을 확정한다. 이후 구현 실패 관점 20개와 PR diff 실패 관점 20개를 새로 수행한다.
