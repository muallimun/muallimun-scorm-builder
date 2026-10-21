# Muallimun SCORM Paketleyici

EBA (Eğitim Bilişim Ağı) ve Moodle gibi LMS (Öğrenme Yönetim Sistemleri) platformları için geliştirilmiş, HTML/JS/CSS tabanlı e-eğitim içeriklerini otomatik olarak SCORM 1.2 veya SCORM 2004 standartlarında paketleyen açık kaynaklı bir araçtır.

🌐 **[Çevrim İçi Versiyonu Kullanmak İçin Tıklayın](https://onlinearapca.muallimun.net/scorm/)**

## ✨ Özellikler

* **Otomatik Manifest Üretimi:** Seçilen dizini özyinelemeli (recursive) tarayarak `imsmanifest.xml` dosyasını standartlara uygun şekilde saniyeler içinde oluşturur.
* **LMS Uyumluluk Filtresi (Sanitizer):** Dosya ve klasör isimlerindeki Türkçe karakterleri (ı, ğ, ü, ş, ö, ç) ve boşlukları tespit ederek LMS sistemlerinde yaşanabilecek "Beyaz Ekran" hatalarını önceden engeller.
* **Kota Kontrolü:** E-içerik klasör boyutunu analiz eder ve 250 MB'ı aşan durumlarda EBA/Moodle yükleme sınırları için uyarı verir.
* **Sürüm Esnekliği:** SCORM 1.2 ve SCORM 2004 (4th Edition) standartları arasında geçiş imkanı sunar.
* **Sıfır Sunucu Maliyeti:** Çevrim içi versiyon `File System Access API` kullandığı için tüm okuma/yazma işlemleri kullanıcının yerel tarayıcısında (RAM şişirmeden) gerçekleşir.
* **Bağımsız Masaüstü Uygulaması:** Rust ve Tauri mimarisiyle geliştirilmiş, kurulumsuz (portable) çalışabilen son derece hafif masaüstü versiyonu mevcuttur.

## 🚀 Masaüstü Sürümünü Geliştirme ve Derleme

Bu proje Tauri çerçevesi kullanılarak geliştirilmiştir. Kendi bilgisayarınızda derlemek için:

**Gereksinimler:**
- Node.js
- Rust (rustup & cargo)
- Microsoft Visual Studio C++ Build Tools (Windows için)

**Kurulum Adımları:**
1. Depoyu bilgisayarınıza klonlayın:
   ```bash
   git clone [https://github.com/KULLANICI_ADINIZ/scorm-paketleyici.git](https://github.com/KULLANICI_ADINIZ/scorm-paketleyici.git)