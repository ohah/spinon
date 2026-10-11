# C12.4 Android AVD surface 픽셀 계획 검토

검토 대상: [Android AVD 실제 surface 픽셀 비교 계획](../../../plan/c12-4-android-surface-pixels.md)

## 실패 관점 검토

1. **오프스크린 결과를 실제 surface로 오인할 위험:** Android `SurfaceView`를 직접 받는 PixelCopy만 인정하고 headless texture readback을 배제했다.
2. **draw 제출 직후 캡처하는 경합:** 다음 UI frame 경계 뒤 PixelCopy를 요청하도록 계획에 고정했다.
3. **이전 surface buffer 캡처:** 매 scene 새 프로세스·새 host를 쓰고 generation/size를 검증한다.
4. **이전 PNG 재사용:** 실행 ID를 캡처 파일명에 넣고 case별 현재 프로세스 log marker가 있어야 회수한다.
5. **실기기 우발 사용:** emulator 속성을 확인하고 emulator가 아닌 serial은 script 단계에서 실패한다.
6. **AVD 결과의 하드웨어 일반화:** ANGLE/SwiftShader metadata를 기록하고 결과를 simulator 한정으로 표시한다.
7. **CSS와 physical pixel 배율 불일치:** density와 실제 surface/bitmap 크기를 함께 검증하고 불일치하면 실패한다.
8. **표본의 반 픽셀 변환 오류:** Chrome 기준과 동일한 `(css coordinate + 0.5) × scale` floor 규칙을 명시했다.
9. **fixture origin 누락:** Android root 내 stage offset `(10, 10)`을 표본 좌표식에 포함한다.
10. **5개 미지원 case를 성공처럼 캡처:** 지원 21개만 surface pixel 비교하고 거부 5개는 별도 matrix gate에 둔다.
11. **장면 간 cascade/cache 오염:** 각 지원 case를 새 앱 프로세스와 새 HostDocument에서 실행한다.
12. **paint order 보조 API를 픽셀 oracle로 오용:** 고정 Chrome screenshot의 actual RGBA와 Android surface 표본을 비교한다.
13. **reference/asset 불일치:** inventory, reference, 앱 asset hash를 서로 연결하고 다른 값은 거부한다.
14. **샘플 평균으로 실제 오류 감춤:** 22개 표본 각각의 채널 오차를 독립적으로 제한한다.
15. **알파 합성 차이 은폐:** fixture는 불투명 단색 표본만 쓰고 alpha 255를 강제한다.
16. **PixelCopy 오류 후 빈 이미지 성공 처리:** callback 오류·빈 결과·범위 밖 좌표에서는 성공 marker를 기록하지 않는다.
17. **잘못된 case ID로 임의 source 실행:** 고정 matrix 내 지원 case ID만 허용한다.
18. **누락/중복 sample이나 screenshot 통과:** 정확한 case/sample 집합 및 수량 검증을 요구한다.
19. **픽셀 비교기 자체의 편향:** 좌표를 비교기가 다시 산출하고 색상은 고정 reference 값과 대조하며 negative control을 둔다.
20. **검증 뒤 문서가 오래된 상태로 남음:** 성공 후에만 상위 C12.4 계획, 상태 대장, 계약/evidence index, roadmap, Tailnet을 같은 결과로 동기화한다.

## 계획 수정 여부

검토에서 계획을 바꿀 미해결 모순은 찾지 못했다. 특히 surface readback, 새 HostDocument 격리, simulator 경계, CSS→pixel 좌표식, exact sample set, 고정 Chrome oracle을 별도 조건으로 기록했다. 구현 전 계획 검토이며 구현 성공의 근거로 사용하지 않는다.

