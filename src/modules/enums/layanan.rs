use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Layanan {
    InstalasiPipaBaru,
    PerbaikanKebocoranPipa,
    DeteksiKebocoran,
    PembersihanSaluranMampet,
    HydroJetting,
    PenggantianPipaLama,
    PenyambunganPipa,
    PengelasanPipa,
    PemasanganKelengkapanSanitasi,
    PemasanganFilterAir,
    PemasanganWaterHeater,
    PemasanganPompaAir,
    ServisPompaAir,
    PemasanganTangkiAir,
    PemasanganTorenBawahTanah,
    PerbaikanToilet,
    PerbaikanWastafel,
    PemasanganBathtubShower,
    PemasanganPemanasMandi,
    PemasanganMesinCuci,
    PemasanganDishwasher,
}

impl Layanan {
    pub fn as_string(&self) -> &'static str {
        match self {
            Layanan::InstalasiPipaBaru => "Instalasi Pipa Baru",
            Layanan::PerbaikanKebocoranPipa => "Perbaikan Kebocoran Pipa",
            Layanan::DeteksiKebocoran => "Deteksi Kebocoran",
            Layanan::PembersihanSaluranMampet => "Pembersihan Saluran Mampet",
            Layanan::HydroJetting => "Hydro Jetting",
            Layanan::PenggantianPipaLama => "Penggantian Pipa Lama",
            Layanan::PenyambunganPipa => "Penyambungan Pipa",
            Layanan::PengelasanPipa => "Pengelasan Pipa",
            Layanan::PemasanganKelengkapanSanitasi => "Pemasangan Kelengkapan Sanitasi",
            Layanan::PemasanganFilterAir => "Pemasangan Filter Air",
            Layanan::PemasanganWaterHeater => "Pemasangan Water Heater",
            Layanan::PemasanganPompaAir => "Pemasangan Pompa Air",
            Layanan::ServisPompaAir => "Servis Pompa Air",
            Layanan::PemasanganTangkiAir => "Pemasangan Tangki Air",
            Layanan::PemasanganTorenBawahTanah => "Pemasangan Toren Bawah Tanah",
            Layanan::PerbaikanToilet => "Perbaikan Toilet",
            Layanan::PerbaikanWastafel => "Perbaikan Wastafel",
            Layanan::PemasanganBathtubShower => "Pemasangan Bathtub / Shower",
            Layanan::PemasanganPemanasMandi => "Pemasangan Pemanas Mandi",
            Layanan::PemasanganMesinCuci => "Pemasangan Mesin Cuci",
            Layanan::PemasanganDishwasher => "Pemasangan Dishwasher",
        }
    }

    pub fn from_string(input: &str) -> Option<Layanan> {
        match input {
            "Instalasi Pipa Baru" => Some(Layanan::InstalasiPipaBaru),
            "Perbaikan Kebocoran Pipa" => Some(Layanan::PerbaikanKebocoranPipa),
            "Deteksi Kebocoran" => Some(Layanan::DeteksiKebocoran),
            "Pembersihan Saluran Mampet" => Some(Layanan::PembersihanSaluranMampet),
            "Hydro Jetting" => Some(Layanan::HydroJetting),
            "Penggantian Pipa Lama" => Some(Layanan::PenggantianPipaLama),
            "Penyambungan Pipa" => Some(Layanan::PenyambunganPipa),
            "Pengelasan Pipa" => Some(Layanan::PengelasanPipa),
            "Pemasangan Kelengkapan Sanitasi" => Some(Layanan::PemasanganKelengkapanSanitasi),
            "Pemasangan Filter Air" => Some(Layanan::PemasanganFilterAir),
            "Pemasangan Water Heater" => Some(Layanan::PemasanganWaterHeater),
            "Pemasangan Pompa Air" => Some(Layanan::PemasanganPompaAir),
            "Servis Pompa Air" => Some(Layanan::ServisPompaAir),
            "Pemasangan Tangki Air" => Some(Layanan::PemasanganTangkiAir),
            "Pemasangan Toren Bawah Tanah" => Some(Layanan::PemasanganTorenBawahTanah),
            "Perbaikan Toilet" => Some(Layanan::PerbaikanToilet),
            "Perbaikan Wastafel" => Some(Layanan::PerbaikanWastafel),
            "Pemasangan Bathtub / Shower" => Some(Layanan::PemasanganBathtubShower),
            "Pemasangan Pemanas Mandi" => Some(Layanan::PemasanganPemanasMandi),
            "Pemasangan Mesin Cuci" => Some(Layanan::PemasanganMesinCuci),
            "Pemasangan Dishwasher" => Some(Layanan::PemasanganDishwasher),
            _ => None,
        }
    }
}
