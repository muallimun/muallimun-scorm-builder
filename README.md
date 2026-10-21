<h1 align="center">
  📦 Muallimun SCORM Paketleyici
</h1>

<p align="center">
  <strong>E-İçeriklerinizi Işık Hızında, Sıfır RAM Harcayarak SCORM Standartlarında Paketleyin!</strong><br>
  <em>Package your e-learning content at lightning speed with zero RAM usage in SCORM standards!</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-v2.0-24C8DB?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri v2">
  <img src="https://img.shields.io/badge/Rust-Backend-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/HTML5-Frontend-E34F26?style=for-the-badge&logo=html5&logoColor=white" alt="HTML5">
  <img src="https://img.shields.io/badge/Size-~2%20MB-success?style=for-the-badge" alt="Size">
</p>

---

## 🇹🇷 Türkçe (Turkish)

**Muallimun SCORM Paketleyici**, EBA (Eğitim Bilişim Ağı) ve tüm popüler LMS (Öğrenme Yönetim Sistemleri) altyapıları için geliştirilmiş tam uyumlu, ultra hafif ve yeni nesil bir masaüstü e-içerik paketleme yazılımıdır. 

Tarayıcıların RAM ve depolama limitlerine takılan eski nesil web araçlarının aksine, gücünü **Rust** programlama dilinden alır ve devasa projeleri bile saniyeler içinde işler.

### ✨ Premium Özellikler

- 🚀 **Işık Hızında ZIP Motoru:** Rust tabanlı çekirdeği sayesinde binlerce dosyadan oluşan HTML5 içeriklerini işletim sistemini yormadan, sıfır RAM darboğazı ile saniyeler içinde `.zip` formatına dönüştürür.
- 🪶 **Ultra Hafif:** Chromium tabanlı hantal Electron uygulamalarının aksine, **Tauri v2** mimarisi sayesinde kurulum dosyası sadece **~2 MB** boyutundadır.
- 🧠 **Akıllı Manifest Yönetimi:** Klasördeki dosyaları analiz ederek SCORM 1.2 veya SCORM 2004 standartlarında kusursuz `imsmanifest.xml` dosyasını otomatik üretir. Proje güncellemelerinde kullanıcıyı yormadan arka planda manifesti sessizce günceller.
- 📂 **Yerel İşletim Sistemi Entegrasyonu:** Paketleme bittiğinde tek tıkla Windows dosya gezginini açarak oluşturulan ZIP dosyasına anında erişim sağlar.

### 📥 Kurulum & Kullanım
1. [Releases](https://github.com/muallimun/muallimun-scorm-builder/releases/latest) sayfasından en güncel `scorm-masaustu_x64-setup.exe` dosyasını indirin ve kurun.
2. Programı açın, SCORM sürümünü (EBA için 1.2 önerilir) ve proje başlığınızı belirleyin.
3. **"Klasörü Seç ve Paketle"** butonuna basın. Gerisini Rust motoru halledecektir!

### 🌐 Çevrim İçi (Web) Sürümü
Uygulamayı bilgisayarınıza kurmak istemiyorsanız veya farklı bir işletim sistemi kullanıyorsanız, hibrit mimarimiz sayesinde tarayıcı üzerinden anında paketleme yapabilirsiniz:
👉 **[Hemen Kullan: Çevrim İçi SCORM Paketleyici](https://onlinearapca.muallimun.net/scorm/)**

* **Kurulumsuz Kullanım:** Sadece klasörünüzü seçin ve web tarayıcınız paketlemeyi yapsın.
* **Akıllı Kota Kontrolü:** EBA ve Moodle gibi sistemlerin 250 MB sınırına yaklaştığınızda sizi otomatik olarak uyarır.
* *Not: Web API kısıtlamaları nedeniyle çevrim içi sürümde `.zip` dosyasını tarayıcının indirme yönergelerine göre manuel olarak indirmeniz/onaylamanız gerekebilir.*
---

## 🇬🇧 English

**Muallimun SCORM Builder** is a fully compliant, ultra-lightweight, next-generation desktop e-learning content packaging software developed for all popular LMS (Learning Management System) infrastructures.

Unlike legacy web tools that hit browser RAM and storage limits, it is powered by the **Rust** programming language and processes even massive projects in seconds.

### ✨ Premium Features

- 🚀 **Lightning-Fast ZIP Engine:** Thanks to its Rust-based core, it packages HTML5 contents consisting of thousands of files into `.zip` format in seconds without OS fatigue and zero RAM bottlenecks.
- 🪶 **Ultra Lightweight:** Unlike bulky Chromium-based Electron apps, the installer is only **~2 MB** in size, courtesy of the **Tauri v2** architecture.
- 🧠 **Smart Manifest Management:** Analyzes directory files to automatically generate flawless `imsmanifest.xml` files in SCORM 1.2 or SCORM 2004 standards. It silently updates the manifest in the background during project iterations without annoying prompts.
- 📂 **Native OS Integration:** Instantly access your generated ZIP file with a single click that opens the native Windows file explorer upon completion.

### 📥 Installation & Usage
1. Download and install the latest `scorm-masaustu_x64-setup.exe` from the [Releases](https://github.com/muallimun/muallimun-scorm-builder/releases/latest) page.
2. Open the app, select the SCORM version (1.2 is recommended), and set your project title.
3. Click the **"Select Folder & Package"** button. The Rust engine will handle the rest!

### 🌐 Online (Web) Edition
If you prefer not to install any software or are using a different operating system, you can use our browser-based hybrid version instantly:
👉 **[Launch Online SCORM Builder](https://onlinearapca.muallimun.net/scorm/)**

* **Zero Installation:** Just select your folder and let your browser handle the packaging.
* **Smart Quota Checker:** Automatically warns you if your package exceeds the standard 250 MB limit for platforms like Moodle.
* *Note: Due to Web API restrictions, the online version requires you to manually confirm/download the final `.zip` file via your browser.*
---

<p align="center">
  Geliştirici: <a href="https://muallimun.net">Muallimun.Net</a> Ar-Ge Ekibi <br>
  <em>Powered by Tauri & Rust</em>
</p>