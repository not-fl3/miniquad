package quad_native;

import android.view.Surface;

// Java force the MainActivity class belongs to a specially named package
// this package name is forced to be different for each app
// and Java do not have any way to specify a native symbol lookup name..
// To workaround this - native callbacks can't be defined in dynamically named
// MainActivity package
public class QuadNative {
    // belongs to MainActivity class
    public native static void activityOnCreate(Object activity);
    public native static void activityOnResume();
    public native static void activityOnPause();
    public native static void activityOnDestroy();

    // belongs to QuadSurface class
    public native static void surfaceOnSurfaceCreated(Surface surface);
    public native static void surfaceOnSurfaceDestroyed(Surface surface);
    public native static void surfaceOnTouch(int id, int phase, float x, float y);
    public native static void surfaceOnSurfaceChanged(Surface surface, int width, int height);
}
