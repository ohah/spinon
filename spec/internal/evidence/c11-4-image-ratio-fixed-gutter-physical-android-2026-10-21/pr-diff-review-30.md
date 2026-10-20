# C11.4 Android 실기기 보충 PR 변경 적대 검토 30개

검토 대상은 이번 문서·근거 변경, 실기기 capture 스크립트, pass31 Tailnet 원본 파일 동기화 경로다. 이 표는 별도 [실기기 동작 검토 20개](./physical-validation-review-20.md)와 pass29 이후 별도로 확인한 PR #357의 기존 구현 검토를 대체하지 않는다. 검토표 자체의 문구는 검토 대상에서 제외했다.

1. **공식 상태를 앞당길 위험** — `spec/STATUS.md`는 C11.4 체크와 완료 수가 미완료라고 명시한다. 완료 항목·집계 숫자는 추가되지 않았다.
2. **실기기 한 번을 플랫폼 전체로 확대** — README와 로드맵은 Samsung 한 대의 보충 검증이라고 범위를 제한한다.
3. **시뮬레이터 결과와 혼합** — 기록은 Android 16/API 36 물리 기기와 기존 API 37 AVD·iOS Simulator를 분리한다.
4. **가상 기기를 물리 기기로 오인** — capture가 정확한 ADB serial, `qemu=0`, 모델 및 API를 검증한다.
5. **잘못된 장치에서 증거 생성** — SM-S731N, API 36, ARM64, Xclipse 940/Vulkan 조건이 각각 고정 확인된다.
6. **다른 GPU 구현 결과를 같은 결과로 취급** — SurfaceFlinger renderer를 보존하고 Samsung ANGLE/Xclipse 940 관찰값을 적는다.
7. **오래된 설치 APK를 실행** — local debug APK와 설치 `base.apk` SHA-256을 비교하고 불일치 시 재설치 후 다시 검사한다.
8. **재빌드하지 않은 APK를 최신 코드라고 주장** — README와 STATUS는 동일 APK를 사용했고 새 빌드는 하지 않았다고 밝힌다.
9. **scene allowlist 이탈** — capture는 column/row fixed-gutter 두 scene ID를 코드에 고정한다.
10. **scene 로그 간 run ID 혼합** — scene별 고유 run ID로 로그를 골라 저장한다.
11. **frame만 통과하고 sizing trace가 누락** — summary의 성공과 trace 필드 `7 + 3 = 10`을 함께 검사한다.
12. **column만 확인하고 row 회귀를 놓침** — 두 방향을 별도 실행해 frame·trace·표본 수를 각각 판정한다.
13. **trace 수치 오해** — 기록된 7px fixed track과 3px 내부 gutter가 10px limit을 이룬다고 기재한다.
14. **PixelCopy 표본 수 부족** — 각 scene에서 기대·관찰·일치 표본이 모두 4개인지 검사한다.
15. **표본을 PNG texel 검증으로 확대 해석** — README는 선택 SurfaceView 색만 검증하며 PNG texel은 제외한다.
16. **표본을 compositor 이후 픽셀로 확대 해석** — 문서가 화면 합성 후 결과를 명시적으로 미검증 처리한다.
17. **화면 캡처 실패를 성공으로 기록** — 빈 screenshot 또는 surface PNG는 capture가 assert로 중단한다.
18. **잘못된 이미지 크기·출처 기록** — 로컬 및 Agent Browser에서 기기 screenshot 1080×2340을 확인했다.
19. **raw PNG 상대 링크 404** — pass31이 run 디렉터리를 `public/internal/evidence/`로 복사하고 Tailnet에서 직접 1080×2340 로드를 확인했다.
20. **JSON 직접 주소 404 또는 HTML fallback** — 원격 브라우저에서 원본 경로의 `application/json`, run ID, passed 상태를 확인했다.
21. **README 안 이미지 변환과 raw 주소를 혼동** — 변환 `static/image` 경로와 source-relative raw 경로를 각각 검증한다.
22. **포함 문서의 원본과 미리보기 불일치** — 로드맵, STATUS, 계획, evidence 본문을 Tailnet에서 각각 열어 해당 문구를 확인했다.
23. **문서 build 범위를 숨김** — pass31 보고서에서 선별 root build와 S04 이미지가 빠진 전체 build 실패를 구분한다.
24. **기존 preview 자산 삭제** — preflight/copy/verify 모두 삭제가 없고 sync 코드는 source 파일만 create/replace한다.
25. **기존 파일 무백업 덮어쓰기** — 교체 100개 전부 이전 크기·해시를 전용 backup root에 기록하고 검증한다.
26. **preflight 이후 대상 변경** — copy는 대상 존재 여부와 이전 SHA-256이 preflight와 같지 않으면 중단한다.
27. **심볼릭 링크 경로 공격** — source tree와 target root 경로를 각 segment별 `lstat`해 symlink를 거부한다.
28. **부분 파일 노출** — 임시 파일을 `wx`로 만들고 `fsync` 뒤 원자 rename을 사용한다.
29. **열린 파일을 가진 무관 프로세스 덮어쓰기** — 허용 PID·명령·listener 주소를 검사하며 다른 holder면 상세 로그와 함께 중단한다.
30. **미리보기 갱신 후 원격 파일 손상 또는 브라우저 미종료** — 로컬 1,760/1,760·백업 100/100과 원격 HTTP 200·해시 1,760/1,760을 확인하고 Agent Browser 세션 및 활성 browser 프로세스가 없는 것을 확인했다.

검토 중 수정한 결함은 원본 evidence 파일의 직접 URL이 Tailnet에서 404가 되는 경로였다. pass31 공개 mirror 추가 후 본문·PNG·JSON 경로를 다시 확인했다. 이번 PR은 문서와 검증 근거만 갱신하며 제품 동작은 바꾸지 않는다.
