# C13.1 복합 stacking descendant 구현 실패 관점 20개

코드·고정 oracle·플랫폼 로그를 서로 다른 실패 경계로 대조했다. 이전 계획 검토는 재사용하지 않았다.

1. **고정 HTML 바뀜** — include된 fixture hash `6b691d…0791e`를 코드 상수와 검사해 임의 scene 변경을 거부한다.
2. **Chrome 버전 오염** — sample/observation의 고정 Chrome 155 버전과 metadata/hash 조건을 대조한다.
3. **viewport/DPR 불일치** — oracle `540×128`, DPR 1, scroll 0을 검사하고 앱 surface viewport도 같은 값인지 확인한다.
4. **screenshot 기준 드리프트** — PNG SHA가 pinned 상수와 같아야 한다; 시각 비교를 동작 판정 대신 쓰지 않는다.
5. **observation/sample 파일 교체** — 두 JSON의 hash를 고정해 둘 중 하나만 업데이트하는 변경을 실패시킨다.
6. **frame 누락·추가** — 기대 node/frame 13개와 정확히 대조하고 12/14 frame 입력을 거부한다.
7. **frame 순서 바뀜** — index별 기대 ID와 geometry를 함께 비교해 같은 box 집합의 순열도 거부한다.
8. **geometry drift** — 좌표·크기별 최대 `0.5 CSS px` 허용을 적용하고 NaN·무한 값은 수치 검증에서 실패한다.
9. **node ID 중복** — 13개 frame의 node ID가 유일해야 한다.
10. **hit 표본 누락·추가** — 정확히 18개의 JSON row만 허용한다.
11. **case/name 순서 바뀜** — `visible/clip/hidden`과 각 6개 point name을 행 순서 그대로 검사한다; 이 검사는 초기 negative test에서 빠져 있던 필드였고 이번에 보강됐다.
12. **좌표 중복·pixel center 아님** — 각 표본의 `.5` 중심, viewport bounds, 고유 정수 pixel cell을 검사한다.
13. **잘못된 hit target** — Chrome `hitId`에서 고정 node index를 구하고 runtime `NodeId` 결과와 exact match한다.
14. **hit 색으로 오판** — color와 element identity를 독립적으로 검사하므로 overlay 색만 맞춘 child hit는 통과하지 못한다.
15. **RGBA malformed** — 네 채널 shape·정수 범위와 pinned expected color를 대조하고 alpha 포함 exact RGBA를 비교한다.
16. **부분 성공 report 노출** — 검증 중 오류가 나면 JSON report 생성 전에 `Err`를 반환하며 partial sample list를 성공 report로 내보내지 않는다.
17. **FFI 잘못된 포인터/길이** — null host/output, 0 capacity, output 부족 길이 negative tests와 safety 계약을 대조한다.
18. **fixture 기본 제품 빌드 유입** — default archive에서 두 export·payload 0, opt-in build에서 symbol 2개와 source marker가 있어 feature 경계를 확인한다.
19. **플랫폼 route 혼용·stale capture** — C13 route 충돌은 host 생성 전에 실패한다. 별도 앱 실행에서 document revision 65→66 변이를 주입해 Android PixelCopy와 iOS Metal readback 전에 거부되는 것을 확인한다.
20. **surface/run identity와 과장** — PixelCopy는 Android surface generation·presentation sequence를 callback 전후에 고정하고 iOS capture manifest는 run/PID/generation/format/sample hash를 보존한다. 두 결과 모두 fixture 전용이며 compositor/scanout·실기기·WPT 증거로 확대하지 않는다.

**결과:** 위 20개 관점에서 차단 결함 1건(case/name validator 누락)을 고치고 focused C13.1 14/14, workspace tests, 두 Clippy, 양 플랫폼 fixture build/runtime를 다시 확인했다. 소스·로그·범위는 [실행 근거](README.md)에 있다.
