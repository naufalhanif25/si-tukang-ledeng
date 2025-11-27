# Sistem Informasi Penyewaan Jasa Tukang Ledeng

Aplikasi ini merupakan sistem penyewaan jasa tukang ledeng berbasis **Command-Line Interface (CLI)** yang dibangun menggunakan bahasa pemrograman **Rust**. Program ini menyediakan fitur untuk:

* Mengelola **User**
* Mengelola **Tukang Ledeng**
* Membuat dan memodifikasi status **Pesanan**
* Melakukan pencarian dan filter tukang ledeng
* Mengatur dan memperbarui status pembayaran

Proyek ini juga menerapkan **tiga Design Pattern penting** untuk menjaga struktur program tetap bersih, fleksibel, dan maintainable.

---

## Fitur Utama

### **Manajemen User**

* Registrasi dan login user
* Melihat profile pengguna

### **Manajemen Tukang Ledeng**

* Registrasi dan login tukang ledeng
* Melihat daftar pesanan yang masuk
* Melihat profile tukang ledeng

### **Manajemen Pesanan**

* Membuat pesanan ke tukang ledeng tertentu
* Update status pembayaran pesanan

### **Pencarian Tukang Ledeng**

* Cari berdasarkan nama atau lokasi
* Filter berdasarkan kategori

---

## Design Pattern yang Diimplementasikan

Proyek ini menggunakan **tiga design pattern** untuk meningkatkan skalabilitas dan maintainability aplikasi.

### **Builder Pattern**

Design pattern ini diimplementasikan pada class:

* `UserBuilder` → membangun instance `User`
* `TukangLedengBuilder` → membangun instance `TukangLedeng`

#### **Tujuan:**

Builder Pattern digunakan untuk membuat object kompleks secara bertahap, terutama ketika jumlah parameter pada constructor sangat banyak dan rawan error.

#### **Manfaat:**

* Menghindari constructor panjang
* Parameter lebih jelas dan aman
* Mudah dikembangkan

### **State Pattern**

Design pattern ini diimplementasikan pada class:

* `StatusPembayaran` → diintegrasikan melalui state objects

#### **Tujuan:**

State Pattern memisahkan logika perubahan status pembayaran menjadi objek yang berbeda-beda, sehingga setiap state memiliki perilaku unik seperti:

* `bayar()`
* `gagal()`
* `reset()`

#### **State yang diterapkan:**

* `PendingState`
* `BerhasilState`
* `GagalState`

#### **Manfaat:**

* Menghindari `match` panjang
* Memisahkan perilaku berdasarkan state
* Lebih mudah menambahkan state baru

### **Strategy Pattern**

Design pattern ini diimplementasikan pada class:

* `CariTukangLedeng`

#### **Tujuan:**

Memberikan fleksibilitas dalam memilih algoritma pencarian tukang ledeng secara dinamis. Hanya ada dua strategi:

* `CariStrategy` → mencari berdasarkan nama/lokasi
* `FilterStrategy` → memfilter berdasarkan kategori

#### **Manfaat:**

* Memisahkan logic pencarian dari class utama
* Memudahkan menambah strategi baru
* Menjaga kode tetap bersih dan SRP compliant

---

## Cara Menjalankan Program

Pastikan **Rust & Cargo** sudah terinstall. Jika belum, install melalui tautan berikut ini:
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### 1. Clone Repository

Clone repository GitHub dari project ini dengan menggunakan perintah berikut:

```bash
git clone https://github.com/naufalhanif25/si-tukang-ledeng.git
cd si-tukang-ledeng
```

### 2. Install Dependensi

Install semua dependensi yang dibutuhkan dengan menggunakan perintah berikut:

```bash
cargo build
```

### 3. Jalankan Program

Jalankan program dengan menggunakan **Cargo** seperti berikut:

```bash
cargo run
```

---

Aplikasi ini dirancang dengan struktur yang rapi dan scalable melalui penerapan tiga design pattern utama, sehingga mudah untuk dikembangkan di masa mendatang. Dengan arsitektur yang fleksibel dan modular, proyek ini dapat menjadi fondasi yang kuat untuk sistem yang profesional.