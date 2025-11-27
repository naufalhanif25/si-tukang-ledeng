use std::io;
use strum::IntoEnumIterator;
use crossterm::{ execute, terminal::{ Clear, ClearType }, cursor::MoveTo };
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::status_pembayaran::StatusPembayaran;
use crate::modules::enums::status_state::PaymentState;
use crate::modules::utils;
use crate::modules::printer;

pub fn tukang_ledeng_profile_menu<'a>(tukang_ledeng: &'a mut TukangLedeng, width: &usize) {
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        printer::show_profile_tukang(tukang_ledeng, width); 
    
        printer::menu_generator("Daftar opsi tersedia", vec!["Kembali"], width);
        println!("Masukkan opsi: ");
        let profile_opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { 
                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                continue; 
            }
        };
    
        if profile_opsi == 1 { break }
        else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", profile_opsi)], 1, width, false); }
    }
}

pub fn tukang_ledeng_dashboard<'a>(tukang_ledeng: &'a mut TukangLedeng, daftar_user: &'a mut Vec<User>, daftar_pesanan: &'a mut Vec<Pesanan>, width: &usize) -> utils::MenuReturn {    
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        printer::print_title(vec![&format!("Selamat Datang, {}", tukang_ledeng.get_nama()), "Dashboard Tukang Ledeng"], width, true);
        printer::menu_generator("Daftar opsi tersedia", vec!["Pesanan", "Profile", "Keluar"], width);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { 
                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                continue; 
            }
        };

        if opsi == 1 { tukang_ledeng_pesanan_menu(tukang_ledeng, daftar_pesanan, width) }
        else if opsi == 2 { tukang_ledeng_profile_menu(tukang_ledeng, width) }
        else if opsi == 3 { 
            utils::save_users_to_file(&daftar_user, "database/users.json").unwrap();
            utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            return  utils::MenuReturn::Kembali 
        }
        else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", opsi)], 1, width, false); }
    }
}

pub fn tukang_ledeng_pesanan_menu<'a>(tukang_ledeng: &'a mut TukangLedeng, daftar_pesanan: &'a mut Vec<Pesanan>, width: &usize) {
    
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        let daftar_pesanan_filter = utils::get_pesanan_by_tukang_id(tukang_ledeng.get_id(), daftar_pesanan);
        printer::show_daftar_pesanan(&daftar_pesanan_filter, width);

        printer::menu_generator("Daftar opsi tersedia", vec!["Konfirmasi", "Kembali"], width);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => {  
                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                continue; 
            }
        };

        if opsi == 1 {
            println!("Pilih pesanan (sesuai urutan): ");
            let urutan_status: i8 = match utils::console_read_line().parse::<i8>() {
                Ok(value) => value,
                Err(_) => { 
                    printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                    continue; 
                }
            };
            let daftar_pesanan_id_filter = utils::filter_pesanan_id_by_tukang_id(tukang_ledeng.get_id(), daftar_pesanan);
            let mut current_index = (urutan_status as usize) - 1;
            match utils::range_err_handler(current_index, daftar_pesanan_id_filter.len(), width) {
                utils::MenuReturn::Kembali => { continue }
                utils::MenuReturn::Lanjut => {}
            }
            let current_pesanan_id = &daftar_pesanan_id_filter[current_index];
            
            let daftar_status: Vec<&str> = StatusPembayaran::iter().map(|key| key.as_string()).collect();
            printer::menu_generator("Daftar opsi konfirmasi tersedia", daftar_status.clone(), width);
            
            println!("Pilih status (sesuai urutan): ");
            let opsi_status: i8 = match utils::console_read_line().parse::<i8>() {
                Ok(value) => value,
                Err(_) => {  
                    printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                    continue; 
                }
            };
            current_index = (opsi_status as usize) - 1;
            match utils::range_err_handler(current_index, daftar_status.len(), width) {
                utils::MenuReturn::Kembali => { continue }
                utils::MenuReturn::Lanjut => {}
            }
            let current_status = daftar_status[current_index];
            let status_enum = StatusPembayaran::from_string(current_status);
            let state = status_enum.to_state();
            let new_state: Box<dyn PaymentState> = match status_enum {
                StatusPembayaran::Berhasil => state.bayar(),
                StatusPembayaran::Gagal => state.gagal(),
                StatusPembayaran::Pending => state.reset(),
            };
            let update_return = utils::update_status_pesanan(current_pesanan_id, daftar_pesanan, new_state.status());
            
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
        else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", opsi)], 1, width, false); }
    }
}