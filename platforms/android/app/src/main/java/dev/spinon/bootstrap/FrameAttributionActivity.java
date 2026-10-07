package dev.spinon.bootstrap;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
import android.os.Bundle;
import android.os.Trace;
import android.view.Gravity;
import android.view.View;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

/** Android View 대조 조건을 제공하는 개발 전용 화면입니다. */
public final class FrameAttributionActivity extends Activity {
    public static final String EXTRA_MODE = "spinon_frame_mode";
    public static final String MODE_INPUT_ONLY = "input-only";
    public static final String MODE_STATUS = "status";
    public static final String MODE_LOG_SCROLL = "log-scroll";

    private String mode;
    private int inputCount;
    private TextView status;
    private TextView log;
    private ScrollView scroll;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        mode = getIntent().getStringExtra(EXTRA_MODE);
        if (mode == null) mode = MODE_INPUT_ONLY;
        if (!MODE_INPUT_ONLY.equals(mode)
                && !MODE_STATUS.equals(mode)
                && !MODE_LOG_SCROLL.equals(mode)) {
            TextView error = new TextView(this);
            error.setText("지원하지 않는 R05 대조 모드입니다: " + mode);
            setContentView(error);
            return;
        }

        float density = getResources().getDisplayMetrics().density;
        int inset = Math.round(18 * density);
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(inset, Math.round(20 * density), inset, inset);
        root.setBackgroundColor(Color.rgb(14, 19, 31));

        TextView title = new TextView(this);
        title.setText("SPINON · Android 프레임 대조 실험");
        title.setTextColor(Color.rgb(230, 237, 248));
        title.setTextSize(20);
        title.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        root.addView(title);

        TextView description = new TextView(this);
        description.setText("개발 전용 · 동일 Android View 입력 대조 · V8 호출 없음");
        description.setTextColor(Color.rgb(170, 184, 207));
        description.setTextSize(13);
        description.setPadding(0, Math.round(8 * density), 0, Math.round(12 * density));
        root.addView(description);

        status = new TextView(this);
        status.setText("준비됨 · 입력 0회");
        status.setTextColor(Color.rgb(97, 185, 255));
        status.setTextSize(14);
        root.addView(status);

        Button button = new Button(this);
        button.setText("벤치마크 탭");
        button.setAllCaps(false);
        button.setTextSize(16);
        button.setGravity(Gravity.CENTER_VERTICAL | Gravity.START);
        LinearLayout.LayoutParams buttonParams = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.WRAP_CONTENT);
        buttonParams.topMargin = Math.round(8 * density);
        root.addView(button, buttonParams);

        scroll = new ScrollView(this);
        scroll.setPadding(0, Math.round(16 * density), 0, 0);
        log = new TextView(this);
        log.setTextColor(Color.rgb(218, 226, 240));
        log.setTextSize(12);
        log.setTypeface(Typeface.MONOSPACE);
        log.setBackgroundColor(Color.rgb(8, 11, 17));
        log.setPadding(Math.round(6 * density), Math.round(12 * density),
                Math.round(6 * density), Math.round(20 * density));
        scroll.addView(log);
        root.addView(scroll, new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, 0, 1));
        setContentView(root);

        button.setOnClickListener(view -> onBenchmarkTap());
    }

    private void onBenchmarkTap() {
        Trace.beginSection("SpinonR05:control-input");
        try {
            inputCount++;
            Trace.setCounter("SpinonR05ControlInputCount", inputCount);
            if (MODE_STATUS.equals(mode)) {
                Trace.beginSection("SpinonR05:control-status-update");
                try {
                    status.setText("상태 갱신 · 입력 " + inputCount + "회");
                } finally {
                    Trace.endSection();
                }
            } else if (MODE_LOG_SCROLL.equals(mode)) {
                Trace.beginSection("SpinonR05:control-log-append");
                try {
                    log.append("Android View 대조 로그 " + inputCount + "\n");
                } finally {
                    Trace.endSection();
                }
                scroll.post(() -> {
                    Trace.beginSection("SpinonR05:control-log-scroll");
                    try {
                        scroll.fullScroll(View.FOCUS_DOWN);
                    } finally {
                        Trace.endSection();
                    }
                });
            }
        } finally {
            Trace.endSection();
        }
    }
}
