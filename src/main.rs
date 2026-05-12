use chrono::{Datelike, NaiveDate,};
use clap::Parser;

pub struct Keygen;

impl Keygen {
    const DATA: [u32; 256] = [
        0x39cb44b8, 0x23754f67, 0x5f017211, 0x3ebb24da, 0x351707c6, 0x63f9774b, 0x17827288, 0x0fe74821,
        0x5b5f670f, 0x48315ae8, 0x785b7769, 0x2b7a1547, 0x38d11292, 0x42a11b32, 0x35332244, 0x77437b60,
        0x1eab3b10, 0x53810000, 0x1d0212ae, 0x6f0377a8, 0x43c03092, 0x2d3c0a8e, 0x62950cbf, 0x30f06ffa,
        0x34f710e0, 0x28f417fb, 0x350d2f95, 0x5a361d5a, 0x15cc060b, 0x0afd13cc, 0x28603bcf, 0x3371066b,
        0x30cd14e4, 0x175d3a67, 0x6dd66a13, 0x2d3409f9, 0x581e7b82, 0x76526b99, 0x5c8d5188, 0x2c857971,
        0x15f51fc0, 0x68cc0d11, 0x49f55e5c, 0x275e4364, 0x2d1e0dbc, 0x4cee7ce3, 0x32555840, 0x112e2e08,
        0x6978065a, 0x72921406, 0x314578e7, 0x175621b7, 0x40771dbf, 0x3fc238d6, 0x4a31128a, 0x2dad036e,
        0x41a069d6, 0x25400192, 0x00dd4667, 0x6afc1f4f, 0x571040ce, 0x62fe66df, 0x41db4b3e, 0x3582231f,
        0x55f6079a, 0x1ca70644, 0x1b1643d2, 0x3f7228c9, 0x5f141070, 0x3e1474ab, 0x444b256e, 0x537050d9,
        0x0f42094b, 0x2fd820e6, 0x778b2e5e, 0x71176d02, 0x7fea7a69, 0x5bb54628, 0x19ba6c71, 0x39763a99,
        0x178d54cd, 0x01246e88, 0x3313537e, 0x2b8e2d17, 0x2a3d10be, 0x59d10582, 0x37a163db, 0x30d6489a,
        0x6a215c46, 0x0e1c7a76, 0x1fc760e7, 0x79b80c65, 0x27f459b4, 0x799a7326, 0x50ba1782, 0x2a116d5c,
        0x63866e1b, 0x3f920e3c, 0x55023490, 0x55b56089, 0x2c391fd1, 0x2f8035c2, 0x64fd2b7a, 0x4ce8759a,
        0x518504f0, 0x799501a8, 0x3f5b2cad, 0x38e60160, 0x637641d8, 0x33352a42, 0x51a22c19, 0x085c5851,
        0x032917ab, 0x2b770ac7, 0x30ac77b3, 0x2bec1907, 0x035202d0, 0x0fa933d3, 0x61255df3, 0x22ad06bf,
        0x58b86971, 0x5fca0de5, 0x700d6456, 0x56a973db, 0x5ab759fd, 0x330e0be2, 0x5b3c0ddd, 0x495d3c60,
        0x53bd59a6, 0x4c5e6d91, 0x49d9318d, 0x103d5079, 0x61ce42e3, 0x7ed5121d, 0x14e160ed, 0x212d4ef2,
        0x270133f0, 0x62435a96, 0x1fa75e8b, 0x6f092fbe, 0x4a000d49, 0x57ae1c70, 0x004e2477, 0x561e7e72,
        0x468c0033, 0x5dcc2402, 0x78507ac6, 0x58af24c7, 0x0df62d34, 0x358a4708, 0x3cfb1e11, 0x2b71451c,
        0x77a75295, 0x56890721, 0x0fef75f3, 0x120f24f1, 0x01990ae7, 0x339c4452, 0x27a15b8e, 0x0ba7276d,
        0x60dc1b7b, 0x4f4b7f82, 0x67db7007, 0x4f4a57d9, 0x621252e8, 0x20532cfc, 0x6a390306, 0x18800423,
        0x19f3778a, 0x462316f0, 0x56ae0937, 0x43c2675c, 0x65ca45fd, 0x0d604ff2, 0x0bfd22cb, 0x3afe643b,
        0x3bf67fa6, 0x44623579, 0x184031f8, 0x32174f97, 0x4c6a092a, 0x5fb50261, 0x01650174, 0x33634af1,
        0x712d18f4, 0x6e997169, 0x5dab7afe, 0x7c2b2ee8, 0x6edb75b4, 0x5f836fb6, 0x3c2a6dd6, 0x292d05c2,
        0x052244db, 0x149a5f4f, 0x5d486540, 0x331d15ea, 0x4f456920, 0x483a699f, 0x3b450f05, 0x3b207c6c,
        0x749d70fe, 0x417461f6, 0x62b031f1, 0x2750577b, 0x29131533, 0x588c3808, 0x1aef3456, 0x0f3c00ec,
        0x7da74742, 0x4b797a6c, 0x5ebb3287, 0x786558b8, 0x00ed4ff2, 0x6269691e, 0x24a2255f, 0x62c11f7e,
        0x2f8a7dcd, 0x643b17fe, 0x778318b8, 0x253b60fe, 0x34bb63a3, 0x5b03214f, 0x5f1571f4, 0x1a316e9f,
        0x7acf2704, 0x28896838, 0x18614677, 0x1bf569eb, 0x0ba85ec9, 0x6aca6b46, 0x1e43422a, 0x514d5f0e,
        0x413e018c, 0x307626e9, 0x01ed1dfa, 0x49f46f5a, 0x461b642b, 0x7d7007f2, 0x13652657, 0x6b160bc5,
        0x65e04849, 0x1f526e1c, 0x5a0251b6, 0x2bd73f69, 0x2dbf7acd, 0x51e63e80, 0x5cf2670f, 0x21cd0a03,
        0x5cff0261, 0x33ae061e, 0x3bb6345f, 0x5d814a75, 0x257b5df4, 0x0a5c2c5b, 0x16a45527, 0x16f23945
    ];

    fn generate_license_key(name: &str,num_days:u32, num_users: u32) -> u32 {
        let mut day_index = num_days.wrapping_add(num_days << 4) as usize;
        let mut user_index  = (num_users << 4).wrapping_sub(num_users) as usize;
        //let mut temp_num = (num_users << 4).wrapping_sub(num_users);

        let mut hash_value: u32 = 0;
        //let mut data2: usize = 0;
        let mut counter: usize = 0;

        for &t in name.to_ascii_uppercase().as_bytes() {
            let mut ch = t as usize;
            hash_value = hash_value.wrapping_add(Self::DATA[ch & 0xFF]);
            hash_value ^= Self::DATA[(ch.wrapping_add(0xD)) & 0xFF];
            hash_value = hash_value.wrapping_mul(Self::DATA[(ch.wrapping_add(0x2F)) & 0xFF]);

            hash_value = hash_value.wrapping_add(Self::DATA[day_index & 0xFF]);
            hash_value = hash_value.wrapping_add(Self::DATA[user_index & 0xFF]);
            hash_value = hash_value.wrapping_add(Self::DATA[counter & 0xFF]);

            counter += 19;
            day_index += 9;
            user_index += 13;
        }
        hash_value
    }

    fn check_input(username: &str, number: u32) -> bool {
        let len = username.as_bytes().len();
        number >= 1 && number <= 1000 && len > 0 && len < 15
    }

    pub fn make_license(username: &str, days_since_1970: u32, num_users: u32) -> String {
        if !Self::check_input(username, num_users) {
            return String::new();
        }

        let hash = Self::generate_license_key(username, days_since_1970, num_users).to_le_bytes();
        let encode_num_users = (((num_users.wrapping_mul(0xB)^ 0x3421).wrapping_sub(0x4D30)) ^ 0x7892).to_le_bytes();
        let mut encode_days = (days_since_1970 * 17) ^ 0xE53167;
        encode_days = encode_days.wrapping_add(0x2C175) ^ 0x794C5F;
        let encode_days = encode_days.to_le_bytes();
        let mut key_codes = [0u8; 10];
        key_codes[0] = encode_days[0] ^ hash[2];
        key_codes[1] = encode_num_users[1] ^ hash[3];
        key_codes[2] = encode_num_users[0] ^ hash[1];
        key_codes[3] = 0xAC;
        key_codes[4] = hash[0]; // byte thấp nhất
        key_codes[5] = hash[1]; // byte thứ 2
        key_codes[8] = hash[0] ^ encode_days[1];

        //đảo byte của encode_days
        key_codes[9] = encode_days[2] ^ hash[1];
        let temp =
        key_codes[6] = hash[2];
        key_codes[7] = hash[3];
        format!(
            "{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}",
            key_codes[0], key_codes[1],
            key_codes[2], key_codes[3],
            key_codes[4], key_codes[5],
            key_codes[6], key_codes[7],
            key_codes[8], key_codes[9]
        )
    }

    /// Hàm tính số ngày từ 1/1/1970
    pub fn date_to_unix_days(day:u32,month:u32,year:i32) -> Option<i32>{
        let start_date = NaiveDate::from_ymd_opt(1970, 1, 1)?;
        let target_date = NaiveDate::from_ymd_opt(year, month, day)?;
        // Tính khoảng cách giữa 2 mốc thời gian
        let duration = target_date.signed_duration_since(start_date);
        Some(duration.num_days() as i32)
    }

    /// Hàm tạo key nhận vào Ngày/Tháng/Năm
    pub fn make_key_from_date(username: &str, users: u32, y: i32, m: u32, d: u32) -> String {
        match Self::date_to_unix_days( d,m,y) {
            Some(num_days) => {
                // Gọi lại hàm make_key cũ với num_days đã tính được
                Self::make_license(username, num_days as u32, users)
            },
            None => "Ngày tháng không hợp lệ".to_string(),
        }
    }
}

fn validate_username(s: &str) -> Result<String, String> {
    if s.is_empty() {
        Err(String::from("Tên người dùng không được để trống."))
    } else if s.len() > 15 {
        Err(String::from("Tên người dùng không được vượt quá 15 ký tự."))
    } else {
        Ok(s.to_string())
    }
}
/// Công cụ tạo License Key cho 010 Editor
#[derive(Parser, Debug,Default)]
#[command(author = "YourName", version = "1.0", about = "010 Editor Keygen CLI", long_about = None)]
struct Args {
    /// Tên người dùng (Username)
    #[arg(short, long,required = true, value_parser = validate_username, help="Nhập tên đăng ký (Ví dụ: 'Nguyen Van A')")]
    user_name: String,

    /// Năm hết hạn (ví dụ: 2026)
    #[arg(short, long, help = "Ví dụ: 2026", required = false,default_value = "2999")]
    year: i32,

    /// Tháng hết hạn (1-12)
    /// Tháng hết hạn (1 - 12)
    #[arg(short, long, required = false,default_value = "1",help = "Tháng hết hạn (1 - 12)", value_parser = clap::value_parser!(u32).range(1..=12))]
    month: u32,

    /// Ngày hết hạn (1-31)
    #[arg(short, long,required = false,default_value = "1", help = "Ngày hết hạn trong tháng (1 - 31)", value_parser = clap::value_parser!(u32).range(1..=31))]
    day: u32,
}
fn main() {
    let mut args = Args::parse();
    match Keygen::date_to_unix_days(args.day,args.month,args.year) {
        Some(days) if days < 0=>{
            let date = NaiveDate::MAX;
            args.day = date.day();
            args.month = date.month();
            args.year = date.year();
        },
        None =>{
            let date = NaiveDate::MAX;
            args.day = date.day();
            args.month = date.month();
            args.year = date.year();
        }
        _ =>{}
    }
    let key = Keygen::make_key_from_date(&args.user_name, 1,args.year,args.month,args.day);
    println!("---------------------------------------");
    println!("User Name : {}", args.user_name);
    println!("Expiry    : {:02}/{:02}/{}", args.day, args.month, args.year);
    println!("License   : {}", key);
    println!("---------------------------------------");
}