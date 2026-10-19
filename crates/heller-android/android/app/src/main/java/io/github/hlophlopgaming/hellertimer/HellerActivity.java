package io.github.hlophlopgaming.hellertimer;

import android.app.*;
import android.os.*;
import android.content.*;
import java.io.*;
import java.nio.charset.StandardCharsets;

public class HellerActivity extends NativeActivity {
    @Override public void onCreate(Bundle state) {
        super.onCreate(state);
        HellerBridge.channel(this);
        if (Build.VERSION.SDK_INT >= 33) requestPermissions(new String[]{"android.permission.POST_NOTIFICATIONS"}, 1);
    }
    public String loadData() {
        File file = new File(getFilesDir(), "data.json");
        if (!file.exists()) return "";
        try (FileInputStream in = new FileInputStream(file); ByteArrayOutputStream out = new ByteArrayOutputStream()) {
            byte[] buffer = new byte[4096]; int n;
            while ((n = in.read(buffer)) != -1) out.write(buffer, 0, n);
            return out.toString("UTF-8");
        } catch (IOException e) { android.util.Log.e("HellerTimer", "Load", e); return ""; }
    }
    public void saveData(String json) {
        File tmp = new File(getFilesDir(), "data.json.tmp");
        File dest = new File(getFilesDir(), "data.json");
        try (FileOutputStream out = new FileOutputStream(tmp)) {
            out.write(json.getBytes(StandardCharsets.UTF_8)); out.getFD().sync();
        } catch (IOException e) { android.util.Log.e("HellerTimer", "Save", e); return; }
        if (!tmp.renameTo(dest)) android.util.Log.e("HellerTimer", "Atomic rename failed");
    }
    public void schedule(long deadline, String text) { HellerBridge.schedule(this, deadline, text); }
    public void notifyTimer(String text) { HellerBridge.notify(this, text, 0); }
    public void openUrl(String url) { try { startActivity(new Intent(Intent.ACTION_VIEW, android.net.Uri.parse(url))); } catch (ActivityNotFoundException e) { android.util.Log.w("HellerTimer", "Open URL", e); } }
    public int utcOffset() { return java.util.TimeZone.getDefault().getOffset(System.currentTimeMillis()) / 1000; }
}
