mod modules;

use std::io;
use crossterm::{ execute, terminal::{ Clear, ClearType }, cursor::MoveTo };
use modules::user::User;
use modules::tukang_ledeng::TukangLedeng;
use modules::pesanan::Pesanan;
use modules::utils;
use modules::app;

fn main() {
    let mut daftar_user: Vec<User> = utils::load_users_from_file("database/users.json");
    let mut daftar_tukang_ledeng: Vec<TukangLedeng> = utils::load_tukang_from_file("database/tukang_ledeng.json");
    let mut daftar_pesanan: Vec<Pesanan> = utils::load_pesanan_from_file("database/pesanan.json");

    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    app::main::main_menu(&mut daftar_user, &mut daftar_tukang_ledeng, &mut daftar_pesanan);
    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

    utils::save_users_to_file(&daftar_user, "database/users.json").unwrap();
    utils::save_tukang_to_file(&daftar_tukang_ledeng, "database/tukang_ledeng.json").unwrap();
    utils::save_pesanan_to_file(&daftar_pesanan, "database/pesanan.json").unwrap();
}
