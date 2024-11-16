use rustc_hash::FxHashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Data
{
    pub length: usize,
    tokens: FxHashMap<String, Vec<u32>>,
    extra_separators: FxHashMap<usize, u8>,
}

impl Data
{
    pub fn new() -> Data {
        return Data{length: 0, tokens: FxHashMap::default(), extra_separators: FxHashMap::default()};
    }

    pub fn add_element(&mut self, s: &String) {
        let index = self.length as u32;

        if index > 128 {
            println!("TOKEN: {}", s);
        }
        if self.tokens.contains_key(s) {
            let elem = self.tokens.get_mut(s).unwrap();
            elem.push(index);
        } else {
            self.tokens.insert(s.clone(), Vec::from([index]));
        }
        self.length += 1;
    }

    pub fn add_separator(&mut self, count: usize) {
        let index = self.length;
        self.extra_separators.insert(index, count as u8);
        self.length += count;
    }

    pub fn compress(self) -> Vec<u8> {
        //println!("COMPRESS DATA: {:?}", self);

        return bincode::serialize(&self).unwrap();
    }

    pub fn decompress(self) -> String {
        let mut data = Vec::<String>::new();
        data.reserve(self.length);

        for i in 0..self.length {
            let mut flag = false;
                for (k, v) in self.tokens.clone() {
                    for index in v {
                        if i == index as usize {
                            data.push(k);
                            flag = true;
                            break;
                        } 
                    }
                    if flag {
                        break;
                    }
                }
        }

        println!("TOKENS: {}", self.length);
        println!("DECOMPRESS DATA: {:?}", self);
        return data.join(" ");
    }
}