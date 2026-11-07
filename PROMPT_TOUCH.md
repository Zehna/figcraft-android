# Spesifikasi dan Panduan Pengembanan Antarmuka Touch (Android / Mobile)

Dokumen ini mendefinisikan seluruh tahapan pengembangan adaptasi touch untuk FigCraft di platform mobile dan Android:

## 1. Safe Area Insets
- **Latar Belakang**: Layar perangkat modern memiliki kamera punch-hole / notch di bagian atas dan gesture navigation bar / home pill di bagian bawah.
- **Implementasi**:
  - Penambahan struktur `SafeAreaInsets` (`top`, `right`, `bottom`, `left`) di state antarmuka.
  - Nilai default otomatis disesuaikan untuk Android (misal: top 24pt, bottom 24pt).
  - Penyesuaian `tab_bar` bagian atas agar memiliki padding top dari safe area insets.
  - Penyesuaian floating bottom toolbar (`pill_rect`) dan tombol mobile agar terangkat di atas navigasi bawah Android.
  - Penyesuaian margin/padding pada panel samping (kiri/kanan) dalam mode landscape.

## 2. Touch Scaling 1.25x
- **Latar Belakang**: Target sentuh pada layar sentuh ponsel/tablet membutuhkan ukuran yang lebih lega agar nyaman ditekan oleh jari dibandingkan kursor mouse.
- **Implementasi**:
  - Dukungan faktor skala touch 1.25x (`touch_scale = 1.25`).
  - Penyesuaian kalkulasi `ui_scale::plan` agar pada mode touch atau layar narrow, batas constraint `FIT_POINTS` desktop (800x500) dilonggarkan ke `MOBILE_FIT_POINTS` (320x480), sehingga zoom level 1.25x dapat diterapkan penuh tanpa terpotong atau dipaksa turun ke 1.0x.
  - Menghasilkan ukuran tombol, teks, ikon, dan area interaksi yang 25% lebih besar dan proporsional untuk sentuhan jari.

## 3. Gestur Multi-Touch Pan/Zoom
- **Latar Belakang**: Gestur dua jari (pinch-to-zoom dan pan) pada layar sentuh seringkali terjadi bersamaan, namun sebelumnya ditangani secara mutual-eksklusif (`else if`).
- **Implementasi**:
  - Pemrosesan simultan untuk zoom dan pan dua jari dalam satu frame.
  - Titik tumpu zoom (anchor) dipusatkan secara dinamis pada `center_pos` dari multi-touch (centroid jari pengguna).
  - Pembatalan atau supresi interaksi tool single-finger (misal marquee selection atau shape drag) ketika gestur multi-touch (≥2 jari) aktif, mencegah goresan atau seleksi yang tidak disengaja.

## 4. Virtual Modifier Puck
- **Latar Belakang**: Alur kerja profesional desain (Figma/Illustrator) bergantung pada tombol modifier keyboard (Shift untuk menjaga proporsi / snap / multi-select, Alt/Option untuk duplikasi dan value scrubbing, Ctrl/Cmd untuk deep selection). Di layar sentuh, tidak tersedia keyboard fisik saat menggambar.
- **Implementasi**:
  - Komponen melayang "Modifier Puck" yang dapat digeser (draggable) ke posisi nyaman pengguna.
  - Menyediakan tombol sentuh untuk `Shift`, `Alt`, dan `Cmd/Ctrl`.
  - Mendukung penekanan langsung (hold saat menggambar/menyeret dengan tangan lain) dan mode penguncian (lock / latch via tap/double-tap) dengan indikator visual yang jelas.
  - Terintegrasi langsung dengan modifier state aplikasi, sehingga gestur canvas dan field scrubbing otomatis merespon modifier aktif.

## 5. Value Scrubbing untuk Layar Sentuh
- **Latar Belakang**: Scrubbing nilai numerik (menggeser label angka untuk menaikkan/menurunkan nilai) sangat penting untuk tweaking ukuran dan koordinat.
- **Implementasi**:
  - Pelebaran area sentuh (touch hit target) label numerik menjadi minimal 28–32 pt sehingga mudah digeser oleh jari.
  - Integrasi dengan Virtual Modifier Puck: saat modifier `Alt` aktif, seluruh area input field dapat disentuh dan digeser untuk langsung melakukan scrubbing nilai.
  - Tampilan visual umpan balik (scrub notice / badge) yang informatif saat scrubbing berlangsung.
