use std::io;
use crossterm::{ execute, terminal::{ Clear, ClearType }, cursor::MoveTo };
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::utils;
use crate::modules::app::auth;

pub fn main_menu<'a>(mut daftar_user: &'a mut Vec<User>, mut daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, mut daftar_pesanan: &'a mut Vec<Pesanan>) {
    println!("Selamat Datang di Sistem Penyewaan Jasa Tukang Ledeng\n");

    'dashboard_loop: loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        utils::menu_generator("Daftar opsi tersedia", vec!["Masuk", "Daftar", "Keluar"]);
        println!("Masukkan opsi: ");
        let mut opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { 
                utils::print_for_seconds(&format!("Opsi harus berupa angka"), 1);
                continue 'dashboard_loop;
            }
        };

        if opsi == 1 {
            'auth_loop: loop {
                execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
                utils::menu_generator("Masuk sebagai", vec!["Pengguna", "Tukang Ledeng", "Kembali"]);
                println!("Masukkan opsi: ");
                opsi = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { 
                        utils::print_for_seconds(&format!("Opsi harus berupa angka"), 1);
                        continue 'auth_loop; 
                    }
                };

                if opsi == 1 {
                    match auth::auth_menu(daftar_user, daftar_tukang_ledeng, daftar_pesanan, &utils::UserRole::User, &utils::AuthType::Login) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 2 {
                    match auth::auth_menu(daftar_user, daftar_tukang_ledeng, daftar_pesanan, &utils::UserRole::Tukang, &utils::AuthType::Login) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 3 { continue 'dashboard_loop }
                else { utils::print_for_seconds(&format!("Opsi '{}' tidak tersedia", opsi), 1); }
            }
        }
        else if opsi == 2 {
            'auth_loop: loop {
                execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
                utils::menu_generator("Daftar sebagai", vec!["Pengguna", "Tukang Ledeng", "Kembali"]);
                println!("Masukkan opsi: ");
                opsi = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { 
                        utils::print_for_seconds(&format!("Opsi harus berupa angka"), 1);
                        continue 'auth_loop; 
                    }
                };

                if opsi == 1 {
                    match auth::auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &mut daftar_pesanan, &utils::UserRole::User, &utils::AuthType::Register) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 2 {
                    match auth::auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &mut daftar_pesanan, &utils::UserRole::Tukang, &utils::AuthType::Register) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if  opsi == 3 { continue 'dashboard_loop }
                else { utils::print_for_seconds(&format!("Opsi '{}' tidak tersedia", opsi), 1); }
            }
        }
        else if opsi == 3 {
            execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
            utils::print_for_seconds("Bye", 1);
            break;
        }
        else { utils::print_for_seconds(&format!("Opsi '{}' tidak tersedia", opsi), 1); }
    }
}