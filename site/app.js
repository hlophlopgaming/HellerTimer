'use strict';
const root = document.documentElement;
try { root.classList.toggle('light', localStorage.getItem('heller-theme') === 'light'); } catch (_) {}
document.querySelector('#theme').addEventListener('click', () => {
  root.classList.toggle('light');
  try { localStorage.setItem('heller-theme', root.classList.contains('light') ? 'light' : 'dark'); } catch (_) {}
});
async function releases() {
  try {
    const response = await fetch('https://api.github.com/repos/hlophlopgaming/HellerTimer/releases/latest');
    if (!response.ok) throw new Error('No published release');
    const release = await response.json();
    const os = /Windows/i.test(navigator.userAgent) ? 'windows' : /Mac/i.test(navigator.userAgent) ? 'macos' : 'linux';
    const container = document.querySelector('#downloads');
    for (const asset of release.assets || []) {
      if (!/\.(exe|zip|dmg|deb|AppImage|apk|tar\.gz)$/i.test(asset.name)) continue;
      const url = new URL(asset.browser_download_url);
      if (url.hostname !== 'github.com' || url.protocol !== 'https:') continue;
      const a = document.createElement('a');
      a.className = 'button' + (asset.name.toLowerCase().includes(os) ? ' primary' : '');
      a.href = url.href; a.textContent = asset.name; container.append(a);
    }
    document.querySelector('#release-status').textContent = `${release.tag_name} · Choose the package for your OS and architecture.`;
  } catch (_) { document.querySelector('#release-status').textContent = 'No binary release published yet. Build from source or try the web demo.'; }
}
async function benchmark() {
  try {
    const response = await fetch('bench/latest.json');
    if (!response.ok) return;
    const data = await response.json();
    if (!Array.isArray(data.measurements) || !data.measurements.length) return;
    const container = document.querySelector('#benchmark'); container.replaceChildren();
    const title = document.createElement('p'); title.textContent = `Measured on ${String(data.host)} · private memory`; container.append(title);
    const max = Math.max(...data.measurements.map(row => Number(row.private_bytes)), 1);
    for (const row of data.measurements) {
      const bytes = Number(row.private_bytes);
      if (!Number.isFinite(bytes) || bytes < 0) continue;
      const label = document.createElement('p'); label.textContent = `${row.application} / ${row.scenario} — ${(bytes / 1048576).toFixed(1)} MiB · ${Number(row.cpu_percent).toFixed(2)}% CPU`;
      const bar = document.createElement('div'); bar.className = 'bar'; bar.style.width = `${Math.min(100, bytes / max * 100)}%`;
      container.append(label, bar);
    }
  } catch (_) { /* No published measurements yet. */ }
}
releases(); benchmark();
