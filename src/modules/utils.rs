use std::io;
use regex::Regex;
use strum::IntoEnumIterator;
use rand::{thread_rng, Rng, distributions::Alphanumeric};
use sha2::{Sha256, Digest};
use chrono::Utc;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::kategori::Kategori;
use crate::modules::enums::layanan::Layanan;
use crate::modules::enums::status_pembayaran::StatusPembayaran;
use crate::modules::enums::metode_pembayaran::MetodePembayaran;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole { User, Tukang }
pub enum MenuReturn { Kembali, Lanjut }
pub enum Account { User(User), Tukang(TukangLedeng) }
pub enum AuthType { Register, Login }

pub fn is_email_valid(email: &String) -> bool { return Regex::new(r".+@.+\..+").unwrap().is_match(email) }
pub fn is_password_valid(password: &String) -> bool { return password.len() >= 8 }
pub fn generate_newline(newline_total: i8) { for _ in 0..newline_total { println!("") } }
pub fn menu_generator(title: &str, menu: Vec<&str>) { println!("{}: \n{}\n", title, menu.iter().enumerate().map(|(index, element)| format!("{}. {}", index + 1, element)).collect::<Vec<String>>().join("\n")) }
pub fn is_email_used(email: &str, daftar_user: &Vec<User>, daftar_tukang_ledeng: &Vec<TukangLedeng>) -> bool { return daftar_user.iter().any(|u| u.get_email() == email) || daftar_tukang_ledeng.iter().any(|t| t.get_email() == email) }

pub fn generate_unique_id() -> String {
    let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or_default().to_string();
    let random: String = thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();
    let mut hasher = Sha256::new();
    hasher.update(timestamp);
    hasher.update(random);
    let hash = hasher.finalize();
    format!("{:x}", hash)[..16].to_string()
}

pub fn user_data_checker(email: &String, password: &String, role: &UserRole, daftar_user: &Vec<User>, daftar_tukang_ledeng: &Vec<TukangLedeng>) -> bool {
    return match role {
        UserRole::User => { daftar_user.iter().any(|data| data.get_email() == email && data.get_password() == password) }
        UserRole::Tukang => { daftar_tukang_ledeng.iter().any(|data| data.get_email() == email && data.get_password() == password) }
    }
}

pub fn console_read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input\n");
    println!("");
    return input.trim().to_string();
}

pub fn auth_menu(daftar_user: &mut Vec<User>, daftar_tukang_ledeng: &mut Vec<TukangLedeng>, role: &UserRole, auth_type: &AuthType) -> MenuReturn {
    let mut email = String::new();
    let mut password = String::new();
    let mut name = String::new();
    let mut tarif = 0.0;
    let mut kategori: Kategori = Kategori::DomestikRumahTangga;
    let mut lokasi = String::new();
    let mut rekening: i32 = 0;
    let mut rekening_type: MetodePembayaran = MetodePembayaran::TfBsi;
    let mut attemp_remaining = 3;

    'auth_loop: loop {
        println!("Masukkan data: ");
        println!("Email (ex: john.doe@gmail.com): ");
        email = console_read_line();
        println!("Password (min. 8 karakter): ");
        password = console_read_line();
        match auth_type {
            AuthType::Login => { }
            AuthType::Register => {
                println!("Nama (ex: John Doe): ");
                name = console_read_line();
                match role {
                    UserRole::User => {}
                    UserRole::Tukang => {
                        println!("Tarif per Jam (ex: 50000): ");
                        tarif = console_read_line().parse().expect("Input tidak valid\n");
                        menu_generator("Daftar kategori: ", Kategori::iter().map(|key| key.as_string()).collect());
                        println!("Kategori (sesuai kategori yang tersedia): ");
                        kategori = Kategori::from_string(&console_read_line()).expect("Kategori tidak valid");
                        println!("Lokasi (ex: Jakarta): ");
                        lokasi = console_read_line();
                        println!("No. Rekening (ex: 1122334455667788): ");
                        rekening = console_read_line().parse().expect("Input tidak valid\n");
                        menu_generator("Daftar nama rekening: ", MetodePembayaran::iter().map(|key| key.as_string()).collect());
                        println!("Nama Rekening (sesuai metode yang tersedia): ");
                        rekening_type = MetodePembayaran::from_string(&console_read_line()).expect("Kategori tidak valid");
                    }
                }
            }
        }
        generate_newline(1);
        if is_email_valid(&email) && is_password_valid(&password) {
            match auth_type {
                AuthType::Login => { 
                    if user_data_checker(&email, &password, role, &daftar_user, &daftar_tukang_ledeng) { return MenuReturn::Lanjut }
                    else { continue 'auth_loop }
                }
                AuthType::Register => { 
                    let create_user_result: MenuReturn;
                    match role {
                        UserRole::Tukang => { create_user_result = create_user(daftar_tukang_ledeng, daftar_user, Account::Tukang(TukangLedeng::new(generate_unique_id(), &name, &email, &password, tarif, kategori, &lokasi, rekening, rekening_type)), &mut attemp_remaining) }
                        UserRole::User => { create_user_result = create_user(daftar_tukang_ledeng, daftar_user, Account::User(User::new(generate_unique_id(), &name, &email, &password)), &mut attemp_remaining) }
                    }
                    match create_user_result {
                        MenuReturn::Kembali => { continue 'auth_loop }
                        MenuReturn::Lanjut => { return MenuReturn::Lanjut }
                    }
                }
            }
        }
        else { 
            attemp_remaining -= 1;
            println!("Email atau password tidak valid ({} kesempatan tersisa)\n", attemp_remaining);
        }
        if attemp_remaining == 0 { return MenuReturn::Kembali }
    }
}

pub fn create_user(daftar_tukang_ledeng: &mut Vec<TukangLedeng>, daftar_user: &mut Vec<User>, account: Account, attemp_remaining: &mut i8) -> MenuReturn {
    let mut result = true;
    match account {
        Account::User(user) => {
            if is_email_used(user.get_email(), daftar_user, daftar_tukang_ledeng) { result = false } 
            else {
                daftar_user.push(user);
                println!("Pengguna berhasil dibuat");
            }
        }
        Account::Tukang(tukang) => {
            if is_email_used(tukang.get_email(), daftar_user, daftar_tukang_ledeng) { result = false } 
            else {
                daftar_tukang_ledeng.push(tukang);
                println!("Tukang Ledeng berhasil dibuat");
            }
        }
    }
    if result { return MenuReturn::Lanjut }
    else {
        *attemp_remaining -= 1;
        println!("Email sudah digunakan ({} kesempatan tersisa\n)", attemp_remaining);
        return MenuReturn::Kembali;
    }
}