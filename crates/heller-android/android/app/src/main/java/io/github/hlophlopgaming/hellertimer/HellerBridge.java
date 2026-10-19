package io.github.hlophlopgaming.hellertimer;

import android.app.*;
import android.content.*;
import android.os.*;

public final class HellerBridge {
    static final String CHANNEL = "timer";
    static void channel(Context c) {
        NotificationManager manager = c.getSystemService(NotificationManager.class);
        manager.createNotificationChannel(new NotificationChannel(CHANNEL, "Timer", NotificationManager.IMPORTANCE_DEFAULT));
    }
    static PendingIntent alarm(Context c, String text) {
        return PendingIntent.getBroadcast(c, 1, new Intent(c, AlarmReceiver.class).putExtra("text", text), PendingIntent.FLAG_UPDATE_CURRENT | PendingIntent.FLAG_IMMUTABLE);
    }
    static void schedule(Context c, long deadline, String text) {
        AlarmManager manager = c.getSystemService(AlarmManager.class);
        PendingIntent intent = alarm(c, text);
        manager.cancel(intent);
        if (deadline == 0) { c.getSystemService(NotificationManager.class).cancel(1); return; }
        try {
            if (Build.VERSION.SDK_INT < 31 || manager.canScheduleExactAlarms()) manager.setExactAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, deadline, intent);
            else manager.setAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, deadline, intent);
        } catch (SecurityException e) { android.util.Log.w("HellerTimer", "Exact alarm permission", e); }
        notify(c, text, deadline);
    }
    static void notify(Context c, String text, long deadline) {
        channel(c);
        PendingIntent open = PendingIntent.getActivity(c, 0, new Intent(c, HellerActivity.class), PendingIntent.FLAG_UPDATE_CURRENT | PendingIntent.FLAG_IMMUTABLE);
        Notification.Builder b = new Notification.Builder(c, CHANNEL)
            .setSmallIcon(android.R.drawable.ic_lock_idle_alarm).setContentTitle("HellerTimer")
            .setContentText(text).setContentIntent(open).setOnlyAlertOnce(deadline != 0);
        if (deadline != 0) b.setOngoing(true).setWhen(deadline).setUsesChronometer(true).setChronometerCountDown(true);
        try { c.getSystemService(NotificationManager.class).notify(1, b.build()); }
        catch (SecurityException e) { android.util.Log.w("HellerTimer", "Notification permission", e); }
    }
}
