use crate::modules::utils;
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;

pub fn show_daftar_tukang(daftar_tukang_ledeng: &Vec<&TukangLedeng>) {
    if daftar_tukang_ledeng.is_empty() {
        utils::print_for_seconds("Tidak ada data tukang ledeng yang tersedia", 1);
        return;
    }
    println!("Hasil pencarian tukang ledeng: \n");

    for (index, item) in daftar_tukang_ledeng.iter().enumerate() {
        println!("{}. {}", index + 1, item.nama);
        println!("   Spesialisasi : {}", item.kategori.as_string());
        println!("   Lokasi       : {}", item.get_lokasi());
        println!("   Tarif        : Rp {}", item.get_tarif());
        println!("   Email        : {}", item.get_email());
        println!("   Rekening     : {} ({})\n", item.get_rekening(), item.get_rekening_type());
    }

    println!("Total {} tukang ledeng ditemukan\n", daftar_tukang_ledeng.len());
}

pub fn show_daftar_pesanan(daftar_pesanan: &Vec<&Pesanan>) {
    if daftar_pesanan.is_empty() {
        utils::print_for_seconds("Tidak ada data pesanan yang tersedia", 1);
        return;
    }
    println!("Hasil pencarian pesanan: ");

    for (index, item) in daftar_pesanan.iter().enumerate() {
        println!("{}. {} ({})", index + 1, item.kategori.as_string(), item.get_nama());
        println!("   ID Pesanan   : {}", item.get_id());
        println!("   Pemesan      : {}", item.user.get_nama());
        println!("   Lokasi       : {}", item.get_lokasi());
        println!("   Tarif        : Rp {}", item.get_tarif());
        println!("   Jadwal       : {}", item.get_jadwal());
        println!("   Layanan      : {}", item.get_layanan().as_string());
        println!("   Status       : {}", item.get_status().as_string());
        println!("   Rekening     : {} ({})\n", item.get_rekening(), item.get_rekening_type());
    }

    println!("Total {} pesanan ditemukan\n", daftar_pesanan.len());
}

pub fn show_profile_user(user: &User) {
    println!("Profil pengguna: ");
    println!("ID            : {}", user.get_id());
    println!("Nama          : {}", user.get_nama());
    println!("Email         : {}\n", user.get_email());
}

pub fn show_profile_tukang(tukang_ledeng: &TukangLedeng) {
    println!("Profil tukang ledeng: ");
    println!("ID                : {}", tukang_ledeng.get_id());
    println!("Nama              : {}", tukang_ledeng.get_nama());
    println!("Email             : {}", tukang_ledeng.get_email());
    println!("Tarif             : Rp {}", tukang_ledeng.get_tarif());
    println!("Kategori          : {}", tukang_ledeng.get_kategori().as_string());
    println!("Lokasi            : {}", tukang_ledeng.get_lokasi());
    println!("No. Rekening      : {}", tukang_ledeng.get_rekening());
    println!("Metode Pembayaran : {}\n", tukang_ledeng.get_rekening_type());
}