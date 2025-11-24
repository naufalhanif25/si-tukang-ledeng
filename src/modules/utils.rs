use std::io;
use std::fs::File;
use std::io::Write;
use regex::Regex;
use rand::{ thread_rng, Rng, distributions::Alphanumeric };
use sha2::{ Sha256, Digest };
use chrono::{ NaiveDateTime, Local, Utc };
use serde::{ Serialize, Deserialize };
use bcrypt::{ hash, verify, DEFAULT_COST };
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::status_pembayaran::StatusPembayaran;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole { User, Tukang }
pub enum MenuReturn { Kembali, Lanjut }
pub enum Account { User(User), Tukang(TukangLedeng) }
pub enum AuthType { Register, Login }

pub fn is_email_valid(email: &String) -> bool { 
    return Regex::new(r".+@.+\..+").unwrap().is_match(email) 
}

pub fn is_password_valid(password: &String) -> bool { 
    return password.len() >= 8 
}

pub fn menu_generator(title: &str, menu: Vec<&str>) { 
    println!("{}: \n{}\n", title, menu.iter().enumerate().map(|(index, element)| format!("{}. {}", index + 1, element)).collect::<Vec<String>>().join("\n")) 
}

pub fn is_email_used(email: &str, daftar_user: &Vec<User>, daftar_tukang_ledeng: &Vec<TukangLedeng>) -> bool { 
    return daftar_user.iter().any(|u| u.get_email() == email) || daftar_tukang_ledeng.iter().any(|item| item.get_email() == email) 
}

pub fn get_user_by_email<'a>(email: &str, daftar_user: &'a mut Vec<User>) -> Option<&'a mut User> { 
    return daftar_user.iter_mut().find(|user| user.get_email() == email) 
}

pub fn get_tukang_by_email<'a>(email: &str, daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>) -> Option<&'a mut TukangLedeng> { 
    return daftar_tukang_ledeng.iter_mut().find(|tukang| tukang.get_email() == email) 
}

pub fn get_current_datetime() -> String {
    let now = Local::now();
    return now.format("%d-%m-%Y %H:%M").to_string();
}

pub fn to_naive_datetime(input: &str) -> NaiveDateTime { 
    return match NaiveDateTime::parse_from_str(input, "%d-%m-%Y %H:%M") {
        Ok(date_time) => date_time,
        Err(_) => { NaiveDateTime::parse_from_str(&get_current_datetime(), "%d-%m-%Y %H:%M").expect("Fallback datetime gagal — ini tidak seharusnya terjadi") }
    }
}

pub fn get_pesanan_by_user_id<'a>(user_id: &str, daftar_pesanan: &'a Vec<Pesanan>) -> Vec<&'a Pesanan> { 
    return daftar_pesanan.iter().filter(|data| data.user.get_id() == user_id).collect() 
}

pub fn get_pesanan_by_tukang_id<'a>(tukang_ledeng_id: &str, daftar_pesanan: &'a Vec<Pesanan>) -> Vec<&'a Pesanan> { 
    return daftar_pesanan.iter().filter(|data| data.tukang_ledeng.get_id() == tukang_ledeng_id).collect() 
}

pub fn filter_pesanan_id_by_tukang_id<'a>(tukang_id: &str, daftar: &'a Vec<Pesanan>) -> Vec<String> { 
    return daftar.iter().filter(|p| p.tukang_ledeng.get_id() == tukang_id).map(|p| p.id.clone()).collect() 
}

pub fn hash_password(password: &str) -> String { 
    return hash(password, DEFAULT_COST).expect("Gagal hash password") 
}

pub fn verify_password(password: &str, hashed: &str) -> bool { 
    return verify(password, hashed).unwrap_or(false) 
}

pub fn update_status_pesanan(id_pesanan: &str, daftar_pesanan: &mut Vec<Pesanan>, status_baru: StatusPembayaran) -> bool {
    for pesanan in daftar_pesanan.iter_mut() {
        if pesanan.get_id() == id_pesanan {
            pesanan.status = status_baru;
            return true;
        }
    }
    return false;
}

pub fn generate_unique_id() -> String {
    let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or_default().to_string();
    let random: String = thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();
    let mut hasher = Sha256::new();

    hasher.update(timestamp);
    hasher.update(random);

    let hash = hasher.finalize();

    format!("{:x}", hash)[..16].to_string()
}

pub fn user_data_verify(email: &String, password: &String, role: &UserRole, daftar_user: &Vec<User>, daftar_tukang_ledeng: &Vec<TukangLedeng>) -> bool {
    return match role {
        UserRole::User => { daftar_user.iter().any(|data| data.get_email() == email && verify_password(password, data.get_password())) }
        UserRole::Tukang => { daftar_tukang_ledeng.iter().any(|data| data.get_email() == email && verify_password(password, data.get_password())) }
    }
}

pub fn console_read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input\n");
    println!("");

    return input.trim().to_string();
}

pub fn save_users_to_file(users: &Vec<User>, filename: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(users).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn load_users_from_file(filename: &str) -> Vec<User> {
    let data = std::fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

pub fn save_tukang_to_file(tukang_ledeng: &Vec<TukangLedeng>, filename: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(tukang_ledeng).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn load_tukang_from_file(filename: &str) -> Vec<TukangLedeng> {
    let data = std::fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

pub fn save_pesanan_to_file(pesanan: &Vec<Pesanan>, filename: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(pesanan).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn load_pesanan_from_file(filename: &str) -> Vec<Pesanan> {
    let data = std::fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

pub fn show_daftar_tukang(daftar_tukang_ledeng: &Vec<&TukangLedeng>) {
    if daftar_tukang_ledeng.is_empty() {
        println!("Tidak ada data tukang ledeng yang tersedia\n");
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
        println!("Tidak ada data pesanan yang tersedia\n");
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

pub fn create_user<'a>(daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, daftar_user: &'a mut Vec<User>, account: Account, attemp_remaining: &mut i8) -> MenuReturn {
    let mut result = true;

    match account {
        Account::User(user) => {
            if is_email_used(user.get_email(), daftar_user, daftar_tukang_ledeng) { result = false } 
            else {
                daftar_user.push(user);
                println!("Pengguna berhasil dibuat\n");
            }
        }
        Account::Tukang(tukang) => {
            if is_email_used(tukang.get_email(), daftar_user, daftar_tukang_ledeng) { result = false } 
            else {
                daftar_tukang_ledeng.push(tukang);
                println!("Tukang Ledeng berhasil dibuat\n");
            }
        }
    }

    if result { 
        save_users_to_file(&daftar_user, "database/users.json").unwrap();
        save_tukang_to_file(&daftar_tukang_ledeng, "database/tukang_ledeng.json").unwrap();
        return MenuReturn::Lanjut;
    }
    else {
        *attemp_remaining -= 1;
        println!("Email sudah digunakan ({} kesempatan tersisa)\n", attemp_remaining);
        return MenuReturn::Kembali;
    }
}