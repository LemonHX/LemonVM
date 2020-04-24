pub struct Reader {
    data: *const u8,
    pos: usize,
}
impl Reader {
    pub fn new(data: *const u8) -> Reader {
        Reader { data, pos: 0 }
    }

    pub fn read_byte(&mut self) -> u8 {
        unsafe {
            let b = *self.data.add(self.pos);
            self.pos += 1;
            b
        }
    }
    //TODO: use with gc
    pub fn read_bytes(&mut self, len: usize) -> *const u8 {
        unsafe {
            let buffer = std::alloc::alloc(std::alloc::Layout::new::<u8>());
            std::ptr::copy(self.data.add(self.pos), buffer, len);
            self.pos += len;
            buffer
        }
    }
    pub fn read_vm_char(&mut self) -> super::VMChar {
        let byte = self.read_bytes(std::mem::size_of::<super::VMChar>());
        unsafe { *(byte as *const super::VMChar) }
    }
    pub fn read_vm_int(&mut self) -> super::VMInt {
        let byte = self.read_bytes(std::mem::size_of::<super::VMInt>());
        unsafe { *(byte as *const super::VMInt) }
    }

    pub fn read_vm_number(&mut self) -> super::VMNum {
        let byte = self.read_bytes(std::mem::size_of::<super::VMNum>());
        unsafe { *(byte as *const super::VMNum) }
    }

    pub fn read_vm_symbol(&mut self) -> super::VMSym {
        self.read_vec(|f| f.read_vm_char())
    }

    pub fn read_vec<T, F>(&mut self, f: F) -> Vec<T>
    where
        F: Fn(&mut Reader) -> T,
    {
        let n = self.read_vm_int() as usize;
        let mut vec = Vec::with_capacity(n);
        for _i in 0..n {
            vec.push(f(self));
        }
        vec
    }
    pub fn read_header(&mut self) -> super::Header {
        unsafe {
            let sig =
                std::ptr::slice_from_raw_parts(self.read_bytes(super::SIG_LEN), super::SIG_LEN);
            let version = self.read_byte();
            let instruction_size = self.read_byte();
            let sizeof_vm_char = self.read_byte();
            let sizeof_vm_int = self.read_byte();
            let sizeof_vm_number = self.read_byte();
            super::Header {
                sig: super::clone_into_array(&*sig),
                version,
                instruction_size,
                sizeof_vm_char,
                sizeof_vm_int,
                sizeof_vm_number,
            }
        }
    }
    // assine value to global constant pool
    pub fn read_constant_pool(data: *const u8, len: usize) {
        let mut reader = Reader::new(data);
        while reader.pos != len {
            let uuid = reader.read_vm_symbol();
            let consts = reader.read_vec(|r| r.read_constant());
            super::CONSTANT_POOL.write().unwrap().pool.insert(uuid, consts);
        }
    }
    pub fn read_proto(&mut self) -> super::Prototype {
        super::Prototype {
            uuid: self.read_vm_symbol(),
            line_start: self.read_vm_int(),
            line_end: self.read_vm_int(),
            params: self.read_byte(),
            is_vargs: self.read_byte(),
            stack_size: self.read_byte(),
            instruction_table: self.read_vec(|r| r.read_vm_int()),
            // lex_constant: CONSTANT_POOL.read().unwrap(),
            closure_caps: self.read_vec(|r| r.read_closure_cap()),
            protos: self.read_vec(|r| r.read_proto()),
            debug_line_info: self.read_vec(|r| r.read_vm_int()),
            debug_local_variables: self.read_vec(|r| r.read_loc_var()),
            debug_closure_cap_names: self.read_vec(|r| r.read_vm_symbol()),
        }
    }

    pub fn read_constant(&mut self) -> super::Constant {
        let tag = self.read_byte();
        match tag {
            super::TAG_NULL => super::Constant::Null,
            super::TAG_BOOL => super::Constant::Bool(self.read_byte() != 0),
            super::TAG_CHAR => super::Constant::Char(self.read_vm_char()),
            super::TAG_INT => super::Constant::Int(self.read_vm_int()),
            super::TAG_NUM => super::Constant::Num(self.read_vm_number()),
            super::TAG_SYM => super::Constant::Sym(self.read_vm_symbol()),
            _ => panic!("corrupted!"),
        }
    }

    pub fn read_closure_cap(&mut self) -> super::ClosureCap {
        super::ClosureCap {
            instack: self.read_byte(),
            idx: self.read_byte(),
        }
    }

    pub fn read_loc_var(&mut self) -> super::LocalVar {
        super::LocalVar {
            name: self.read_vm_symbol(),
            start_pc: self.read_vm_int(),
            end_pc: self.read_vm_int(),
        }
    }
    pub fn read_binary_chunk(&mut self) -> super::BinaryChunk {
        super::BinaryChunk {
            header: self.read_header(),
            up_value_size: self.read_byte(),
            entry: self.read_proto(),
        }
    }
}