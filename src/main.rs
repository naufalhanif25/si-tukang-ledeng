mod modules;

use std::io;
use modules::user::User;
use modules::tukang_ledeng::TukangLedeng;
use modules::pesanan::Pesanan;
use modules::cari_tukang_ledeng::CariTukangLedeng;
use modules::utils;

fn main() {
    let mut daftar_user: Vec<User> = Vec::new();
    let mut daftar_tukang_ledeng: Vec<TukangLedeng> = Vec::new();
    let mut daftar_pesanan: Vec<Pesanan> = Vec::new();
    let mut opsi = String::new();

    println!("Selamat Datang di Sistem Penyewaan Jasa Tukang Ledeng\n");
    'dashboard_loop: loop {
        utils::menu_generator("Daftar opsi tersedia", vec!["Masuk", "Daftar", "Keluar"]);
        println!("Masukkan opsi: ");
        opsi.clear();
        io::stdin().read_line(&mut opsi).expect("Gagal membaca input\n");
        let mut opsi_input: i32 = opsi.trim().parse().expect("Opsi harus angka\n");
        utils::generate_newline(1);

        if opsi_input == 1 {
            'auth_loop: loop {
                utils::menu_generator("Masuk sebagai", vec!["Pengguna", "Tukang Ledeng", "Kembali"]);
                println!("Masukkan opsi: ");
                opsi.clear();
                io::stdin().read_line(&mut opsi).expect("Gagal membaca input\n");
                opsi_input = opsi.trim().parse().expect("Opsi harus angka\n");
                utils::generate_newline(1);

                if opsi_input == 1 {
                    match utils::auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &utils::UserRole::User, &utils::AuthType::Login) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi_input == 2 {
                    match utils::auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &utils::UserRole::Tukang, &utils::AuthType::Login) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi_input == 3 { continue 'dashboard_loop }
                else { println!("Opsi '{}' tidak tersedia\n", opsi_input) }
            }
        }
        else if opsi_input == 2 {
            'auth_loop: loop {
                utils::menu_generator("Daftar sebagai", vec!["Pengguna", "Tukang Ledeng", "Kembali"]);
                println!("Masukkan opsi: ");
                opsi.clear();
                io::stdin().read_line(&mut opsi).expect("Gagal membaca input\n");
                opsi_input = opsi.trim().parse().expect("Opsi harus angka\n");
                utils::generate_newline(1);

                if opsi_input == 1 {
                    match utils::auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &utils::UserRole::User, &utils::AuthType::Register) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi_input == 2 {
                    match utils::auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &utils::UserRole::Tukang, &utils::AuthType::Register) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if  opsi_input == 3 { continue 'dashboard_loop }
                else { println!("Opsi '{}' tidak tersedia\n", opsi_input) }
            }
        }
        else if opsi_input == 3 {
            println!("Bye");
            break;
        }
        else { println!("Opsi '{}' tidak tersedia\n", opsi_input) }
    }
}
