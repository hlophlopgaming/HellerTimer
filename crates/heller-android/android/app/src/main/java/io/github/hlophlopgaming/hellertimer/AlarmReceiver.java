package io.github.hlophlopgaming.hellertimer;
import android.content.*;
public class AlarmReceiver extends BroadcastReceiver {
    @Override public void onReceive(Context c, Intent intent) { HellerBridge.notify(c, "Timer finished · open HellerTimer to continue", 0); }
}
