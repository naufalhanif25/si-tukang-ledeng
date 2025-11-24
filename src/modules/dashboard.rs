use chrono::NaiveDateTime;
use strum::IntoEnumIterator;
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::cari_tukang_ledeng::CariTukangLedeng;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::kategori::Kategori;
use crate::modules::enums::layanan::Layanan;
use crate::modules::enums::status_pembayaran::StatusPembayaran;
use crate::modules::enums::metode_pembayaran::MetodePembayaran;
use crate::modules::utils;

pub fn main_menu<'a>(mut daftar_user: &'a mut Vec<User>, mut daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, mut daftar_pesanan: &'a mut Vec<Pesanan>) {
    println!("Selamat Datang di Sistem Penyewaan Jasa Tukang Ledeng\n");

    'dashboard_loop: loop {
        utils::menu_generator("Daftar opsi tersedia", vec!["Masuk", "Daftar", "Keluar"]);
        println!("Masukkan opsi: ");
        let mut opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { println!("Opsi harus berupa angka\n"); continue 'dashboard_loop }
        };

        if opsi == 1 {
            'auth_loop: loop {
                utils::menu_generator("Masuk sebagai", vec!["Pengguna", "Tukang Ledeng", "Kembali"]);
                println!("Masukkan opsi: ");
                opsi = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { println!("Opsi harus berupa angka\n"); continue 'auth_loop }
                };

                if opsi == 1 {
                    match auth_menu(daftar_user, daftar_tukang_ledeng, daftar_pesanan, &utils::UserRole::User, &utils::AuthType::Login) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 2 {
                    match auth_menu(daftar_user, daftar_tukang_ledeng, daftar_pesanan, &utils::UserRole::Tukang, &utils::AuthType::Login) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 3 { continue 'dashboard_loop }
                else { println!("Opsi '{}' tidak tersedia\n", opsi) }
            }
        }
        else if opsi == 2 {
            'auth_loop: loop {
                utils::menu_generator("Daftar sebagai", vec!["Pengguna", "Tukang Ledeng", "Kembali"]);
                println!("Masukkan opsi: ");
                opsi = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { println!("Opsi harus berupa angka\n"); continue 'auth_loop }
                };

                if opsi == 1 {
                    match auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &mut daftar_pesanan, &utils::UserRole::User, &utils::AuthType::Register) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if opsi == 2 {
                    match auth_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &mut daftar_pesanan, &utils::UserRole::Tukang, &utils::AuthType::Register) {
                        utils::MenuReturn::Lanjut => { continue 'dashboard_loop }
                        utils::MenuReturn::Kembali => { continue 'auth_loop }
                    }
                }
                else if  opsi == 3 { continue 'dashboard_loop }
                else { println!("Opsi '{}' tidak tersedia\n", opsi) }
            }
        }
        else if opsi == 3 {
            println!("Bye");
            break;
        }
        else { println!("Opsi '{}' tidak tersedia\n", opsi) }
    }
}

pub fn auth_menu<'a>(daftar_user: &'a mut Vec<User>, daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, daftar_pesanan: &'a mut Vec<Pesanan>, role: &utils::UserRole, auth_type: &utils::AuthType) -> utils::MenuReturn {
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
        println!("Masukkan data: ");
        println!("Masukkan email (ex: john.doe@gmail.com): ");
        email = utils::console_read_line();

        println!("Masukkan password (min. 8 karakter): ");
        password = utils::console_read_line();

        match auth_type {
            utils::AuthType::Login => { }
            utils::AuthType::Register => {
                println!("Masukkan nama (ex: John Doe): ");
                nama = utils::console_read_line();

                match role {
                    utils::UserRole::User => {}
                    utils::UserRole::Tukang => {
                        println!("Masukkan tarif per Jam (ex: 50000): ");
                        tarif = match utils::console_read_line().parse::<f32>() {
                            Ok(value) => value,
                            Err(_) => { println!("Opsi harus berupa angka\n"); continue 'auth_loop }
                        };
                        
                        let daftar_kategori: Vec<&str> = Kategori::iter().map(|key| key.as_string()).collect();
                        utils::menu_generator("Daftar kategori", daftar_kategori.clone());
                        
                        println!("Pilih kategori (sesuai urutan): ");
                        let kategori_index: i8 = match utils::console_read_line().parse::<i8>() {
                            Ok(value) => value,
                            Err(_) => { println!("Opsi harus berupa angka\n"); continue 'auth_loop }
                        };
                        kategori = match Kategori::from_string(&daftar_kategori[kategori_index as usize - 1]) {
                            Some(value) => value,
                            _ => { println!("Opsi harus berupa angka\n"); continue 'auth_loop }
                        };
                        
                        println!("Masukkan lokasi (ex: Banda Aceh): ");
                        lokasi = utils::console_read_line();
                        
                        println!("Masukkan no. rekening (ex: 112233445566): ");
                        rekening = match utils::console_read_line().parse::<i64>() {
                            Ok(value) => value,
                            Err(_) => { println!("Opsi harus berupa angka\n"); continue 'auth_loop }
                        };
                        
                        let daftar_rekening_type: Vec<&str> = MetodePembayaran::iter().map(|key| key.as_string()).collect();
                        utils::menu_generator("Daftar metode pembayaran", daftar_rekening_type.clone());
                        
                        println!("Pilih metode pembayaran (sesuai urutan): ");
                        let rekening_index: i8 = match utils::console_read_line().parse::<i8>() {
                            Ok(value) => value,
                            Err(_) => { println!("Opsi harus berupa angka\n"); continue 'auth_loop }
                        };
                        rekening_type = match MetodePembayaran::from_string(&daftar_rekening_type[rekening_index as usize - 1]) {
                            Some(value) => value,
                            _ => { println!("Opsi harus berupa angka\n"); continue 'auth_loop }
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
                                    match tukang_ledeng_dashboard(current_tukang_ledeng, daftar_user, daftar_pesanan) {
                                        utils::MenuReturn::Kembali => { return utils::MenuReturn::Lanjut }
                                        _ => {}
                                    }
                                } 
                                else { println!("Tukang Ledeng tidak ditemukan") }
                            }
                            utils::UserRole::User => { 
                                if let Some(current_user) = utils::get_user_by_email(&email, daftar_user) {
                                    match user_dashboard(current_user, daftar_tukang_ledeng, daftar_pesanan) {
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
                        println!("Email atau password salah ({} kesempatan tersisa)\n", attemp_remaining);
                        continue 'auth_loop;
                    }
                }
                utils::AuthType::Register => { 
                    let create_user_result: utils::MenuReturn;
                    match role {
                        utils::UserRole::Tukang => { create_user_result = utils::create_user(daftar_tukang_ledeng, daftar_user, utils::Account::Tukang(TukangLedeng::new(utils::generate_unique_id(), &nama.clone(), &email.clone(), &utils::hash_password(&password.clone()), tarif, kategori, &lokasi.clone(), rekening, rekening_type.as_string().to_string().clone())), &mut attemp_remaining) }
                        utils::UserRole::User => { create_user_result = utils::create_user(daftar_tukang_ledeng, daftar_user, utils::Account::User(User::new(utils::generate_unique_id(), &nama.clone(), &email.clone(), &utils::hash_password(&password.clone()))), &mut attemp_remaining) }
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
            println!("Email atau password tidak valid ({} kesempatan tersisa)\n", attemp_remaining);
        }
        if attemp_remaining == 0 { return utils::MenuReturn::Kembali }
    }
}

pub fn user_dashboard<'a>(user: &'a mut User, daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, daftar_pesanan: &'a mut Vec<Pesanan>) -> utils::MenuReturn {
    println!("Selamat Datang, {}\n", user.get_nama());
    
    loop {
        utils::menu_generator("Daftar opsi tersedia", vec!["Cari Tukang Ledeng", "Keranjang", "Profile", "Keluar"]);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { println!("Opsi harus berupa angka\n"); continue }
        };

        if opsi == 1 { cari_menu(user, daftar_tukang_ledeng, daftar_pesanan) }
        else if opsi == 2 {
            let daftar_pesanan_filter = utils::get_pesanan_by_user_id(user.get_id(), daftar_pesanan);
            utils::show_daftar_pesanan(&daftar_pesanan_filter);
        }
        else if opsi == 3 { utils::show_profile_user(user) }
        else if opsi == 4 { 
            utils::save_tukang_to_file(&daftar_tukang_ledeng, "database/tukang_ledeng.json").unwrap();
            utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            return  utils::MenuReturn::Kembali 
        }
        else { println!("Opsi '{}' tidak tersedia\n", opsi) }
    }
}

pub fn tukang_ledeng_dashboard<'a>(tukang_ledeng: &'a mut TukangLedeng, daftar_user: &'a mut Vec<User>, daftar_pesanan: &'a mut Vec<Pesanan>) -> utils::MenuReturn {
    println!("Selamat Datang, {}\n", tukang_ledeng.get_nama());
    
    loop {
        utils::menu_generator("Daftar opsi tersedia", vec!["Pesanan", "Profile", "Keluar"]);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { println!("Opsi harus berupa angka\n"); continue }
        };

        if opsi == 1 { pesanan_menu(tukang_ledeng, daftar_pesanan) }
        else if opsi == 2 { utils::show_profile_tukang(tukang_ledeng) }
        else if opsi == 3 { 
            utils::save_users_to_file(&daftar_user, "database/users.json").unwrap();
            utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            return  utils::MenuReturn::Kembali 
        }
        else { println!("Opsi '{}' tidak tersedia\n", opsi) }
    }
}

pub fn pesanan_menu<'a>(tukang_ledeng: &'a mut TukangLedeng, daftar_pesanan: &'a mut Vec<Pesanan>) {
    let daftar_pesanan_filter = utils::get_pesanan_by_tukang_id(tukang_ledeng.get_id(), daftar_pesanan);
    utils::show_daftar_pesanan(&daftar_pesanan_filter);

    loop {
        utils::menu_generator("Daftar opsi tersedia", vec!["Konfirmasi", "Kembali"]);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { println!("Opsi harus berupa angka\n"); continue }
        };

        if opsi == 1 {
            println!("Pilih pesanan (sesuai urutan): ");
            let urutan_status: i8 = match utils::console_read_line().parse::<i8>() {
                Ok(value) => value,
                Err(_) => { println!("Opsi harus berupa angka\n"); continue }
            };
            let daftar_pesanan_id_filter = utils::filter_pesanan_id_by_tukang_id(tukang_ledeng.get_id(), daftar_pesanan);
            let current_pesanan_id = &daftar_pesanan_id_filter[urutan_status as usize - 1];
            
            let daftar_status: Vec<&str> = StatusPembayaran::iter().map(|key| key.as_string()).collect();
            utils::menu_generator("Daftar opsi konfirmasi tersedia", daftar_status.clone());
            
            println!("Pilih status (sesuai urutan): ");
            let opsi_status: i8 = match utils::console_read_line().parse::<i8>() {
                Ok(value) => value,
                Err(_) => { println!("Opsi harus berupa angka\n"); continue }
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
        else { println!("Opsi '{}' tidak tersedia\n", opsi) }
    }
}

pub fn cari_menu<'a>(user: &'a mut User, daftar_tukang_ledeng: &'a mut Vec<TukangLedeng>, daftar_pesanan: &'a mut Vec<Pesanan>) {
    let pencari = CariTukangLedeng::new(&daftar_tukang_ledeng);
    
    loop {
        utils::menu_generator("Daftar opsi tersedia", vec!["Cari", "Kembali"]);
        println!("Masukkan opsi: ");
        let opsi: i8 = match utils::console_read_line().parse::<i8>() {
            Ok(value) => value,
            Err(_) => { println!("Opsi harus berupa angka\n"); continue }
        };

        if opsi == 1 {
            println!("Masukkan kata kunci (nama/lokasi): ");
            let kata_kunci = utils::console_read_line();
            let hasil_cari = pencari.cari(&kata_kunci);

            utils::show_daftar_tukang(&hasil_cari);
            utils::menu_generator("Daftar opsi tersedia", vec!["Pesan", "Kembali"]);
            
            println!("Masukkan opsi: ");
            let opsi_pesan: i8 = match utils::console_read_line().parse::<i8>() {
                Ok(value) => value,
                Err(_) => { println!("Opsi harus berupa angka\n"); continue }
            };
            
            if opsi_pesan == 1 {
                println!("Pilih tukang ledeng (sesuai urutan): ");
                let urutan_tukang: i8 = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { println!("Opsi harus berupa angka\n"); continue }
                };
                let current_tukang_ledeng = hasil_cari[urutan_tukang as usize - 1];
                
                println!("Masukkan lokasi (ex: Banda Aceh): ");
                let lokasi = utils::console_read_line();
                
                println!("Masukkan jadwal DD-MM-YYYY HH:MM (ex: 25-11-2025 14:00): ");
                let jadwal: NaiveDateTime = utils::to_naive_datetime(&utils::console_read_line());
                
                utils::menu_generator("Daftar layanan", Layanan::iter().map(|key| key.as_string()).collect());
                
                println!("Pilih jenis layanan (sesuai urutan): ");
                let daftar_layanan: Vec<&str> = Layanan::iter().map(|key| key.as_string()).collect();
                let layanan_index: i8 = match utils::console_read_line().parse::<i8>() {
                    Ok(value) => value,
                    Err(_) => { println!("Opsi harus berupa angka\n"); continue }
                };
                let current_layanan = match Layanan::from_string(&daftar_layanan[layanan_index as usize - 1]) {
                    Some(value) => value,
                    _ => { println!("Opsi harus berupa angka\n"); continue }
                };
                let pesanan_baru = Pesanan::new(utils::generate_unique_id(), user.clone(), current_tukang_ledeng.clone(), &current_tukang_ledeng.get_nama(), *current_tukang_ledeng.get_tarif(), *current_tukang_ledeng.get_kategori(), &lokasi, *current_tukang_ledeng.get_rekening(), current_tukang_ledeng.get_rekening_type().to_string().clone(), jadwal, current_layanan);
                
                daftar_pesanan.push(pesanan_baru);
                println!("Pesanan berhasil dibuat\n");
                
                utils::save_tukang_to_file(&daftar_tukang_ledeng, "database/tukang_ledeng.json").unwrap();
                utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            }
            else if opsi_pesan == 2 { continue }
            else { println!("Opsi '{}' tidak tersedia\n", opsi_pesan) }
        }
        else if  opsi == 2 { 
            utils::save_tukang_to_file(&daftar_tukang_ledeng, "database/tukang_ledeng.json").unwrap();
            utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
            return 
        }
        else { println!("Opsi '{}' tidak tersedia\n", opsi) }
    }
}