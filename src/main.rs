#![allow(deprecated)]
use region_cn::region::Region;
use std::{
    env,
    fs::{self, File},
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
};

// region data 下载地址
const REGION_DAT_URL: &str =
    "https://github.com/bujnlc8/region-cn/raw/refs/heads/main/data/region_full.dat";

fn download_region_dt(download_url: &str, dest: &Path) -> Result<(), Error> {
    match reqwest::blocking::get(download_url) {
        Ok(mut resp) => {
            let dest_dir = dest.parent().unwrap();
            if !dest_dir.exists() {
                fs::create_dir_all(dest_dir)?;
            }
            let mut dest_file = File::create(dest)?;
            io::copy(&mut resp, &mut dest_file)?;
            Ok(())
        }
        Err(e) => Err(Error::new(ErrorKind::TimedOut, e.to_string())),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("USAGE: idcard your-idcard-number");
        std::process::exit(1);
    }
    let idcard = args.get(1).unwrap();
    if idcard.len() != 18 {
        eprintln!("只支持中国大陆18位身份证");
        std::process::exit(1);
    }
    let dat_path = PathBuf::from(format!(
        "{}/.cache/region_full.dat",
        env::home_dir().unwrap().to_str().unwrap(),
    ));
    if !dat_path.exists() {
        let _ = download_region_dt(REGION_DAT_URL, &dat_path).is_err_and(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
    }
    let mut region = Region::new(dat_path);
    match region.search_with_data(&idcard[..6]) {
        Ok(result) => {
            println!(
                "生日: {}-{}-{}",
                &idcard[6..10],
                &idcard[10..12],
                &idcard[12..14]
            );
            let sex: i32 = idcard[16..17].parse().unwrap();
            if sex % 2 == 0 {
                println!("性别: 女");
            } else {
                println!("性别: 男");
            }
            println!("区号: {}", result.region_code);
            if result.discard_year > 0 {
                println!("地址: {} ({}年废止)", result.name, result.discard_year);
            } else {
                println!("地址: {}", result.name);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
