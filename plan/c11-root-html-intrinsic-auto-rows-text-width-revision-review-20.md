# C11.4 text min-content row 계획의 width 정정 검토 20개

이 검토는 계획 PR #1189 병합 후 구현 준비 중 발견한 text item width 표현의 계획·fixture 불일치를 다룬다. 이전 계획 문장은 `width:auto`였지만 공통 `.item` 규칙이 `width:40px`을 상속해 fixture와 계약이 달랐다. 수정은 `#wrapped-text`의 authored width를 `auto`로 명시하고, Chrome 156에서 Grid stretch에 따른 resolved width 40px을 다시 관찰한 것이다.

1. **선언과 사용값 분리:** `width:auto` declaration과 40px resolved width를 각기 다른 필드로 남긴다.
2. **Stretch 근거:** resolved 40px이 intrinsic track 추정치가 아니라 explicit column의 Grid stretch에서 온다.
3. **Alignment 기본값:** `justify-items:normal` 동작이 computed style과 fixture에서 바뀌지 않는다.
4. **인라인 측정 기준:** definite 40px column이 text wrapping의 사용 inline size를 계속 고정한다.
5. **명시 track 보존:** width 정정이 explicit 40px column을 auto 또는 intrinsic track으로 바꾸지 않는다.
6. **줄 수 재관찰:** 수정된 fixture에서 Chrome 156이 여전히 두 line rect를 보고한다.
7. **행 기여 재관찰:** 두 line × 10px line-height의 20px row 기대가 다시 관측된다.
8. **Sibling 대조:** 마지막 34px item의 위치가 수정 전과 같은 y=30px이다.
9. **Overflow 대조:** visible content bottom y=64px와 container 밖 24px이 유지된다.
10. **고정 track control:** 임시 `grid-auto-rows:10px` 결과는 여전히 `[10,10,10]px다.
11. **Box sizing:** auto width resolve에 border/padding/margin이 더해지지 않는다.
12. **Minimum 제약:** `min-width:auto`와 `min-height:auto`가 그대로 유지되어 자동 최소값을 감추지 않는다.
13. **Maximum 제약:** `max-width:none`과 `max-height:none` 조건을 고정한다.
14. **Ratio 배제:** item에 aspect ratio가 없어 width transfer가 row contribution을 바꾸지 않는다.
15. **Writing mode:** horizontal-tb와 LTR을 유지해 inline/block 축을 바꾸지 않는다.
16. **Source 정체성:** text node source와 ASCII input은 width 명세 수정 전후에 같다.
17. **Resource 경계:** auto width 지원은 이 단일 owner/item grammar에 한정하고 일반 Root HTML 항목에 개방하지 않는다.
18. **Unsupported width 대조:** text item의 authored fixed width는 이 slice에서 명시적으로 거부한다.
19. **Hash 갱신:** 수정된 fixture SHA-256과 observation/expected/manifest가 같은 source를 가리킨다.
20. **증거 수명:** Chrome 캡처는 Agent Browser에서 다시 만들고 Browser process를 닫는다.
