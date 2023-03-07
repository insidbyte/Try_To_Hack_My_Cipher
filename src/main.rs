
use std:: collections::HashMap;
use std::fs;

use simmetric::{Crypt, check_mapp, check_arrayy};
use serde_json;
fn main(){

    let corrispond: String =  String::from("a b c d e f g h i l m n o p q r s t u v z a b c d e f g h i l m n o p q r s t u v z A B C D E F G H I L M N O P Q R S T U V Z A B C D E F G H I L M N O P Q R S T U V Z 0 1 2 3 4 5 6 7 8 9 9 0 1 2 3 4 5 6 7 8 9 , . - ò à ù è + ' ì , . - ò à ù è + ' ì ! £ $ % & / ( ) = ? ^ ! £ $ % & ( ) = ? ^ ; : _ ç ° § é * ; : _ ç ° § é *"); //a b c d e f g h i l m n o p q r s t u v z a b c d e f g h i l m n o p q r s t u v z A B C D E F G H I L M N O P Q R S T U V Z A B C D E F G H I L M N O P Q R S T U V Z 0 1 2 3 4 5 6 7 8 9 9 0 1 2 3 4 5 6 7 8 9 , . - ò à ù è + ' ì , . - ò à ù è + ' ì ! £ $ % & / ( ) = ? ^ ! £ $ % & ( ) = ? ^ ; : _ ç ° § é * ; : _ ç ° § é * a b c d e f g h i l m n o p q r s t u v z a b c d e f g h i l m n o p q r s t u v z A B C D E F G H I L M N O P Q R S T U V Z A B C D E F G H I L M N O P Q R S T U V Z 0 1 2 3 4 5 6 7 8 9 9 0 1 2 3 4 5 6 7 8 9 , . - ò à ù è + ' ì , . - ò à ù è + ' ì ! £ $ % & / ( ) = ? ^ ! £ $ % & ( ) = ? ^ ; : _ ç ° § é * ; : _ ç ° § é * a b c d e f g h i l m n o p q r s t u v z a b c d e f g h i l m n o p q r s t u v z A B C D E F G H I L M N O P Q R S T U V Z A B C D E F G H I L M N O P Q R S T U V Z 0 1 2 3 4 5 6 7 8 9 9 0 1 2 3 4 5 6 7 8 9 , . - ò à ù è + ' ì , . - ò à ù è + ' ì ! £ $ % & / ( ) = ? ^ ! £ $ % & ( ) = ? ^ ; : _ ç ° § é * ; : _ ç ° § é * a b c d e f g h i l m n o p q r s t u v z a b c d e f g h i l m n o p q r s t u v z A B C D E F G H I L M N O P Q R S T U V Z A B C D E F G H I L M N O P Q R S T U V Z 0 1 2 3 4 5 6 7 8 9 9 0 1 2 3 4 5 6 7 8 9 , . - ò à ù è + ' ì , . - ò à ù è + ' ì ! £ $ % & / ( ) = ? ^ ! £ $ % & ( ) = ? ^ ; : _ ç ° § é * ; : _ ç ° § é * a b c d e f g h i l m n o p q r s t u v z a b c d e f g h i l m n o p q r s t u v z A B C D E F G H I L M N O P Q R S T U V Z A B C D E F G H I L M N O P Q R S T U V Z 0 1 2 3 4 5 6 7 8 9 9 0 1 2 3 4 5 6 7 8 9 , . - ò à ù è + ' ì , . - ò à ù è + ' ì ! £ $ % & / ( ) = ? ^ ! £ $ % & ( ) = ? ^ ; : _ ç ° § é * ; : _ ç ° § é * a b c d e f g h i l m n o p q r s t u v z a b c d e f g h i l m n o p q r s t u v z A B C D E F G H I L M N O P Q R S T U V Z A B C D E F G H I L M N O P Q R S T U V Z 0 1 2 3 4 5 6 7 8 9 9 0 1 2 3 4 5 6 7 8 9 , . - ò à ù è + ' ì , . - ò à ù è + ' ì ! £ $ % & / ( ) = ? ^ ! £ $ % & ( ) = ? ^ ; : _ ç ° § é * ; : _ ç ° § é * a b c d e f g h i l m n o p q r s t u v z a b c d e f g h i l m n o p q r s t u v z A B C D E F G H I L M N O P Q R S T U V Z A B C D E F G H I L M N O P Q R S T U V Z 0 1 2 3 4 5 6 7 8 9 9 0 1 2 3 4 5 6 7 8 9 , . - ò à ù è + ' ì , . - ò à ù è + ' ì ! £ $ % & / ( ) = ? ^ ! £ $ % & ( ) = ? ^ ; : _ ç ° § é * ; : _ ç ° § é * ");
    let mut finale: String = String::new();
    for i in 0..240{
        finale += &corrispond.clone().as_str();
    }
    let corrispond = finale;
    
    let generic = HashMap::new();
    
    
    if check_mapp("src/map.json".to_string()){
        let data = fs::read_to_string("src/map.json").unwrap();
    
    
        let this_map = serde_json::from_str::<HashMap<i64, &str>>(data.as_str()).unwrap();
        if check_arrayy("src/array.json".to_string()){
            let data = fs::read_to_string("src/array.json").unwrap();
        
        
            let mut this_array = serde_json::from_str::<Vec<i64>>(data.as_str()).unwrap();

            let (map, crypt)  = Crypt::build(&corrispond, this_map.clone(), this_array.clone());
            let secret_text = crypt.crypt(this_map.clone());
            let derypt_text = crypt.decript(this_map, secret_text.clone());
            
            println!("Criptata: {:#?}", secret_text.clone());
            for i in secret_text.clone(){
                let mut cont = 0;
                for j in secret_text.clone(){
                    if i == j{
                        cont+=1;
                    }
                }
                //print!("{cont}");
            }
            println!("Decifrata: {:#?}", derypt_text.trim());
        }
        
    }else{

        //let mut test: Vec<i64> = Vec::new();
        //test.push(1);
        //let data = fs::read_to_string("src/array.json").unwrap();
        let mut test = Vec::new();
        test.push(1);
        let (map, crypt)  = Crypt::build(&corrispond, generic, test);
        
    }
    
    /* 
    let this_map: HashMap<i64, &str> = match map {
        Ok(map) => map,
        Err(e) => panic!("Error: {}", e),
    };
    */
    //println!("KEY: {:#?}", map.keys());
    
    //println!("VALUE: {:#?}", map.values());
    
    //println!("MAPPA: {:#?}", map);
    

}   