package TARGET_PACKAGE_NAME;

import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

import android.app.Activity;
import android.os.Bundle;
import android.os.Build;
import android.util.Log;
import android.util.TypedValue;

import android.view.View;
import android.view.ViewGroup;
import android.view.Surface;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowManager.LayoutParams;
import android.view.SurfaceView;
import android.view.SurfaceHolder;
import android.view.MotionEvent;
import android.view.KeyEvent;
import android.view.inputmethod.InputMethodManager;

import android.content.Context;
import android.content.Intent;
import android.content.res.Configuration;
import android.content.ClipData;
import android.content.ClipboardManager;

import android.graphics.Color;
import android.graphics.Insets;
import android.view.inputmethod.InputConnection;
import android.view.inputmethod.EditorInfo;
import android.widget.LinearLayout;
import android.widget.FrameLayout;
import android.widget.EditText;
import android.widget.TextView;
import android.text.Editable;
import android.text.Selection;
import android.text.TextWatcher;
import android.text.InputType;

import quad_native.QuadNative;

// note: //% is a special miniquad's pre-processor for plugins
// when there are no plugins - //% whatever will be replaced to an empty string
// before compiling

//% IMPORTS

class HiddenEditText extends EditText {
    private boolean mIgnoreUpdates = false;
    private boolean mInitialized = false;

    public HiddenEditText(Context context) {
        super(context);
        setBackgroundColor(Color.TRANSPARENT);
        setTextColor(Color.TRANSPARENT);
        setAlpha(0.01f); // Visible to WindowManager, but transparent to user
        setFocusable(true);
        setFocusableInTouchMode(true);
        setImportantForAccessibility(View.IMPORTANT_FOR_ACCESSIBILITY_YES);
        setLayoutParams(new android.widget.FrameLayout.LayoutParams(1, 1));

        addTextChangedListener(new TextWatcher() {
            @Override
            public void beforeTextChanged(CharSequence s, int start, int count, int after) {}

            @Override
            public void onTextChanged(CharSequence s, int start, int before, int count) {}

            @Override
            public void afterTextChanged(Editable s) {
                sendStateToRust();
            }
        });

        setOnEditorActionListener(new TextView.OnEditorActionListener() {
            @Override
            public boolean onEditorAction(TextView v, int actionId, KeyEvent event) {
                QuadNative.surfaceOnImeAction(actionId);
                return false;
            }
        });

        mInitialized = true;
    }

    @Override
    public boolean dispatchKeyEvent(KeyEvent event) {
        return super.dispatchKeyEvent(event);
    }

    @Override
    public boolean onKeyDown(int keyCode, KeyEvent event) {
        if (keyCode == KeyEvent.KEYCODE_DPAD_UP || keyCode == KeyEvent.KEYCODE_DPAD_DOWN
                || keyCode == KeyEvent.KEYCODE_DPAD_LEFT || keyCode == KeyEvent.KEYCODE_DPAD_RIGHT
                || keyCode == KeyEvent.KEYCODE_MOVE_HOME || keyCode == KeyEvent.KEYCODE_MOVE_END
                || keyCode == KeyEvent.KEYCODE_PAGE_UP || keyCode == KeyEvent.KEYCODE_PAGE_DOWN) {
            if (event.isShiftPressed()) {
                QuadNative.surfaceOnKeyDown(KeyEvent.KEYCODE_SHIFT_LEFT);
            }
            QuadNative.surfaceOnKeyDown(keyCode);
            return true;
        }
        return super.onKeyDown(keyCode, event);
    }

    @Override
    public boolean onKeyUp(int keyCode, KeyEvent event) {
        if (keyCode == KeyEvent.KEYCODE_DPAD_UP || keyCode == KeyEvent.KEYCODE_DPAD_DOWN
                || keyCode == KeyEvent.KEYCODE_DPAD_LEFT || keyCode == KeyEvent.KEYCODE_DPAD_RIGHT
                || keyCode == KeyEvent.KEYCODE_MOVE_HOME || keyCode == KeyEvent.KEYCODE_MOVE_END
                || keyCode == KeyEvent.KEYCODE_PAGE_UP || keyCode == KeyEvent.KEYCODE_PAGE_DOWN) {
            QuadNative.surfaceOnKeyUp(keyCode);
            if (!event.isShiftPressed()) {
                QuadNative.surfaceOnKeyUp(KeyEvent.KEYCODE_SHIFT_LEFT);
            }
            return true;
        }
        return super.onKeyUp(keyCode, event);
    }

    @Override
    protected void onSelectionChanged(int selStart, int selEnd) {
        super.onSelectionChanged(selStart, selEnd);
        sendStateToRust();
    }

    public void setIgnoreUpdates(boolean ignore) {
        mIgnoreUpdates = ignore;
    }

    public void updateStateFromRust(String text, int selectionStart, int selectionEnd) {
        mIgnoreUpdates = true;
        try {
            String currentText = getText() != null ? getText().toString() : "";
            if (!currentText.equals(text)) {
                setText(text);
            }
            int len = getText().length();
            int start = Math.max(0, Math.min(len, selectionStart));
            int end = Math.max(0, Math.min(len, selectionEnd));
            setSelection(start, end);
        } finally {
            mIgnoreUpdates = false;
        }
    }

    private void sendStateToRust() {
        if (!mInitialized || mIgnoreUpdates) return;
        if (MainActivity.sFocusedElementId == -1 || MainActivity.sFocusedElementId == 0) return;

        Editable editable = getText();
        if (editable == null) return;

        String text = editable.toString();
        int selectionStart = Math.max(0, Selection.getSelectionStart(editable));
        int selectionEnd = Math.max(0, Selection.getSelectionEnd(editable));
        int composingStart = android.view.inputmethod.BaseInputConnection.getComposingSpanStart(editable);
        int composingEnd = android.view.inputmethod.BaseInputConnection.getComposingSpanEnd(editable);

        MainActivity.sTextInputText = text;
        MainActivity.sTextInputSelectionStart = selectionStart;
        MainActivity.sTextInputSelectionEnd = selectionEnd;

        QuadNative.surfaceOnImeStateChanged(text, selectionStart, selectionEnd, composingStart, composingEnd, MainActivity.sFocusedElementId);
    }
}

class QuadSurface
    extends
        SurfaceView
    implements
        View.OnTouchListener,
        View.OnKeyListener,
        SurfaceHolder.Callback {

    public QuadSurface(Context context){
        super(context);
        getHolder().addCallback(this);

        setFocusable(true);
        setFocusableInTouchMode(true);
        requestFocus();
        setOnTouchListener(this);
        setOnKeyListener(this);
    }

    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        QuadNative.surfaceOnSurfaceCreated(getNativeSurface());
    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        QuadNative.surfaceOnSurfaceDestroyed(getNativeSurface());
    }

    @Override
    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        QuadNative.surfaceOnSurfaceChanged(getNativeSurface(), width, height);
    }

    private float mDownX = 0;
    private float mDownY = 0;
    private boolean mIsLongPress = false;
    private Runnable mLongPressRunnable = new Runnable() {
        @Override
        public void run() {
            if (MainActivity.sFocusedElementId != 0 && MainActivity.sFocusedElementId != -1 && MainActivity.sHiddenEditText != null) {
                mIsLongPress = true;
                final float x = mDownX;
                final float y = mDownY;
                MainActivity.sHiddenEditText.post(new Runnable() {
                    @Override
                    public void run() {
                        if (MainActivity.sHiddenEditText != null) {
                            MainActivity.sHiddenEditText.requestFocus();
                            if (android.os.Build.VERSION.SDK_INT >= 24) {
                                MainActivity.sHiddenEditText.showContextMenu(x, y);
                            } else {
                                MainActivity.sHiddenEditText.showContextMenu();
                            }
                        }
                    }
                });
            }
        }
    };

    @Override
    public boolean onTouch(View v, MotionEvent event) {
        int pointerCount = event.getPointerCount();

        int action = event.getActionMasked();
        switch (action) {
        case MotionEvent.ACTION_DOWN:
        case MotionEvent.ACTION_POINTER_DOWN: {
            int pointerIndex = event.getActionIndex();
            int id = event.getPointerId(pointerIndex);
            final float x = event.getX(pointerIndex);
            final float y = event.getY(pointerIndex);
            mDownX = x;
            mDownY = y;
            mIsLongPress = false;
            if (getHandler() != null) {
                getHandler().removeCallbacks(mLongPressRunnable);
                getHandler().postDelayed(mLongPressRunnable, 500);
            }
            QuadNative.surfaceOnTouch(id, 0, x, y);

            break;
        }
        case MotionEvent.ACTION_MOVE: {
            for (int i = 0; i < pointerCount; i++) {
                final int id = event.getPointerId(i);
                final float x = event.getX(i);
                final float y = event.getY(i);
                if (Math.hypot(x - mDownX, y - mDownY) > 25 && getHandler() != null) {
                    getHandler().removeCallbacks(mLongPressRunnable);
                }
                QuadNative.surfaceOnTouch(id, 1, x, y);
            }
            break;
        }
        case MotionEvent.ACTION_UP:
        case MotionEvent.ACTION_POINTER_UP: {
            if (getHandler() != null) {
                getHandler().removeCallbacks(mLongPressRunnable);
            }
            int pointerIndex = event.getActionIndex();
            int id = event.getPointerId(pointerIndex);
            final float x = event.getX(pointerIndex);
            final float y = event.getY(pointerIndex);
            if (!mIsLongPress && MainActivity.sHiddenEditText != null && MainActivity.sFocusedElementId != 0 && MainActivity.sFocusedElementId != -1) {
                InputMethodManager imm = (InputMethodManager) getContext().getSystemService(Context.INPUT_METHOD_SERVICE);
                if (imm != null) {
                    imm.viewClicked(MainActivity.sHiddenEditText);
                }
            }
            int phase = mIsLongPress ? 3 : 2;
            QuadNative.surfaceOnTouch(id, phase, x, y);
            break;
        }
        case MotionEvent.ACTION_CANCEL: {
            if (getHandler() != null) {
                getHandler().removeCallbacks(mLongPressRunnable);
            }
            for (int i = 0; i < pointerCount; i++) {
                final int id = event.getPointerId(i);
                final float x = event.getX(i);
                final float y = event.getY(i);
                QuadNative.surfaceOnTouch(id, 3, x, y);
            }
            break;
        }
        default:
            break;
        }

        return true;
    }

    @SuppressWarnings("deprecation")
    @Override
    public boolean onKey(View v, int keyCode, KeyEvent event) {
        if (MainActivity.sFocusedElementId != -1 && MainActivity.sFocusedElementId != 0) {
            return false;
        }

        if (event.getAction() == KeyEvent.ACTION_DOWN && keyCode != 0) {
            QuadNative.surfaceOnKeyDown(keyCode);
        }

        if (event.getAction() == KeyEvent.ACTION_UP && keyCode != 0) {
            QuadNative.surfaceOnKeyUp(keyCode);
        }

        if (event.getAction() == KeyEvent.ACTION_UP || event.getAction() == KeyEvent.ACTION_MULTIPLE) {
            int character = event.getUnicodeChar();
            if (character == 0) {
                String characters = event.getCharacters();
                if (characters != null && !characters.isEmpty()) {
                    character = characters.charAt(0);
                }
            }

            if (character != 0) {
                QuadNative.surfaceOnCharacter(character);
            }
        }

        return true;
    }

    @Override
    public InputConnection onCreateInputConnection(EditorInfo outAttrs) {
        if (MainActivity.sHiddenEditText != null && MainActivity.sFocusedElementId != -1 && MainActivity.sFocusedElementId != 0) {
            return MainActivity.sHiddenEditText.onCreateInputConnection(outAttrs);
        }
        return null;
    }

    public Surface getNativeSurface() {
        return getHolder().getSurface();
    }
}

class ResizingLayout
    extends
        FrameLayout
    implements
        View.OnApplyWindowInsetsListener {
    //% RESIZING_LAYOUT_BODY

    public ResizingLayout(MainActivity activity){
        super(activity);
        setBackgroundColor(Color.BLACK);
        setOnApplyWindowInsetsListener(this);

        //% RESIZING_LAYOUT_CONSTRUCTOR
    }

    @Override
    public WindowInsets onApplyWindowInsets(View v, WindowInsets insets) {
        if (Build.VERSION.SDK_INT >= 30) {
            Insets imeInsets = insets.getInsets(WindowInsets.Type.ime());
            Insets sysInsets = insets.getInsets(WindowInsets.Type.systemBars());

            int bottomPadding = sysInsets.bottom;
            if (imeInsets.bottom > 0) {
                bottomPadding = imeInsets.bottom;
            }

            v.setPadding(
                sysInsets.left,
                sysInsets.top,
                sysInsets.right,
                bottomPadding
            );
        }
        return insets;
    }
}

public class MainActivity extends Activity {
    //% MAIN_ACTIVITY_BODY

    private QuadSurface view;

    public static MainActivity sInstance = null;
    public static HiddenEditText sHiddenEditText = null;
    public static String sTextInputText = "";
    public static int sTextInputSelectionStart = 0;
    public static int sTextInputSelectionEnd = 0;
    public static boolean sTextInputIsPassword = false;
    public static boolean sTextInputIsMultiline = false;
    public static long sFocusedElementId = -1;

    public static String getClipboardTextStatic() {
        if (sInstance != null) {
            return sInstance.getClipboardText();
        }
        return null;
    }

    public static void setClipboardTextStatic(String text) {
        if (sInstance != null) {
            sInstance.setClipboardText(text);
        }
    }

    public void updateTextInputState(final String text, final int selectionStart, final int selectionEnd, final boolean isPassword, final boolean isMultiline, final long elementId, final int maxLength) {
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                if (sHiddenEditText == null) return;

                sHiddenEditText.setIgnoreUpdates(true);
                try {
                    boolean focusChanged = (sFocusedElementId != elementId);
                    boolean paramsChanged = (sTextInputIsPassword != isPassword) || (sTextInputIsMultiline != isMultiline);

                    sTextInputText = text != null ? text : "";
                    sTextInputSelectionStart = selectionStart;
                    sTextInputSelectionEnd = selectionEnd;
                    sTextInputIsPassword = isPassword;
                    sTextInputIsMultiline = isMultiline;
                    sFocusedElementId = elementId;

                    if (maxLength > 0) {
                        sHiddenEditText.setFilters(new android.text.InputFilter[] { new android.text.InputFilter.LengthFilter(maxLength) });
                    } else {
                        sHiddenEditText.setFilters(new android.text.InputFilter[0]);
                    }

                    int inputType = EditorInfo.TYPE_CLASS_TEXT;
                    int imeOptions = EditorInfo.IME_FLAG_NO_FULLSCREEN;
                    if (isPassword) {
                        inputType |= EditorInfo.TYPE_TEXT_VARIATION_PASSWORD;
                        imeOptions |= EditorInfo.IME_ACTION_DONE;
                    } else if (isMultiline) {
                        inputType |= EditorInfo.TYPE_TEXT_FLAG_MULTI_LINE;
                        imeOptions |= EditorInfo.IME_ACTION_NONE;
                    } else {
                        imeOptions |= EditorInfo.IME_ACTION_DONE;
                    }

                    sHiddenEditText.setInputType(inputType);
                    sHiddenEditText.setImeOptions(imeOptions);

                    Editable editable = sHiddenEditText.getText();
                    String currentText = editable != null ? editable.toString() : "";
                    boolean textChanged = !currentText.equals(sTextInputText);
                    if (textChanged) {
                        if (editable != null) {
                            int compStart = android.view.inputmethod.BaseInputConnection.getComposingSpanStart(editable);
                            int compEnd = android.view.inputmethod.BaseInputConnection.getComposingSpanEnd(editable);
                            if (compStart != -1 || compEnd != -1) {
                                android.view.inputmethod.BaseInputConnection.removeComposingSpans(editable);
                            }
                        }
                        sHiddenEditText.setText(sTextInputText);
                    }

                    int len = sHiddenEditText.getText().length();
                    int start = Math.max(0, Math.min(len, sTextInputSelectionStart));
                    int end = Math.max(0, Math.min(len, sTextInputSelectionEnd));

                    sHiddenEditText.setSelection(start, end);

                    InputMethodManager imm = (InputMethodManager) getSystemService(Context.INPUT_METHOD_SERVICE);
                    if (elementId != 0 && elementId != -1) {
                        sHiddenEditText.requestFocus();
                        if (imm != null) {
                            if (focusChanged || paramsChanged) {
                                imm.restartInput(sHiddenEditText);
                            }
                            imm.showSoftInput(sHiddenEditText, InputMethodManager.SHOW_IMPLICIT);
                            imm.updateSelection(sHiddenEditText, start, end, -1, -1);
                        }
                    } else {
                        sHiddenEditText.clearFocus();
                        if (imm != null) {
                            imm.hideSoftInputFromWindow(sHiddenEditText.getWindowToken(), 0);
                        }
                    }
                } finally {
                    sHiddenEditText.setIgnoreUpdates(false);
                }
            }
        });
    }

    static {
        System.loadLibrary("LIBRARY_NAME");
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        sInstance = this;
        super.onCreate(savedInstanceState);

        this.requestWindowFeature(Window.FEATURE_NO_TITLE);

        view = new QuadSurface(this);
        sHiddenEditText = new HiddenEditText(this);

        ResizingLayout layout = new ResizingLayout(this);
        layout.addView(view);
        layout.addView(sHiddenEditText);
        setContentView(layout);

        QuadNative.activityOnCreate(this);

        //% MAIN_ACTIVITY_ON_CREATE
    }

    @Override
    protected void onResume() {
        super.onResume();
        QuadNative.activityOnResume();

        //% MAIN_ACTIVITY_ON_RESUME
    }

    @Override
    public void onBackPressed() {
        Log.w("SAPP", "onBackPressed");

        // TODO: here is the place to handle request_quit/order_quit/cancel_quit

        super.onBackPressed();
    }

    @Override
    protected void onStop() {
        super.onStop();
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();

        QuadNative.activityOnDestroy();
    }

    @Override
    protected void onPause() {
        super.onPause();
        QuadNative.activityOnPause();

        //% MAIN_ACTIVITY_ON_PAUSE
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        //% MAIN_ACTIVITY_ON_ACTIVITY_RESULT
    }

    public void setFullScreen(final boolean fullscreen) {
        runOnUiThread(new Runnable() {
                @Override
                public void run() {
                    View decorView = getWindow().getDecorView();

                    if (fullscreen) {
                        getWindow().setFlags(LayoutParams.FLAG_LAYOUT_NO_LIMITS, LayoutParams.FLAG_LAYOUT_NO_LIMITS);
                        if (Build.VERSION.SDK_INT >= 28) {
                            getWindow().getAttributes().layoutInDisplayCutoutMode = LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES;
                        }
                        if (Build.VERSION.SDK_INT >= 30) {
                            getWindow().setDecorFitsSystemWindows(false);
                        } else {
                            int uiOptions = View.SYSTEM_UI_FLAG_HIDE_NAVIGATION | View.SYSTEM_UI_FLAG_FULLSCREEN | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY;
                            decorView.setSystemUiVisibility(uiOptions);
                        }
                    }
                    else {
                        if (Build.VERSION.SDK_INT >= 30) {
                            getWindow().setDecorFitsSystemWindows(true);
                        } else {
                          decorView.setSystemUiVisibility(0);
                        }

                    }
                }
            });
    }

    public void showKeyboard(final boolean show) {
        runOnUiThread(new Runnable() {
                @Override
                public void run() {
                    if (sHiddenEditText == null) return;
                    InputMethodManager imm = (InputMethodManager) getSystemService(Context.INPUT_METHOD_SERVICE);
                    if (imm == null) return;

                    if (show) {
                        sHiddenEditText.requestFocus();
                        imm.showSoftInput(sHiddenEditText, InputMethodManager.SHOW_IMPLICIT);
                    } else {
                        sHiddenEditText.clearFocus();
                        imm.hideSoftInputFromWindow(sHiddenEditText.getWindowToken(), 0);
                    }
                }
            });
    }

    public String getClipboardText() {
        ClipboardManager clipboard = (ClipboardManager) getSystemService(Context.CLIPBOARD_SERVICE);

        if (!clipboard.hasPrimaryClip())
            return null;

        ClipData primaryClip = clipboard.getPrimaryClip();
        if (primaryClip == null || primaryClip.getItemCount() < 1)
            return null;

        CharSequence clipData = clipboard.getPrimaryClip().getItemAt(0).getText();
        if (clipData == null) {
            return null;
        }

        return clipData.toString();
    }
    public void setClipboardText(String text) {
        ClipboardManager clipboard = (ClipboardManager) getSystemService(Context.CLIPBOARD_SERVICE);
        ClipData clip = ClipData.newPlainText("label", text);
        clipboard.setPrimaryClip(clip);
    }
}

