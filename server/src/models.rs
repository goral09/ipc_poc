// This file is generated by rust-protobuf 3.0.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]

#[derive(PartialEq,Clone,Default)]
#[derive(Serialize, Deserialize)]
pub struct Person {
    // message fields
    pub name: ::std::string::String,
    pub age: u32,
    // special fields
    #[serde(skip)]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[serde(skip)]
    pub cached_size: ::protobuf::CachedSize,
}

impl Person {
    pub fn new() -> Person {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for Person {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.age = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.age != 0 {
            my_size += ::protobuf::rt::value_size(2, self.age, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.age != 0 {
            os.write_uint32(2, self.age)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Person {
        Person::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::rt::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Person| { &m.name },
                |m: &mut Person| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::rt::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "age",
                |m: &Person| { &m.age },
                |m: &mut Person| { &mut m.age },
            ));
            ::protobuf::reflect::MessageDescriptor::new::<Person>(
                "Person",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Person {
        static instance: ::protobuf::rt::Lazy<Person> = ::protobuf::rt::Lazy::INIT;
        instance.get(Person::new)
    }
}

impl ::protobuf::Clear for Person {
    fn clear(&mut self) {
        self.name.clear();
        self.age = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Person {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Person {
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cmodels.proto\".\n\x06Person\x12\x12\n\x04name\x18\x01\x20\x01(\tR\
    \x04name\x12\x10\n\x03age\x18\x02\x20\x01(\rR\x03ageb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
