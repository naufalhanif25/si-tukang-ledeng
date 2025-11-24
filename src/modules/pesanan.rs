use serde::{ Serialize, Deserialize };
use crate::modules::user::User;
use crate::modules::tukang_ledeng::TukangLedeng;
use crate::modules::enums::status_pembayaran::StatusPembayaran;
use crate::modules::enums::layanan::Layanan;
use crate::modules::enums::kategori::Kategori;
use chrono::{ NaiveDateTime, Local };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pesanan {
    pub id: String,
    pub user: User,
    pub tukang_ledeng: TukangLedeng,
    pub nama: String,
    pub tarif: f32,
    pub kategori: Kategori,
    pub lokasi: String,
    pub rekening: i64,
    pub rekening_type: String,
    pub status: StatusPembayaran,
    pub jadwal: NaiveDateTime,
    pub layanan: Layanan
}

impl Pesanan {
    pub fn new(id: String, user: User, tukang_ledeng: TukangLedeng, nama: &str, tarif: f32, kategori: Kategori, lokasi: &str, rekening: i64, rekening_type: String, jadwal: NaiveDateTime, layanan: Layanan) -> Self {
        return Self { id: id.clone(), user, tukang_ledeng, nama: nama.to_string(), tarif, kategori, lokasi: lokasi.to_string(), rekening, rekening_type: rekening_type.clone(), status: StatusPembayaran::Pending, jadwal, layanan }
    }

    pub fn get_id(&self) -> &str { 
        return &self.id 
    }

    pub fn set_nama(&mut self, nama: String) { 
        self.nama = nama 
    }

    pub fn get_nama(&self) -> &str { 
        return &self.nama 
    }

    pub fn set_tarif(&mut self, tarif: f32) { 
        self.tarif = tarif 
    }

    pub fn get_tarif(&self) -> &f32 { 
        return  &self.tarif 
    }

    pub fn set_kategori(&mut self, kategori: Kategori) { 
        self.kategori = kategori 
    }

    pub fn get_kategori(&self) -> &Kategori { 
        return  &self.kategori 
    }

    pub fn set_lokasi(&mut self, lokasi: String) { 
        self.lokasi = lokasi 
    }

    pub fn get_lokasi(&self) -> &str { 
        return  &self.lokasi 
    }

    pub fn set_rekening(&mut self, rekening: i64) { 
        self.rekening = rekening 
    }

    pub fn get_rekening(&self) -> &i64 { 
        return  &self.rekening 
    }

    pub fn set_rekening_type(&mut self, rekening_type: String) { 
        self.rekening_type = rekening_type 
    }

    pub fn get_rekening_type(&self) -> &str { 
        return &self.rekening_type 
    }

    pub fn set_status(&mut self, status: StatusPembayaran) { 
        self.status = status 
    }

    pub fn get_status(&self) -> &StatusPembayaran { 
        return &self.status 
    }

    pub fn set_jadwal(&mut self, jadwal: NaiveDateTime) { 
        self.jadwal = jadwal 
    }
    
    pub fn set_jadwal_now(&mut self) {
        let local = Local::now();
        self.jadwal = NaiveDateTime::new(local.date_naive(), local.time());
    }

    pub fn get_jadwal(&self) -> &NaiveDateTime { 
        return &self.jadwal 
    }

    pub fn set_layanan(&mut self, layanan: Layanan) { 
        self.layanan = layanan 
    }

    pub fn get_layanan(&self) -> &Layanan { 
        return &self.layanan 
    }
}
