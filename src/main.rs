mod modules;

use regex::Regex;
use std::io;
use modules::user::User;
use modules::tukang_ledeng::TukangLedeng;
use modules::pesanan::Pesanan;
use modules::cari_tukang_ledeng::CariTukangLedeng;
use modules::enums::metode_pembayaran::MetodePembayaran;
use modules::enums::status_pembayaran::StatusPembayaran;

fn is_email_valid(email: &String) -> bool {
    let regex = Regex::new(r".+@.+\..+").unwrap();
    return regex.is_match(email);
} 

fn is_password_valid(password: &String) -> bool {
    return password.len() >= 8;
}

fn main() {
    let mut daftar_user: Vec<User>;
    let mut daftar_tukang_ledeng: Vec<TukangLedeng>;
    let mut daftar_pesanan: Vec<Pesanan>;
    
    let mut opsi = String::new();

    println!("Selamat Datang di Sistem Penyewaan Jasa Tukang Ledeng\n");

    'main_loop: loop {
        println!("Daftar opsi tersedia: \n1. Masuk\n2. Daftar\n3. Keluar\n");
        println!("Masukkan opsi: ");
        opsi.clear();
        io::stdin().read_line(&mut opsi).expect("Gagal membaca input");

        let mut opsi_input: i32 = opsi.trim().parse().expect("Opsi harus angka");
        println!("");
        if opsi_input == 1 {
            'masuk_loop: loop {
                println!("Masuk sebagai: \n1. Pengguna\n2. Tukang Ledeng\n3. Kembali\n");
                println!("Masukkan opsi: ");
                opsi.clear();
                io::stdin().read_line(&mut opsi).expect("Gagal membaca input");

                opsi_input = opsi.trim().parse().expect("Opsi harus angka");
                println!("");
                if opsi_input == 1 {
                    let mut email = String::new();
                    let mut password = String::new();

                    println!("Masukkan data: ");
                    println!("Email: ");
                    io::stdin().read_line(&mut email).expect("Gagal membaca input");
                    println!("Password: ");
                    io::stdin().read_line(&mut password).expect("Gagal membaca input");

                    email = String::from(email.trim());
                    password = String::from(password.trim());
                    println!("");
                    if is_email_valid(&email) && is_password_valid(&password) {

                    }
                    else {
                        println!("Email atau password tidak valid");
                        continue 'masuk_loop;
                    }

                    email.clear();
                    password.clear();
                }
                else if opsi_input == 2 {
                    
                }
                else if opsi_input == 3 {
                    continue 'main_loop; 
                }
                else {
                    println!("Opsi '{}' tidak tersedia", opsi_input);
                }
            }
        }
        else if opsi_input == 2 {
            'daftar_loop: loop {
                println!("Daftar sebagai: \n1. Pengguna\n2. Tukang Ledeng\n3. Kembali\n");
                println!("Masukkan opsi: ");
                opsi.clear();
                io::stdin().read_line(&mut opsi).expect("Gagal membaca input");

                opsi_input = opsi.trim().parse().expect("Opsi harus angka");
                println!("");
                if opsi_input == 1 {

                }
                else if opsi_input == 2 {
                    
                }
                else if  opsi_input == 3 {
                    continue 'main_loop; 
                }
                else {
                    println!("Opsi '{}' tidak tersedia", opsi_input);
                }
            }
        }
        else if opsi_input == 3 {
            println!("Bye");
            break;
        }
        else {
            println!("Opsi '{}' tidak tersedia", opsi_input);
        }
    }
}
