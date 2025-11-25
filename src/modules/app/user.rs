use std::io;
use chrono::NaiveDateTime;
use strum::IntoEnumIterator;
use crossterm::{ execute, terminal::{ Clear, ClearType }, cursor::MoveTo };
use crate::modules::cari_tukang_ledeng::CariTukangLedeng;
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::layanan::Layanan;
use crate::modules::utils;
use crate::modules::printer;

pub fn user_profile_menu<'a>(user: &'a mut User, width: &usize) {
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        printer::show_profile_user(user, width);

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

pub fn user_dashboard<'a>(user: &'a mut User, daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, daftar_pesanan: &'a mut Vec<Pesanan>, width: &usize) -> utils::MenuReturn {    
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        printer::print_title(vec![&format!("Selamat Datang, {}", user.get_nama()), "Dashboard Pengguna"], width, true);
        printer::menu_generator("Daftar opsi tersedia", vec!["Cari Tukang Ledeng", "Keranjang", "Profile", "Keluar"], width);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { 
                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                continue; 
            }
        };

        if opsi == 1 { cari_menu(user, daftar_tukang_ledeng, daftar_pesanan, width) }
        else if opsi == 2 { user_pesanan_menu(user, daftar_pesanan, width) }
        else if opsi == 3 { user_profile_menu(user, width) }
        else if opsi == 4 { 
            utils::save_tukang_to_file(&daftar_tukang_ledeng, "database/tukang_ledeng.json").unwrap();
            utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            return  utils::MenuReturn::Kembali 
        }
        else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", opsi)], 1, width, false); }
    }
}

pub fn cari_menu<'a>(user: &'a mut User, daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, daftar_pesanan: &'a mut Vec<Pesanan>, width: &usize) {
    let pencari = CariTukangLedeng::new(&daftar_tukang_ledeng);
    
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        printer::print_title(vec!["Menu Pencarian Tukang Ledeng"], width, false);
        printer::menu_generator("Daftar opsi tersedia", vec!["Cari", "Kembali"], width);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { 
                printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                continue; 
            }
        };

        if opsi == 1 {
            println!("Masukkan kata kunci (nama/lokasi): ");
            let kata_kunci = utils::console_read_line();
            let hasil_cari = pencari.cari(&kata_kunci);

            execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
            printer::show_daftar_tukang(&hasil_cari, width);
            printer::menu_generator("Daftar opsi tersedia", vec!["Pesan", "Kembali"], width);
            
            println!("Masukkan opsi: ");
            let opsi_pesan: i8 = match utils::console_read_line().parse::<i8>() {
                Ok(value) => value,
                Err(_) => {  
                    printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                    continue; 
                }
            };
            
            if opsi_pesan == 1 {
                println!("Pilih tukang ledeng (sesuai urutan): ");
                let urutan_tukang: i8 = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { 
                        printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                        continue; 
                    }
                };
                let current_tukang_ledeng = hasil_cari[urutan_tukang as usize - 1];
                
                println!("Masukkan lokasi (ex: Banda Aceh): ");
                let lokasi = utils::console_read_line();
                
                println!("Masukkan jadwal DD-MM-YYYY HH:MM (ex: 25-11-2025 14:00): ");
                let jadwal: NaiveDateTime = utils::to_naive_datetime(&utils::console_read_line());
                
                printer::menu_generator("Daftar layanan", Layanan::iter().map(|key| key.as_string()).collect(), width);
                
                println!("Pilih jenis layanan (sesuai urutan): ");
                let daftar_layanan: Vec<&str> = Layanan::iter().map(|key| key.as_string()).collect();
                let layanan_index: i8 = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => {  
                        printer::print_for_seconds(vec![&format!("Opsi harus berupa angka")], 1, width, false);
                        continue;
                    }
                };
                let current_layanan = match Layanan::from_string(&daftar_layanan[layanan_index as usize - 1]) {
                    Some(value) => value,
                    _ => { 
                        printer::print_for_seconds(vec![&format!("Opsi tidak valid")], 1, width, false); 
                        continue; 
                    }
                };
                let pesanan_baru = Pesanan::new(utils::generate_unique_id(), user.clone(), current_tukang_ledeng.clone(), &current_tukang_ledeng.get_nama(), *current_tukang_ledeng.get_tarif(), *current_tukang_ledeng.get_kategori(), &lokasi, *current_tukang_ledeng.get_rekening(), current_tukang_ledeng.get_rekening_type().to_string().clone(), jadwal, current_layanan);
                
                daftar_pesanan.push(pesanan_baru);
                printer::print_for_seconds(vec![&format!("Pesanan berhasil dibuat")], 1, width, false);
                
                utils::save_tukang_to_file(&daftar_tukang_ledeng, "database/tukang_ledeng.json").unwrap();
                utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            }
            else if opsi_pesan == 2 { continue }
            else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", opsi_pesan)], 1, width, false); }
        }
        else if  opsi == 2 { 
            utils::save_tukang_to_file(&daftar_tukang_ledeng, "database/tukang_ledeng.json").unwrap();
            utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            return 
        }
        else { printer::print_for_seconds(vec![&format!("Opsi '{}' tidak tersedia", opsi)], 1, width, false); }
    }
}

pub fn user_pesanan_menu<'a>(user: &'a mut User, daftar_pesanan: &'a mut Vec<Pesanan>, width: &usize) {
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        let daftar_pesanan_filter = utils::get_pesanan_by_user_id(user.get_id(), daftar_pesanan);
        printer::show_daftar_pesanan(&daftar_pesanan_filter, width);

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