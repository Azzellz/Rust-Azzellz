use std::fs::{read, File};
use std::io::{prelude::*, stdin};
use std::path::Path;
fn main() {

    println!("🥳 Welcome to yuzu_locker 🥳\nPlease input the file path you want to lock/unlock.\nthe program will judge the file which need to lock or unlock.\nyou just need to provide the correct path ✨ :\n");
    loop {
        let mut path = String::new();

        stdin().read_line(&mut path).expect("Failed to read input line!");

        //trim()去掉路径后面的回车,即换行符.
        let path = path.trim();

        //先确定这个文件是否存在
        if !check_file(path) {
            println!("❌ the file not exist, please check the path! ❌\n");
            continue;
        }


        //根据文件的拓展名来判断是加密还是解密
        match get_file_foot(path) {
            "yuzu" => {
                println!("the file going to unlock...");
                unlock(114, &format!("{}.yuzu", get_file_name(path)))
            }
            _ => {
                println!("the file going to lock...");
                lock(114, path)
            }
        }
        println!("Finish! you can lock/unlock again!\n")
    }
}

//检查文件是否存在,如果存在返回true,否则返回false
fn check_file(path: &str) -> bool {
    Path::new(path).exists()
}

fn lock(key: u8, path: &str) {
    //以字节vec读取文件
    let mut arr = read(path).expect("lock error - read error , please check the path ");
    //获取文件名,并且去掉拓展名,因为最后一行是拓展名
    let file_name = get_file_name(path);

    //这里确保最后一行是文件的拓展名
    //将拓展名转换为字节vec
    let foot = get_file_foot(path);
    let file_foot = String::new() + foot + foot.len().to_string().as_str();
    let file_foot = file_foot.as_bytes().to_vec();

    //将拓展名加入到arr中
    arr.extend(file_foot);

    //使用迭代器对每个比特进行key的加密
    arr.iter_mut().for_each(|x| {
        *x = x.wrapping_add(key);
    });

    //在当前文件夹创建加密文件
    let target = &format!("{}.yuzu", file_name)[..];
    create_file(target, &arr);
}

fn unlock(key: u8, path: &str) {
    let mut arr = read(path).expect("unlock error ! read error , please check the path ");

    //对每个比特进行key的解密
    arr.iter_mut().for_each(|x| {
        *x = x.wrapping_sub(key);
    });
    
    //对读取出的arr进行处理,去掉最后一行,因为最后一行是拓展名
    let count = arr.pop().unwrap()-'0' as u8;
    let mut foot = Vec::new();
    for _ in 0..count {
        let a = arr.pop().unwrap();
        foot.push(a)
    }

    foot.reverse();

    let foot = String::from_utf8(foot).unwrap();
    
    //获取文件名
    let file_name = get_file_name(path);

    let target = &format!("{}.{}", file_name,foot)[..];
    create_file(target, &arr);
}

fn get_file_name(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split(".")
        .collect::<Vec<&str>>()[0]
}

fn get_file_foot(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split(".")
        .collect::<Vec<&str>>()[1]
}

fn get_file_fullname(path: &str) -> String {
    let fullname = Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split(".")
        .collect::<Vec<&str>>();
    if fullname.len() < 2 {
        panic!("error file name without the foot")
    }
    format!("{}.{}", fullname[0], fullname[1])
}

fn create_file(path: &str, arr: &Vec<u8>) {
    File::create(path)
        .expect("Create file error")
        .write_all(&arr)
        .expect("Write file error");
}
