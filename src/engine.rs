use rustc_hash::FxHashMap;

trait AllowedExportTypes {}

impl AllowedExportTypes for u8 {}
impl AllowedExportTypes for u16 {}
impl AllowedExportTypes for u32 {}
impl AllowedExportTypes for usize {}

trait Export<T>
where
    T: AllowedExportTypes,
{
    fn from_data(data: Data) -> DataExport<T>;
    fn to_data(self) -> Data;

    fn from_bytes(bytes: &[u8]) -> DataExport<T>;
    fn to_bytes(self) -> Vec<u8>;
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct DataExport<T>
where
    T: AllowedExportTypes,
{
    pub length: usize,
    tokens: FxHashMap<String, Vec<T>>,
    extra_separators: FxHashMap<usize, T>,
}

impl Export<u8> for DataExport<u8> {
    fn from_data(data: Data) -> DataExport<u8> {
        let length = data.length;
        let mut tokens = FxHashMap::<String, Vec<u8>>::default();
        let mut extra_separators: FxHashMap<usize, u8> = FxHashMap::default();

        for (k, v) in data.tokens {
            let mut arr: Vec<u8> = Vec::new();

            for i in v {
                arr.push(i as u8);
            }

            tokens.insert(k, arr);
        }

        for (k, v) in data.extra_separators {
            extra_separators.insert(k, v as u8);
        }

        return DataExport {
            length,
            tokens,
            extra_separators,
        };
    }

    fn to_data(self) -> Data {
        let length = self.length;
        let mut tokens = FxHashMap::<String, Vec<usize>>::default();
        let mut extra_separators: FxHashMap<usize, u16> = FxHashMap::default();

        for (k, v) in self.tokens {
            let mut arr: Vec<usize> = Vec::new();

            for i in v {
                arr.push(i as usize);
            }

            tokens.insert(k, arr);
        }

        for (k, v) in self.extra_separators {
            extra_separators.insert(k, v as u16);
        }

        return Data {
            length,
            tokens,
            extra_separators,
        };
    }

    fn from_bytes(bytes: &[u8]) -> DataExport<u8> {
        return bincode::deserialize(bytes).unwrap();
    }

    fn to_bytes(self) -> Vec<u8> {
        return bincode::serialize(&self).unwrap();
    }
}

impl Export<u16> for DataExport<u16> {
    fn from_data(data: Data) -> DataExport<u16> {
        let length = data.length;
        let mut tokens = FxHashMap::<String, Vec<u16>>::default();
        let mut extra_separators: FxHashMap<usize, u16> = FxHashMap::default();

        for (k, v) in data.tokens {
            let mut arr: Vec<u16> = Vec::new();

            for i in v {
                arr.push(i as u16);
            }

            tokens.insert(k, arr);
        }

        for (k, v) in data.extra_separators {
            extra_separators.insert(k, v as u16);
        }

        return DataExport {
            length,
            tokens,
            extra_separators,
        };
    }

    fn to_data(self) -> Data {
        let length = self.length;
        let mut tokens = FxHashMap::<String, Vec<usize>>::default();
        let mut extra_separators: FxHashMap<usize, u16> = FxHashMap::default();

        for (k, v) in self.tokens {
            let mut arr: Vec<usize> = Vec::new();

            for i in v {
                arr.push(i as usize);
            }

            tokens.insert(k, arr);
        }

        for (k, v) in self.extra_separators {
            extra_separators.insert(k, v as u16);
        }

        return Data {
            length,
            tokens,
            extra_separators,
        };
    }

    fn from_bytes(bytes: &[u8]) -> DataExport<u16> {
        return bincode::deserialize(bytes).unwrap();
    }

    fn to_bytes(self) -> Vec<u8> {
        return bincode::serialize(&self).unwrap();
    }
}

impl Export<u32> for DataExport<u32> {
    fn from_data(data: Data) -> DataExport<u32> {
        let length = data.length;
        let mut tokens = FxHashMap::<String, Vec<u32>>::default();
        let mut extra_separators: FxHashMap<usize, u32> = FxHashMap::default();

        for (k, v) in data.tokens {
            let mut arr: Vec<u32> = Vec::new();

            for i in v {
                arr.push(i as u32);
            }

            tokens.insert(k, arr);
        }

        for (k, v) in data.extra_separators {
            extra_separators.insert(k, v as u32);
        }

        return DataExport {
            length,
            tokens,
            extra_separators,
        };
    }

    fn to_data(self) -> Data {
        let length = self.length;
        let mut tokens = FxHashMap::<String, Vec<usize>>::default();
        let mut extra_separators: FxHashMap<usize, u16> = FxHashMap::default();

        for (k, v) in self.tokens {
            let mut arr: Vec<usize> = Vec::new();

            for i in v {
                arr.push(i as usize);
            }

            tokens.insert(k, arr);
        }

        for (k, v) in self.extra_separators {
            extra_separators.insert(k, v as u16);
        }

        return Data {
            length,
            tokens,
            extra_separators,
        };
    }

    fn from_bytes(bytes: &[u8]) -> DataExport<u32> {
        return bincode::deserialize(bytes).unwrap();
    }

    fn to_bytes(self) -> Vec<u8> {
        return bincode::serialize(&self).unwrap();
    }
}

impl Export<usize> for DataExport<usize> {
    fn from_data(data: Data) -> DataExport<usize> {
        let length = data.length;
        let mut tokens = FxHashMap::<String, Vec<usize>>::default();
        let mut extra_separators: FxHashMap<usize, usize> = FxHashMap::default();

        for (k, v) in data.tokens {
            let mut arr: Vec<usize> = Vec::new();

            for i in v {
                arr.push(i);
            }

            tokens.insert(k, arr);
        }

        for (k, v) in data.extra_separators {
            extra_separators.insert(k, v as usize);
        }

        return DataExport {
            length,
            tokens,
            extra_separators,
        };
    }

    fn to_data(self) -> Data {
        let length = self.length;
        let mut tokens = FxHashMap::<String, Vec<usize>>::default();
        let mut extra_separators: FxHashMap<usize, u16> = FxHashMap::default();

        for (k, v) in self.tokens {
            let mut arr: Vec<usize> = Vec::new();

            for i in v {
                arr.push(i as usize);
            }

            tokens.insert(k, arr);
        }

        for (k, v) in self.extra_separators {
            extra_separators.insert(k, v as u16);
        }

        return Data {
            length,
            tokens,
            extra_separators,
        };
    }

    fn from_bytes(bytes: &[u8]) -> DataExport<usize> {
        return bincode::deserialize(bytes).unwrap();
    }

    fn to_bytes(self) -> Vec<u8> {
        return bincode::serialize(&self).unwrap();
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Data {
    pub length: usize,
    tokens: FxHashMap<String, Vec<usize>>,
    extra_separators: FxHashMap<usize, u16>,
}

impl Data {
    pub fn new() -> Data {
        return Data {
            length: 0,
            tokens: FxHashMap::default(),
            extra_separators: FxHashMap::default(),
        };
    }

  
    pub fn add_element(&mut self, s: &String) {
        let index = self.length;

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
        self.extra_separators.insert(index, count as u16);
        self.length += 1;
    }

    pub fn from_bytes(bytes: &[u8]) -> Data {
        let prefix = *bytes.get(0).unwrap();

        if prefix == 0 {
            return DataExport::<u8>::from_bytes(&bytes[1..]).to_data();
        } else if prefix == 1 {
            return DataExport::<u16>::from_bytes(&bytes[1..]).to_data();
        } else if prefix == 2 {
            return DataExport::<u32>::from_bytes(&bytes[1..]).to_data();
        } else if prefix == 3 {
            return DataExport::<usize>::from_bytes(&bytes[1..]).to_data();
        } else {
            panic!("Not valid prefix in file!");
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        //println!("COMPRESS DATA: {:?}", self);
        println!("TOKENS: {}", self.length);

        if self.length < u8::MAX as usize {
            println!("COMPRESS WITH U8: {} < {}", self.length, u8::MAX);
            let mut prefix = Vec::<u8>::from([0]);
            let data = DataExport::<u8>::from_data(self);
            let bytes = bincode::serialize(&data).unwrap();

            prefix.extend(bytes);
            return prefix;
        } else if self.length < u16::MAX as usize {
            println!("COMPRESS WITH U16: {} < {}", self.length, u16::MAX);
            let mut prefix = Vec::<u8>::from([1]);
            let data = DataExport::<u16>::from_data(self);
            let bytes = bincode::serialize(&data).unwrap();
            prefix.extend(bytes);
            return prefix;
        } else if self.length < u32::MAX as usize {
            println!("COMPRESS WITH U32: {} < {}", self.length, u32::MAX);

            let mut prefix = Vec::<u8>::from([2]);
            let data = DataExport::<u32>::from_data(self);
            let bytes = bincode::serialize(&data).unwrap();
            prefix.extend(bytes);
            return prefix;
        } else {
            println!("COMPRESS WITH USIZE: {} < {}", self.length, usize::MAX);

            let mut prefix = Vec::<u8>::from([3]);
            let data = DataExport::<usize>::from_data(self);
            let bytes = bincode::serialize(&data).unwrap();
            prefix.extend(bytes);
            return prefix;
        };
    }

    pub fn decompress(self) -> String {
        let mut data = Vec::<String>::new();
        data.reserve(self.length);

        for i in 0..self.length {
            if self.extra_separators.contains_key(&i) {
                let value = self.extra_separators.get(&i).unwrap();

                println!("AS USIZE: {}", *value as usize);
                for _ in 0..(*value as usize - 1) {
                    data.push("".to_string());
                }
            } else {
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
        }

        println!("TOKENS: {}", self.length);
        //println!("DECOMPRESS DATA: {:?}", self);
        return data.join(" ");
    }
}
