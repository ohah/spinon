# C04.10 Android surface 수명주기 실행 근거

- 실행 시각: 2026-10-09 22:10 KST
- 기기: Android Emulator, API 37, `arm64-v8a`, 1080×2400, 420 dpi
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bash tools/build-android-app.sh`
- 실행 모드: 실제 V8 fixture와 WGPU surface를 띄운 뒤 emulator 회전을 잠금 1회 적용하고 기본 방향으로 복구
- 화면: [회전 복구 뒤 Android 화면](c04-runtime-css-to-gpu-android-lifecycle-2026-10-09.png)
- 선별 로그: [surface 생성·종료 로그](c04-runtime-css-to-gpu-android-lifecycle-2026-10-09.log)

## 확인 결과

- 두 번의 Activity/surface 재생성 모두에서 `surfaceDestroyed`가 render queue의 renderer 파괴 완료 뒤에 반환했다. 로그 순서는 `SPINON_C0410_RENDERER_DESTROY status=0` → `SPINON_C0410_SURFACE_DESTROYED_DRAINED generation=3` → `SPINON_C0410_DISPOSE_DRAINED generation=4`다.
- 각 재생성 뒤 새 surface가 만들어졌고, renderer 생성 및 `presented boxes=3`가 다시 성공했다.
- 세대 로그는 surface 생성 1, 크기 설정 2, 종료 3, host dispose 4 순으로 증가했다. 이 재실행에서는 surface 크기가 계속 790×263이어서 **실제 크기가 바뀌는 resize 경로는 검증하지 않았다.**
- renderer는 `Gl` surface를 사용했고 ANGLE의 Vulkan SwiftShader 소프트웨어 장치에 연결됐다. 하드웨어 GPU 성능이나 실기기 동작 근거가 아니다.

회전 callback 대기 시간은 이 실행에서 수 ms 단위였으나, 고부하/장치 손실 시 최악 시간이나 main-thread stall 한도는 측정하지 않았다. Android SurfaceView는 drawing thread가 surface를 사용하는 동안 `surfaceDestroyed`가 반환하지 않도록 동기화해야 한다는 공식 API 계약을 따른다.
