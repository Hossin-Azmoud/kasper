// STACK.


pub struct Stack<'a> {
    str_map: HashMap<String, String>,
    int_map_8: HashMap<String, i8>, // 8 bit..
    int_map_16: HashMap<String, i16>, // 16 bit..
    int_map: HashMap<String, i32>, // 32 bit..
    int_map_64: HashMap<String, i64>, // 64 bit..
    int_map_128: HashMap<String, i128>, // 128 bit..
    int_map_8_array: HashMap<String, Vec<i8>>, // 8 bit..
    int_map_16_array: HashMap<String, Vec<i16>>, // 16 bit..
    int_map_array: HashMap<String, Vec<i32>>, // 32 bit..
    int_map_64_array: HashMap<String, Vec<i64>>, // 64 bit..
   int_map_128_array: HashMap<String, Vec<i128>>, // 128 bit..                                
}
impl Stack {
    pub fn new() -> Self {
        Self {
            str_map:            HashMap::new(), 
            int_map_8:          HashMap::new(),
            int_map_16:         HashMap::new(),
            int_map:            HashMap::new(),
            int_map_64:         HashMap::new(),
            int_map_128:        HashMap::new(),
            int_map_8_array:    HashMap::new(),
            int_map_16_array:   HashMap::new(),
            int_map_array:      HashMap::new(), 
            int_map_64_array:   HashMap::new(),             
            int_map_128_array:  HashMap::new(),
        }
    }
    
    pub fn get_from_str_map(&mut self, key: String) -> Option<&String> {
        if self.str_map.contains_key(&key) {
            return Some(self.str_map[&key]);
        }

        return None; 
    }  
     
    pub fn get_from_int_map_8(&mut self, key: String) -> Option<&i8> {
        if self.int_map_8.contains_key(&key) {
            return Some(self.int_map_8[&key]);
        }

        return None; 
    }  
     
    pub fn get_from_int_map_16(&mut self, key: String) -> Option<&i16> {
        if self.int_map_16.contains_key(&key) {
            return Some(self.int_map_16[&key]);
        }       
    
       return None; 
    }  
     
    pub fn get_from_int_map(&mut self, key: String) -> Option<&i32> {
        if self.int_map.contains_key(&key) {
            return Some(self.int_map[&key]);
        }  

        return None; 
    }  
     
    pub fn get_from_int_map_64(&mut self, key: String) -> Option<&i64> {
        
        if self.int_map_64.contains_key(&key) {
            return Some(self.int_map_64[&key]);
        }  

        return None; 
    }  
     
    pub fn get_from_int_map_128(&mut self, key: String) -> Option<&i128> {
        if self.int_map_128.contains_key(&key) {
            return Some(self.int_map_128[&key]);
        }  
    
        return None; 
    }  
     
    pub fn get_from_int_map_8_array(&mut self, key: String) -> Option<&Vec<i8>> {
        if self.int_map_8_array.contains_key(&key) {
            return Some(&self.int_map_8_array[&key]);
        }
    
        return None; 
    }  
     
    pub fn get_from_int_map_16_array(&mut self, key: String) -> Option<&Vec<i16>> {
        if self.int_map_16_array.contains_key(&key) {
            return Some(&self.int_map_16_array[&key]);
        }
    
        return None; 
    }  
     
    pub fn get_from_int_map_array(&mut self, key: String) -> Option<&Vec<i32>> {
        if self.int_map.contains_key(&key) {
            return Some(&self.int_map[&key]);
        }
    
        return None; 
    }  
     
    pub fn get_from_int_map_64_array(&mut self, key: String) -> Option<&Vec<i64>> {
        if self.int_map_64_array.contains_key(&key) {
            return Some(&self.int_map_64_array[&key]);
        }

        return None; 
    }  
     
    pub fn get_from_int_map_128_array(&mut self, key: String) -> Option<&Vec<i128>> {
        if self.int_map_128_array.contains_key(&key) {
            return Some(&self.int_map_128_array[&key]);
        }
        
        return None; 
    }      
}






