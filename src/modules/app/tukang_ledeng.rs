use std::io;
use strum::IntoEnumIterator;
use crossterm::{ execute, terminal::{ Clear, ClearType }, cursor::MoveTo };
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::status_pembayaran::StatusPembayaran;
use crate::modules::utils;
use crate::modules::printer;

pub fn tukang_ledeng_profile_menu<'a>(tukang_ledeng: &'a mut TukangLedeng) {
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        printer::show_profile_tukang(tukang_ledeng); 
    
        utils::menu_generator("Daftar opsi tersedia", vec!["Kembali"]);
        println!("Masukkan opsi: ");
        let profile_opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { 
                utils::print_for_seconds(&format!("Opsi harus berupa angka"), 1);
                continue; 
            }
        };
    
        if profile_opsi == 1 { break }
        else { utils::print_for_seconds(&format!("Opsi '{}' tidak tersedia", profile_opsi), 1); }
    }
}

pub fn tukang_ledeng_dashboard<'a>(tukang_ledeng: &'a mut TukangLedeng, daftar_user: &'a mut Vec<User>, daftar_pesanan: &'a mut Vec<Pesanan>) -> utils::MenuReturn {
    println!("Selamat Datang, {}\n", tukang_ledeng.get_nama());
    
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        utils::menu_generator("Daftar opsi tersedia", vec!["Pesanan", "Profile", "Keluar"]);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { 
                utils::print_for_seconds(&format!("Opsi harus berupa angka"), 1);
                continue; 
            }
        };

        if opsi == 1 { tukang_ledeng_pesanan_menu(tukang_ledeng, daftar_pesanan) }
        else if opsi == 2 { tukang_ledeng_profile_menu(tukang_ledeng) }
        else if opsi == 3 { 
            utils::save_users_to_file(&daftar_user, "database/users.json").unwrap();
            utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            return  utils::MenuReturn::Kembali 
        }
        else { utils::print_for_seconds(&format!("Opsi '{}' tidak tersedia", opsi), 1); }
    }
}

pub fn tukang_ledeng_pesanan_menu<'a>(tukang_ledeng: &'a mut TukangLedeng, daftar_pesanan: &'a mut Vec<Pesanan>) {
    
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        let daftar_pesanan_filter = utils::get_pesanan_by_tukang_id(tukang_ledeng.get_id(), daftar_pesanan);
        printer::show_daftar_pesanan(&daftar_pesanan_filter);

        utils::menu_generator("Daftar opsi tersedia", vec!["Konfirmasi", "Kembali"]);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => {  
                utils::print_for_seconds(&format!("Opsi harus berupa angka"), 1);
                continue; 
            }
        };

        if opsi == 1 {
            println!("Pilih pesanan (sesuai urutan): ");
            let urutan_status: i8 = match utils::console_read_line().parse::<i8>() {
                Ok(value) => value,
                Err(_) => { 
                    utils::print_for_seconds(&format!("Opsi harus berupa angka"), 1);
                    continue; 
                }
            };
            let daftar_pesanan_id_filter = utils::filter_pesanan_id_by_tukang_id(tukang_ledeng.get_id(), daftar_pesanan);
            let current_pesanan_id = &daftar_pesanan_id_filter[urutan_status as usize - 1];
            
            let daftar_status: Vec<&str> = StatusPembayaran::iter().map(|key| key.as_string()).collect();
            utils::menu_generator("Daftar opsi konfirmasi tersedia", daftar_status.clone());
            
            println!("Pilih status (sesuai urutan): ");
            let opsi_status: i8 = match utils::console_read_line().parse::<i8>() {
                Ok(value) => value,
                Err(_) => {  
                    utils::print_for_seconds(&format!("Opsi harus berupa angka"), 1);
                    continue; 
                }
            };
            let current_status = daftar_status[opsi_status as usize - 1];
            let update_return = utils::update_status_pesanan(current_pesanan_id, daftar_pesanan, StatusPembayaran::from_string(current_status));
            
            if update_return {
                println!("Status berhasil diperbarui");
                utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            } 
            else { println!("Status gagal diperbarui") }
        }
        else if opsi == 2 {
            utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            return 
        }
        else { utils::print_for_seconds(&format!("Opsi '{}' tidak tersedia", opsi), 1); }
    }
}