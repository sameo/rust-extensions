// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `github.com/containerd/containerd/api/types/mount.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct Mount {
    // message fields
    pub field_type: ::std::string::String,
    pub source: ::std::string::String,
    pub target: ::std::string::String,
    pub options: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Mount {
    fn default() -> &'a Mount {
        <Mount as ::protobuf::Message>::default_instance()
    }
}

impl Mount {
    pub fn new() -> Mount {
        ::std::default::Default::default()
    }

    // string type = 1;


    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    // string source = 2;


    pub fn get_source(&self) -> &str {
        &self.source
    }
    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut ::std::string::String {
        &mut self.source
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.source, ::std::string::String::new())
    }

    // string target = 3;


    pub fn get_target(&self) -> &str {
        &self.target
    }
    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        &mut self.target
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.target, ::std::string::String::new())
    }

    // repeated string options = 4;


    pub fn get_options(&self) -> &[::std::string::String] {
        &self.options
    }
    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Mount {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.source)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.target)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.options)?;
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
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        }
        if !self.source.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.source);
        }
        if !self.target.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.target);
        }
        for value in &self.options {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
        }
        if !self.source.is_empty() {
            os.write_string(2, &self.source)?;
        }
        if !self.target.is_empty() {
            os.write_string(3, &self.target)?;
        }
        for v in &self.options {
            os.write_string(4, &v)?;
        };
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Mount {
        Mount::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "type",
                |m: &Mount| { &m.field_type },
                |m: &mut Mount| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "source",
                |m: &Mount| { &m.source },
                |m: &mut Mount| { &mut m.source },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "target",
                |m: &Mount| { &m.target },
                |m: &mut Mount| { &mut m.target },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "options",
                |m: &Mount| { &m.options },
                |m: &mut Mount| { &mut m.options },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Mount>(
                "Mount",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Mount {
        static instance: ::protobuf::rt::LazyV2<Mount> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Mount::new)
    }
}

impl ::protobuf::Clear for Mount {
    fn clear(&mut self) {
        self.field_type.clear();
        self.source.clear();
        self.target.clear();
        self.options.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mount {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n6github.com/containerd/containerd/api/types/mount.proto\x12\x10contain\
    erd.types\x1a\x14gogoproto/gogo.protoX\0\"o\n\x05Mount\x12\x14\n\x04type\
    \x18\x01\x20\x01(\tR\x04typeB\0\x12\x18\n\x06source\x18\x02\x20\x01(\tR\
    \x06sourceB\0\x12\x18\n\x06target\x18\x03\x20\x01(\tR\x06targetB\0\x12\
    \x1a\n\x07options\x18\x04\x20\x03(\tR\x07optionsB\0:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}