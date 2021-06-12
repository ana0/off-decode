use std::{fs, io};
use std::str;
use ecies::*;
use shamirsecretsharing::*;
use std::fs::File;
use std::io::Write;
use substring::Substring;
use image::{open, Rgb};
use std::convert::TryInto;

fn main() {
    let mut file = File::create("secret.txt").expect("Created file");

    let mut shares: Vec<Vec<u8>> = Vec::new();
    
    struct Sentence {
        text: Vec<u8>,
        token_id: isize,
    }

    let mut sentences: Vec<Sentence> = Vec::new();

    let images = find_images();
    let images_iter = images.iter();

    for image in images_iter {
        let blob = parse_datablob(image);
        let share = extract_share(&blob);
        shares.push(share);
        let sentence = Sentence {
            text: extract_sentence(&blob),
            token_id: extract_token_id(&blob),
        };
        sentences.push(sentence);
    }
    let restored = combine_shares(&shares).unwrap();
    let mut key: [u8; 32] = Default::default();
    key.copy_from_slice(&restored.unwrap()[0..32]);
    
    let mut last_id = 0;
    for sentence in sentences.iter() {
        let secret_key = SecretKey::parse(&key).expect("32 bytes, within curve order");
        let decrypted = decrypt(&secret_key.serialize(), &sentence.text).unwrap();
        while sentence.token_id > last_id {
            file.write_all(b"\n").expect("Wrote");
            last_id += 1;
        }
        file.write_all(str::from_utf8(&decrypted).unwrap().as_bytes()).expect("Wrote");
        println!("{:?}", str::from_utf8(&decrypted).unwrap());
    }
}

fn find_images() -> Vec<String>  {
    let mut entries = fs::read_dir(".").expect("Read directory")
        .map(|res| {
            return res.map(|e| {
                return e.file_name().into_string().unwrap();
            });
        })
        .collect::<Result<Vec<_>, io::Error>>().expect("Read file");
    entries.sort();

    return only_pngs(entries);
}

fn only_pngs(filenames: Vec<String>) -> Vec<String> {
    return filenames.into_iter().filter(|name| name.contains("png")).collect();
}

fn extract_token_id(blob: &str) -> isize {
    let token_prefix = blob.substring(0, 16);
    let token = isize::from_str_radix(token_prefix, 2).unwrap();
    return token;
}

fn extract_length(blob: &str) -> isize {
    let length_prefix = blob.substring(16, 32);
    let length = isize::from_str_radix(length_prefix, 2).unwrap();
    return length + 32;
}

fn extract_share(blob: &str) -> Vec<u8> {
    let share_length = 1356;
    let share_string = blob.substring(blob.len()-share_length, blob.len());

    let mut share: Vec<u8> = Vec::new();
    for n in 0..=(share_string.len()/12)-1 {
        let piece = share_string.substring(n*12, (n*12)+12);
        share.push(u8::from_str_radix(piece, 2).unwrap());
    }
    return share;
}

fn extract_sentence(blob: &str) -> Vec<u8> {
    let linebreak_share_length = 1556;
    let sentence_string = blob.substring(32, blob.len()-(linebreak_share_length));
    
    let mut letters: Vec<u8> = Vec::new();
    for n in 0..=(sentence_string.len()/12)-1 {
        let piece = sentence_string.substring(n*12, (n*12)+12);
        letters.push(u8::from_str_radix(piece, 2).unwrap());
    }
    return letters;
}

fn parse_datablob(image: &str) -> String {
    let img = open(&image).unwrap().into_rgb8();

    let black: [u8; 3] = [0, 0, 0];
    let mut blob = "".to_string();
    for (_x, _y, pixel) in img.enumerate_pixels() {
        if pixel != &Rgb(black) {
            //println!("{:?}", pixel);
            blob = blob + "1";
        } else {
            blob = blob + "0";
        }
    }
    let length = extract_length(&blob);
    println!("{:?}", length);
    return blob.substring(0, length.try_into().unwrap()).to_string();
}

