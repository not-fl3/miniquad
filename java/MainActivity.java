package TARGET_PACKAGE_NAME;

import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

import android.app.Activity;
import android.os.Bundle;
import android.util.Log;

import android.view.View;
import android.view.Surface;
import android.view.Window;
import android.view.SurfaceView;
import android.view.SurfaceHolder;
import android.view.MotionEvent;

import android.content.Context;
import android.content.Intent;

import quad_native.QuadNative;

// note: //% is a special miniquad's pre-processor for plugins
// when there are no plugins - //% whatever will be replaced to an empty string
// before compiling

//% IMPORTS

class QuadSurface
    extends
        SurfaceView
    implements
        View.OnTouchListener,
        SurfaceHolder.Callback {

    public QuadSurface(Context context){
        super(context);
        getHolder().addCallback(this);

        setFocusable(true);
        setFocusableInTouchMode(true);
        requestFocus();
        setOnTouchListener(this);
    }

    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        Log.i("SAPP", "surfaceCreated");
        Surface surface = holder.getSurface();
        QuadNative.surfaceOnSurfaceCreated(surface);
    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        Log.i("SAPP", "surfaceDestroyed");
        Surface surface = holder.getSurface();
        QuadNative.surfaceOnSurfaceDestroyed(surface);
    }

    @Override
    public void surfaceChanged(SurfaceHolder holder,
                               int format,
                               int width,
                               int height) {
        Log.i("SAPP", "surfaceChanged");
        Surface surface = holder.getSurface();
        QuadNative.surfaceOnSurfaceChanged(surface, width, height);

    }

    @Override
    public boolean onTouch(View v, MotionEvent event) {
        int pointerCount = event.getPointerCount();
        int action = event.getActionMasked();
        int i, id;
        float x, y;

        switch(action) {
        case MotionEvent.ACTION_MOVE:
            for (i = 0; i < pointerCount; i++) {
                id = event.getPointerId(i);
                x = event.getX(i);
                y = event.getY(i);
                QuadNative.surfaceOnTouch(id, 0, x, y);
            }
            break;
        case MotionEvent.ACTION_UP:
            id = event.getPointerId(0);
            x = event.getX(0);
            y = event.getY(0);
            QuadNative.surfaceOnTouch(id, 1, x, y);
            break;
        case MotionEvent.ACTION_DOWN:
            id = event.getPointerId(0);
            x = event.getX(0);
            y = event.getY(0);
            QuadNative.surfaceOnTouch(id, 2, x, y);
        case MotionEvent.ACTION_CANCEL:
            for (i = 0; i < pointerCount; i++) {
                id = event.getPointerId(i);
                x = event.getX(i);
                y = event.getY(i);
                QuadNative.surfaceOnTouch(id, 3, x, y);
            }
            break;
        default:
            break;
        }

        return true;
    }

    public Surface getNativeSurface() {
        return getHolder().getSurface();
    }
}

public class MainActivity extends Activity {
    //% MAIN_ACTIVITY_BODY

    private QuadSurface view;

    static {
        System.loadLibrary("LIBRARY_NAME");
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        this.requestWindowFeature(Window.FEATURE_NO_TITLE);

        view = new QuadSurface(this);
        setContentView(view);

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
}

