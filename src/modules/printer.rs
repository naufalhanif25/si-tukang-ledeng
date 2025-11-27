use std::{ thread, time::Duration };
use crate::modules::colorize::ColorStyle;
use crate::modules::pesanan::Pesanan;
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::table::TableBorder;

pub fn show_daftar_tukang(daftar_tukang_ledeng: &Vec<&TukangLedeng>, width: &usize) {
    if daftar_tukang_ledeng.is_empty() {
        print_for_seconds(vec!["Tidak ada data tukang ledeng yang tersedia"], 1, width, false);
        return;
    }
    print_title(vec!["Hasil pencarian tukang ledeng"], width, false);

    for (index, item) in daftar_tukang_ledeng.iter().enumerate() {
        let border_line = TableBorder::get("H").repeat(width - 2);
        if index == 0 { println!("{}{}{}", TableBorder::get("TL"), border_line, TableBorder::get("TR")) }

        let info_lines = vec![
            format!("{}. {}", index + 1, item.nama),
            format!("   Spesialisasi : {}", item.kategori.as_string()),
            format!("   Lokasi       : {}", item.get_lokasi()),
            format!("   Tarif        : Rp {}", item.get_tarif()),
            format!("   Email        : {}", item.get_email()),
            format!("   Rekening     : {} ({})", item.get_rekening(), item.get_rekening_type())
        ];

        for line in info_lines {
            let padding = if line.len() + 4 > *width { 0 } else { width - line.len() - 4 };
            println!("{} {}{} {}", TableBorder::get("V"), line, " ".repeat(padding), TableBorder::get("V"));
        }
        if index == daftar_tukang_ledeng.len() - 1 { println!("{}{}{}", TableBorder::get("BL"), border_line, TableBorder::get("BR")) }
        else { println!("{}{}{}", TableBorder::get("ML"), border_line, TableBorder::get("MR")) }
    }
    println!("{}Total {} tukang ledeng ditemukan{}\n", ColorStyle::get("White"), daftar_tukang_ledeng.len(), ColorStyle::get("Reset"));
}

pub fn show_daftar_pesanan(daftar_pesanan: &Vec<&Pesanan>, width: &usize) {
    if daftar_pesanan.is_empty() {
        print_for_seconds(vec!["Tidak ada data pesanan yang tersedia"], 1, width, false);
        return;
    }
    print_title(vec!["Hasil pencarian pesanan"], width, false);

    for (index, item) in daftar_pesanan.iter().enumerate() {
        let border_line = TableBorder::get("H").repeat(width - 2);
        if index == 0 { println!("{}{}{}", TableBorder::get("TL"), border_line, TableBorder::get("TR")) }

        let info_lines = vec![
            format!("{}. {} ({})", index + 1, item.kategori.as_string(), item.get_nama()),
            format!("   ID Pesanan   : {}", item.get_id()),
            format!("   Pemesan      : {}", item.user.get_nama()),
            format!("   Lokasi       : {}", item.get_lokasi()),
            format!("   Tarif        : Rp {}", item.get_tarif()),
            format!("   Jadwal       : {}", item.get_jadwal()),
            format!("   Layanan      : {}", item.get_layanan().as_string()),
            format!("   Status       : {}", item.get_status().to_state().status().as_string()),
            format!("   Rekening     : {} ({})", item.get_rekening(), item.get_rekening_type())
        ];

        for line in info_lines {
            let padding = if line.len() + 4 > *width { 0 } else { width - line.len() - 4 };
            println!("{} {}{} {}", TableBorder::get("V"), line, " ".repeat(padding), TableBorder::get("V"));
        }
        if index == daftar_pesanan.len() - 1 { println!("{}{}{}", TableBorder::get("BL"), border_line, TableBorder::get("BR")) }
        else { println!("{}{}{}", TableBorder::get("ML"), border_line, TableBorder::get("MR")) }
    }
    println!("{}Total {} pesanan ditemukan{}\n", ColorStyle::get("White"), daftar_pesanan.len(), ColorStyle::get("Reset"));
}

pub fn show_profile_user(user: &User, width: &usize) {
    print_title(vec!["Profile pengguna"], width, false);
    
    let border_line = TableBorder::get("H").repeat(width - 2);
    println!("{}{}{}", TableBorder::get("TL"), border_line, TableBorder::get("TR"));

    let info_lines = vec![
        format!("ID            : {}", user.get_id()),
        format!("Nama          : {}", user.get_nama()),
        format!("Email         : {}", user.get_email())
    ];

    for line in info_lines {
        let padding = if line.len() + 4 > *width { 0 } else { width - line.len() - 4 };
        println!("{} {}{} {}", TableBorder::get("V"), line, " ".repeat(padding), TableBorder::get("V"));
    }
    println!("{}{}{}\n", TableBorder::get("BL"), border_line, TableBorder::get("BR"));
}

pub fn show_profile_tukang(tukang_ledeng: &TukangLedeng, width: &usize) {
    print_title(vec!["Profile Tukang Ledeng"], width, false);

    let border_line = TableBorder::get("H").repeat(width - 2);
    println!("{}{}{}", TableBorder::get("TL"), border_line, TableBorder::get("TR"));

    let info_lines = vec![
        format!("ID                : {}", tukang_ledeng.get_id()),
        format!("Nama              : {}", tukang_ledeng.get_nama()),
        format!("Email             : {}", tukang_ledeng.get_email()),
        format!("Tarif             : Rp {}", tukang_ledeng.get_tarif()),
        format!("Kategori          : {}", tukang_ledeng.get_kategori().as_string()),
        format!("Lokasi            : {}", tukang_ledeng.get_lokasi()),
        format!("No. Rekening      : {}", tukang_ledeng.get_rekening()),
        format!("Metode Pembayaran : {}", tukang_ledeng.get_rekening_type())
    ];

    for line in info_lines {
        let padding = if line.len() + 4 > *width { 0 } else { width - line.len() - 4 };
        println!("{} {}{} {}", TableBorder::get("V"), line, " ".repeat(padding), TableBorder::get("V"));
    }
    println!("{}{}{}\n", TableBorder::get("BL"), border_line, TableBorder::get("BR"));
}

pub fn print_for_seconds(message: Vec<&str>, seconds: u64, width: &usize, use_separator: bool) {
    print_title(message, width, use_separator);
    thread::sleep(Duration::from_secs(seconds));
}

pub fn print_title(title_lines: Vec<&str>, width: &usize, use_separator: bool) {
    let border_line = TableBorder::get("H").repeat(width - 2);
    
    if use_separator {
        for (index, line) in title_lines.iter().enumerate() {
            if index == 0 { println!("{}{}{}", TableBorder::get("TL"), border_line, TableBorder::get("TR")) }
            let padding = (width - line.len() - 2) / 2;
            let extra_space = (width - line.len() - 2) % 2;

            if index == 0 {
                println!("{}{}{}{}{}{}{}", TableBorder::get("V"), " ".repeat(padding), ColorStyle::get("Yellow"), line, ColorStyle::get("Reset"), " ".repeat(padding + extra_space), TableBorder::get("V"));
            }
            else {
                println!("{}{}{}{}{}", TableBorder::get("V")," ".repeat(padding), line, " ".repeat(padding + extra_space), TableBorder::get("V"));
            }

            if index < title_lines.len() - 1 { println!("{}{}{}", TableBorder::get("ML"), border_line, TableBorder::get("MR")) } 
            else { println!("{}{}{}\n", TableBorder::get("BL"), border_line, TableBorder::get("BR")) }
        }
    }
    else {
        println!("{}{}{}", TableBorder::get("TL"), border_line, TableBorder::get("TR"));

        for line in title_lines {
            let padding = (width - line.len() - 2) / 2;
            let extra_space = (width - line.len() - 2) % 2;
            
            println!("{}{}{}{}{}{}{}", TableBorder::get("V"), " ".repeat(padding), ColorStyle::get("Yellow"), line, ColorStyle::get("Reset"), " ".repeat(padding + extra_space), TableBorder::get("V"));
        }
        println!("{}{}{}\n", TableBorder::get("BL"), border_line, TableBorder::get("BR"));
    }
}

pub fn menu_generator(title: &str, menu: Vec<&str>, width: &usize) {
    let border_line = TableBorder::get("H").repeat(width - 2);
    let padding_title = (width - title.len() - 2) / 2;
    let extra_space = (width - title.len() - 2) % 2;
    let title_line = format!("{}{}{}{}{}{}{}", TableBorder::get("V"), " ".repeat(padding_title), ColorStyle::get("Yellow"), title, ColorStyle::get("Reset"), " ".repeat(padding_title + extra_space), TableBorder::get("V"));

    println!("{}{}{}", TableBorder::get("TL"), border_line, TableBorder::get("TR"));
    println!("{}", title_line);
    println!("{}{}{}", TableBorder::get("ML"), border_line, TableBorder::get("MR"));

    for (index, item) in menu.iter().enumerate() {
        let menu_text = format!("{}", item);
        let index_len = (index + 1).to_string().len();
        let padding_menu = width - menu_text.len() - (6 + index_len);
        
        println!("{} {}{}.{} {}{} {}", TableBorder::get("V"), ColorStyle::get("Yellow"), index + 1, ColorStyle::get("Reset"), menu_text, " ".repeat(padding_menu), TableBorder::get("V"));
    }
    println!("{}{}{}\n", TableBorder::get("BL"), border_line, TableBorder::get("BR"));
}