use std::io;
use strum::IntoEnumIterator;
use crossterm::{ execute, terminal::{ Clear, ClearType }, cursor::MoveTo };
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::kategori::Kategori;
use crate::modules::enums::metode_pembayaran::MetodePembayaran;
use crate::modules::printer;
use crate::modules::utils;
use crate::modules::app::user;
use crate::modules::app::tukang_ledeng;

pub fn auth_menu<'a>(daftar_user: &'a mut Vec<User>, daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, daftar_pesanan: &'a mut Vec<Pesanan>, role: &utils::UserRole, auth_type: &utils::AuthType, width: &usize) -> utils::MenuReturn {
    let mut email = String::new();
    let mut password = String::new();
    let mut nama = String::new();
    let mut tarif = 0.0;
    let mut kategori: Kategori = Kategori::DomestikRumahTangga;
    let mut lokasi = String::new();
    let mut rekening: i64 = 0;
    let mut rekening_type: MetodePembayaran = MetodePembayaran::TfBsi;
    let mut attemp_remaining = 3;

    'auth_loop: loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        printer::print_title(vec!["Menu Autentikasi"], width, false);
        println!("Masukkan email (ex: john.doe@gmail.com): ");
        email = utils::console_read_line();

        println!("Masukkan password (min. 8 karakter): ");
        password = utils::console_read_line();

        match auth_type {
            utils::AuthType::Login => {}
            utils::AuthType::Register => {
                println!("Masukkan nama (ex: John Doe): ");
                nama = utils::console_read_line();

                match role {
                    utils::UserRole::User => {}
                    utils::UserRole::Tukang => {
                        println!("Masukkan tarif per Jam (ex: 50000): ");
                        tarif = match utils::console_read_line().parse::<f32>() {
                            Ok(value) => value,
                            Err(_) => {  
                                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                                continue 'auth_loop; 
                            }
                        };
                        
                        let daftar_kategori: Vec<&str> = Kategori::iter().map(|key| key.as_string()).collect();
                        printer::menu_generator("Daftar kategori", daftar_kategori.clone(), width);
                        
                        println!("Pilih kategori (sesuai urutan): ");
                        let kategori_index: i8 = match utils::console_read_line().parse::<i8>() {
                            Ok(value) => value,
                            Err(_) => {  
                                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                                continue 'auth_loop; 
                            }
                        };
                        kategori = match Kategori::from_string(&daftar_kategori[kategori_index as usize - 1]) {
                            Some(value) => value,
                            _ => { 
                                printer::print_for_seconds(vec![&format!("Opsi tidak valid")], 1, width, false); 
                                continue 'auth_loop; 
                            }
                        };
                        
                        println!("Masukkan lokasi (ex: Banda Aceh): ");
                        lokasi = utils::console_read_line();
                        
                        println!("Masukkan no. rekening (ex: 112233445566): ");
                        rekening = match utils::console_read_line().parse::<i64>() {
                            Ok(value) => value,
                            Err(_) => { 
                                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                                continue 'auth_loop; 
                            }
                        };
                        
                        let daftar_rekening_type: Vec<&str> = MetodePembayaran::iter().map(|key| key.as_string()).collect();
                        printer::menu_generator("Daftar metode pembayaran", daftar_rekening_type.clone(), width);
                        
                        println!("Pilih metode pembayaran (sesuai urutan): ");
                        let rekening_index: i8 = match utils::console_read_line().parse::<i8>() {
                            Ok(value) => value,
                            Err(_) => { 
                                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                                continue 'auth_loop; 
                            }
                        };
                        rekening_type = match MetodePembayaran::from_string(&daftar_rekening_type[rekening_index as usize - 1]) {
                            Some(value) => value,
                            _ => { 
                                printer::print_for_seconds(vec![&format!("Opsi tidak valid")], 1, width, false); 
                                continue 'auth_loop; 
                            }
                        };
                    }
                }
            }
        }
        if utils::is_email_valid(&email) && utils::is_password_valid(&password) {
            match auth_type {
                utils::AuthType::Login => { 
                    if utils::user_data_verify(&email, &password, role, &daftar_user, &daftar_tukang_ledeng) { 
                        match role {
                            utils::UserRole::Tukang => { 
                                if let Some(current_tukang_ledeng) = utils::get_tukang_by_email(&email, daftar_tukang_ledeng) { 
                                    match tukang_ledeng::tukang_ledeng_dashboard(current_tukang_ledeng, daftar_user, daftar_pesanan, width) {
                                        utils::MenuReturn::Kembali => { return utils::MenuReturn::Lanjut }
                                        _ => {}
                                    }
                                } 
                                else { println!("Tukang Ledeng tidak ditemukan") }
                            }
                            utils::UserRole::User => { 
                                if let Some(current_user) = utils::get_user_by_email(&email, daftar_user) {
                                    match user::user_dashboard(current_user, daftar_tukang_ledeng, daftar_pesanan, width) {
                                        utils::MenuReturn::Kembali => { return utils::MenuReturn::Lanjut }
                                        _ => {}
                                    }
                                } 
                                else { println!("User tidak ditemukan") }
                            }
                        }
                    }
                    else {
                        attemp_remaining -= 1;
                        printer::print_for_seconds(vec![&format!("Email atau password salah ({} kesempatan tersisa)", attemp_remaining)], 1, width, false);
                        continue 'auth_loop;
                    }
                }
                utils::AuthType::Register => { 
                    let create_user_result: utils::MenuReturn;
                    match role {
                        utils::UserRole::Tukang => { create_user_result = utils::create_user(daftar_tukang_ledeng, daftar_user, utils::Account::Tukang(TukangLedeng::new(utils::generate_unique_id(), &nama.clone(), &email.clone(), &utils::hash_password(&password.clone()), tarif, kategori, &lokasi.clone(), rekening, rekening_type.as_string().to_string().clone())), &mut attemp_remaining, width) }
                        utils::UserRole::User => { create_user_result = utils::create_user(daftar_tukang_ledeng, daftar_user, utils::Account::User(User::new(utils::generate_unique_id(), &nama.clone(), &email.clone(), &utils::hash_password(&password.clone()))), &mut attemp_remaining, width) }
                    }
                    match create_user_result {
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                        utils::MenuReturn::Lanjut => { return utils::MenuReturn::Lanjut }
                    }
                }
            }
        }
        else { 
            attemp_remaining -= 1;
            printer::print_for_seconds(vec![&format!("Email atau password tidak valid ({} kesempatan tersisa)", attemp_remaining)], 1, width, false);
        }
        if attemp_remaining == 0 { return utils::MenuReturn::Kembali }
    }
}