use std::io;
use crossterm::{ execute, terminal::{ Clear, ClearType }, cursor::MoveTo };
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::printer;
use crate::modules::utils;
use crate::modules::app::auth;

pub fn main_menu<'a>(mut daftar_user: &'a mut Vec<User>, mut daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, mut daftar_pesanan: &'a mut Vec<Pesanan>, width: &usize) {
    'dashboard_loop: loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        printer::print_title(vec!["Sistem Penyewaan Jasa Tukang Ledeng", "Menu Utama"], width, true);
        printer::menu_generator("Daftar opsi tersedia", vec!["Masuk", "Daftar", "Keluar"], width);
        println!("Masukkan opsi: ");
        let mut opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { 
                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                continue 'dashboard_loop;
            }
        };

        if opsi == 1 {
            'auth_loop: loop {
                execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
                printer::print_title(vec!["Menu Masuk"], width, false);
                printer::menu_generator("Masuk sebagai", vec!["Pengguna", "Tukang Ledeng", "Kembali"], width);
                println!("Masukkan opsi: ");
                opsi = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { 
                        printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                        continue 'auth_loop; 
                    }
                };

                if opsi == 1 {
                    match auth::auth_menu(daftar_user, daftar_tukang_ledeng, daftar_pesanan, &utils::UserRole::User, &utils::AuthType::Login, width) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 2 {
                    match auth::auth_menu(daftar_user, daftar_tukang_ledeng, daftar_pesanan, &utils::UserRole::Tukang, &utils::AuthType::Login, width) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 3 { continue 'dashboard_loop }
                else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", opsi)], 1, width, false); }
            }
        }
        else if opsi == 2 {
            'auth_loop: loop {
                execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
                printer::print_title(vec!["Menu Daftar"], width, false);
                printer::menu_generator("Daftar sebagai", vec!["Pengguna", "Tukang Ledeng", "Kembali"], width);
                println!("Masukkan opsi: ");
                opsi = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { 
                        printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                        continue 'auth_loop; 
                    }
                };

                if opsi == 1 {
                    match auth::auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &mut daftar_pesanan, &utils::UserRole::User, &utils::AuthType::Register, width) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 2 {
                    match auth::auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &mut daftar_pesanan, &utils::UserRole::Tukang, &utils::AuthType::Register, width) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if  opsi == 3 { continue 'dashboard_loop }
                else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", opsi)], 1, width, false); }
            }
        }
        else if opsi == 3 {
            execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
            printer::print_for_seconds(vec!["Sampai Jumpa Kembali"], 1, width, false);
            break;
        }
        else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", opsi)], 1, width, false); }
    }
}