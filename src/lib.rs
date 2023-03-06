use std::collections::HashMap;
use serde_json;
use std::{fs::{File, OpenOptions}, io::ErrorKind};
use std::fs;

#[derive(Debug)]
pub struct Crypt<'a>{
    pub map: HashMap<i64, &'a str>,
    pub plain_text: String, 
}

impl<'a> Crypt<'a> {
    fn check_map(path: String) -> bool{
        let file = File::open(path);
        match file {
            Ok(file) => return true,
            Err(e) => return false,
        }
    }
    fn save_map(map: HashMap<i64, &str>, path: String) {
        let file = File::open(path.clone());
        
        let this_file = match file {
            Ok(file) => serde_json::to_writer(file, &map).expect("error"),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match OpenOptions::new().create(true).write(true).open(path.clone()) {
                    Ok(fc) => serde_json::to_writer(fc, &map).expect("error"),
                    Err(e) => panic!("Impossibile creare file"),
                }
                other => panic!("other error"),
            }
        };
    }
    
    fn save_array(array: Vec<i64>, path: String){
        let file = File::open(path.clone());

        let this_file = match file {
            Ok(file) => serde_json::to_writer(file, &array).expect("error"),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match OpenOptions::new().create(true).write(true).open(path.clone()) {
                    Ok(fc) => serde_json::to_writer(fc, &array).expect("error"),
                    Err(e) => panic!("Impossibile creare file"),
                }
                other => panic!("other error"),
            }
        };
    }
    fn load_array(){

    }
    fn new(plain_text: String, map: HashMap<i64, &'a str>) -> Self{
        Self { 
            map: map,
            plain_text: plain_text, 
        }
    }
    pub fn build(corrispond: &'a String, mappa: HashMap<i64, &'a str>, secret: Vec<i64>) ->  (HashMap<i64, &'a str>, Self ) {
        if !Crypt::check_map("src/map.json".to_string()){
            let mut map: HashMap<i64, &str> = HashMap::new();
            let mut secret = secret;
            // if i haven't the map and array files in this place i should put the file code situated in:
            // simmetric/codice tolto array.txt with the correct instructions.
            Crypt::save_array(secret.clone(), "src/array.json".to_string());
            let mut start = secret.len()-1;
            
            let mut vector = Vec::new();
            
            
            for c in corrispond.split(" "){
                vector.push(c);   
            }
            for i in 0..240{
                vector.push(" ");
            }
            

            println!("Digitare frase da criptare:\n");
            let mut plain = String::new();
            /*for i in 0..240{
                plain += "ciao ";
            }*/
            std::io::stdin().read_line(&mut plain);
        

            
            for c in 0..vector.len() {
                secret.sort();
                vector.sort();
                map.insert(secret[start], vector[c]);    
                start -= 1;
            }

            let crypt = Self{
                plain_text: plain,
                map: map.clone(),
            };
            //println!("MAPPA: {:#?}", map);
            
            Crypt::save_map(map.clone(), "src/map.json".to_string());
            
            //Crypt::save_map(map.clone(), "src/map.json".to_string());
            return (map, crypt);
        }else{
            
            
           
            let mut plain = String::new();
            println!("Inserire frase: ");
            std::io::stdin().read_line(&mut plain);

            let crypt = Self{
                plain_text: plain,
                map: mappa.clone(),
            };
            
            return (mappa, crypt)
        }

    }

    
    pub fn crypt(&self, map: HashMap<i64, &str>) -> Vec<i64>{
        let mut vector = Vec::new();
        for (k, v) in map.clone(){
            vector.push(v);
        }   
        let mut crypt: Vec<i64> = Vec::new();
        let plain = self.plain_text.as_str().split("");
        let mut cont = 0;
        let mut cont_space = 0;
        let mut cond = false;
        let mut temp: i64 = 0; 
        let mut temp_space: i64 = 0;

        
        
        
        for c in plain.clone(){
            cond = false;
            cont = 0;
            cont_space = 0;
            
            //println!("{:#?}",plain.clone());
            
            
            for (k, v) in map.clone(){
                if v == c {
                    if !cond && k != temp && !crypt.contains(&k){
                        cond = true;
                        crypt.push(k);
                        temp = k;
                    }
                    if cond {
                        if v == c && k != temp{
                            cont += 1;
                            if cont == 1000  && v != " "{
                                cond = false;
                                crypt.push(k);
                                print!("miao");
                            }
                        }
                    }
                    
                }/*else if v == c && v == " " {
                    if temp_space == 0 && cont_space == 0{
                        crypt.push(k);
                        temp_space = k;
                        cont_space += 1;
                    }else if temp_space != k && cont_space == 0{
                        crypt.push(k);
                        temp_space = 0;
                        cont_space += 1;
                    }
    
                }*/
                
            }
            
        }
        return crypt;
    }

    pub fn decript(&self, map: HashMap<i64, &'a str>, crypt: Vec<i64>) -> String{
        let mut clean = String::from("");
        let mut secret = crypt;
        
        for n in secret.clone(){
            for (k,v) in map.clone() {
                
                if k == n {
                    clean += &v.to_string();

                }   
        }
        }
        return clean;
    }
}

pub fn check_mapp(path: String) -> bool{
    let file = File::open(path);
    match file {
        Ok(file) => return true,
        Err(e) => return false,
    }
}
pub fn check_arrayy(path: String) -> bool{
    let file = File::open(path);
    match file {
        Ok(file) => return true,
        Err(e) => return false,
    }
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
