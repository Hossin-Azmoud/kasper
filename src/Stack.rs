// STACK.
use std::collections::HashMap;
use crate::enums::BOOL_TRUE;



pub struct Stack {
    
    pub bool_map:     HashMap<String, bool>,
    pub str_map:      HashMap<String, String>,
    pub int_map_8:    HashMap<String, i8>, // 8 bit..
    pub int_map_16:   HashMap<String, i16>, // 16 bit..
    pub int_map:      HashMap<String, i32>, // 32 bit.     
    pub int_map_64:   HashMap<String, i64>, // 64 bit..
    pub int_map_128:  HashMap<String, i128>, // 128 bit..
     
}


#[allow(dead_code)]
impl Stack {
    pub fn new() -> Self {
        Self {
            bool_map:           HashMap::new(),
            str_map:            HashMap::new(), 
            
            int_map_8:          HashMap::new(),
            int_map_16:         HashMap::new(),
            int_map:            HashMap::new(),
            int_map_64:         HashMap::new(),
            int_map_128:        HashMap::new(),
            
        }
    }
     
    pub fn print_variable(&mut self, key: &String) -> bool {

            if let Some(v) = self.get_from_int_map(key) {
                print!("{}", v);
                return true;
            }

            if let Some(v) = self.get_from_str_map(key) {
                print!("{}", v);
                return true;
            }
            

            if let Some(v) = self.get_from_int_map_64(key) { 
                print!("{}", v);
                return true;
            }
            
            if let Some(v) = self.get_from_bool_map(key) {
                if *v {
                    print!("{}", BOOL_TRUE);
                    return true;           
                }
                
                print!("False");
                return true;
            }

            return false;
    }

    pub fn get_int(&mut self, key: &String) -> Option<f64>{
        
        if let Some(v) = self.get_from_int_map(key) {
            return Some(*v as f64);
        }


        if let Some(v) = self.get_from_int_map_64(key) { 
            return Some(*v as f64);
        }

        return None;
    }
    
    pub fn defined(&mut self, key: &String) -> bool {
        return self.int_map.contains_key(key) 
            || self.str_map.contains_key(key) 
            || self.int_map_64.contains_key(key)
            || self.bool_map.contains_key(key);
    }
    
    pub fn get_from_bool_map(&mut self, key: &String) -> Option<&bool> {

        if self.bool_map.contains_key(key) {
            return Some(&self.bool_map[key]);
        }  

        return None; 
    } 
    
    pub fn get_from_int_map_64(&mut self, key: &String) -> Option<&i64> {

        if self.int_map_64.contains_key(key) {
            return Some(&self.int_map_64[key]);
        }  

        return None; 
    }  

    pub fn get_from_int_map(&mut self, key: &String) -> Option<&i32> {
        if self.int_map.contains_key(key) {
            return Some(&self.int_map[key]);
        }  

        return None; 
    }  
    
    pub fn get_from_str_map(&mut self, key: &String) -> Option<&String> {
        
        if self.str_map.contains_key(key) {
            return Some(&self.str_map[key]);
        }

        return None; 
    }  
/* 
    pub fn get_from_int_map_8(&mut self, key: &String) -> Option<&i8> {
        if self.int_map_8.contains_key(&key) {
            return Some(&self.int_map_8[&key]);
        }

        return None; 
    }  
     
    pub fn get_from_int_map_16(&mut self, key: &String) -> Option<&i16> {
        if self.int_map_16.contains_key(&key) {
            return Some(&self.int_map_16[&key]);
        } 
    
        return None;
    }  
     
    
    
    pub fn get_from_int_map_128(&mut self, key: &String) -> Option<&i128> {
        if self.int_map_128.contains_key(&key) {
            return Some(&self.int_map_128[&key]);
        }  
    
        return None; 
    }  
*/    
    pub fn push_str_map(&mut self, key: &String, value: String) {
         self.str_map.insert(key.clone(), value);
    } 
    
    pub fn push_bool_map(&mut self, key: &String, value: bool) {
         self.bool_map.insert(key.clone(), value);
    } 
    
    pub fn push_int_map(&mut self, key: &String, value: i32) {
        self.int_map.insert(key.clone(), value);
    }  
    
    pub fn push_int_map_64(&mut self, key: &String, value: i64) {
        self.int_map_64.insert(key.clone(), value);
    }  

    /*     
    pub fn set_int_map_8(&mut self, key: String, value: i8) {
        self.int_map_8.insert(key, value);
    }  
     
    pub fn set_int_map_16(&mut self, key: String, value: i16) {
        self.int_map_16.insert(key, value);
    }  
     
    
         
    pub fn set_int_map_128(&mut self, key: String, value: i128) {
        self.int_map_128.insert(key, value);
    }
*/
}
