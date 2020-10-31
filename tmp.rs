#![feature(prelude_import)]
#![cfg(feature = "frontend")]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use wasm_bindgen::prelude::*;
mod todo {
    use remote_attr::remote;
    use serde::{Serialize, Deserialize};
    #[cfg(feature = "frontend")]
    use wasm_bindgen::JsCast;
    pub struct Todo {
        pub id: u8,
        pub content: String,
        pub completed: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Todo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Todo {
                    id: ref __self_0_0,
                    content: ref __self_0_1,
                    completed: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Todo");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("content", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("completed", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Todo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Todo",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "content",
                    &self.content,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "completed",
                    &self.completed,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Todo {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::export::Ok(__Field::__field0),
                            "content" => _serde::export::Ok(__Field::__field1),
                            "completed" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::export::Ok(__Field::__field0),
                            b"content" => _serde::export::Ok(__Field::__field1),
                            b"completed" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Todo>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Todo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct Todo")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Todo with 3 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Todo with 3 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Todo with 3 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(Todo {
                            id: __field0,
                            content: __field1,
                            completed: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<u8> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<bool> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "content",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "completed",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("id") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("content") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("completed") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(Todo {
                            id: __field0,
                            content: __field1,
                            completed: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["id", "content", "completed"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Todo",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Todo>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    use wasm_bindgen::prelude::*;
    pub async fn get_todo() -> Todo {
        let window = web_sys::window().unwrap();
        console_error_panic_hook::set_once();
        let input_as_bytes: Vec<u8> = bincode::serialize(&()).unwrap();
        let mut opts = web_sys::RequestInit::new();
        opts.method("POST");
        opts.mode(web_sys::RequestMode::Cors);
        opts.body(&input_as_bytes);
        let request =
            web_sys::Request::new_with_str_and_init("http://localhost:3030/rpc", &opts).unwrap();
        let response: web_sys::Response =
            wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap()
                .dyn_into()
                .expect("Failed to cast to response.");
        let bin: Vec<u8> = wasm_bindgen_futures::JsFuture::from(response.text().unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap()
            .into_bytes();
        bincode::deserialize(&bin[..]).unwrap()
    }
    use wasm_bindgen::prelude::*;
    pub async fn add_todo(new_todo: &Todo) {
        let window = web_sys::window().unwrap();
        console_error_panic_hook::set_once();
        let input_as_bytes: Vec<u8> = bincode::serialize(&(new_todo)).unwrap();
        let mut opts = web_sys::RequestInit::new();
        opts.method("POST");
        opts.mode(web_sys::RequestMode::Cors);
        opts.body(&input_as_bytes);
        let request =
            web_sys::Request::new_with_str_and_init("http://localhost:3030/rpc", &opts).unwrap();
        let response: web_sys::Response =
            wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap()
                .dyn_into()
                .expect("Failed to cast to response.");
        let bin: Vec<u8> = wasm_bindgen_futures::JsFuture::from(response.text().unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap()
            .into_bytes();
        ()
    }
}
use todo::{Todo, get_todo, add_todo};
#[cfg(feature = "wee_alloc")]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
const _: () = {
    #[rustc_std_internal_symbol]
    unsafe fn __rg_alloc(arg0: usize, arg1: usize) -> *mut u8 {
        ::core::alloc::GlobalAlloc::alloc(
            &ALLOC,
            ::core::alloc::Layout::from_size_align_unchecked(arg0, arg1),
        ) as *mut u8
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rg_dealloc(arg0: *mut u8, arg1: usize, arg2: usize) -> () {
        ::core::alloc::GlobalAlloc::dealloc(
            &ALLOC,
            arg0 as *mut u8,
            ::core::alloc::Layout::from_size_align_unchecked(arg1, arg2),
        )
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rg_realloc(arg0: *mut u8, arg1: usize, arg2: usize, arg3: usize) -> *mut u8 {
        ::core::alloc::GlobalAlloc::realloc(
            &ALLOC,
            arg0 as *mut u8,
            ::core::alloc::Layout::from_size_align_unchecked(arg1, arg2),
            arg3,
        ) as *mut u8
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rg_alloc_zeroed(arg0: usize, arg1: usize) -> *mut u8 {
        ::core::alloc::GlobalAlloc::alloc_zeroed(
            &ALLOC,
            ::core::alloc::Layout::from_size_align_unchecked(arg0, arg1),
        ) as *mut u8
    }
};
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
pub async fn do_get_todo() -> JsValue {
    let todo: Todo = get_todo().await;
    JsValue::from_serde(&todo).unwrap()
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
pub extern "C" fn __wasm_bindgen_generated_do_get_todo(
) -> <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret = { do_get_todo() };
    <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
        wasm_bindgen_futures::future_to_promise(async move {
            <JsValue as wasm_bindgen::__rt::IntoJsResult>::into_js_result(_ret.await)
        })
        .into(),
    )
}
pub async fn do_add_todo(content: JsValue) {
    let todo: Todo = content.into_serde().unwrap();
    add_todo(&todo).await;
}
#[allow(non_snake_case)]
#[allow(clippy::all)]
pub extern "C" fn __wasm_bindgen_generated_do_add_todo(
    arg0: <JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi,
) -> <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret = {
        let arg0 = unsafe { <JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0) };
        do_add_todo(arg0)
    };
    <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
        wasm_bindgen_futures::future_to_promise(async move {
            <() as wasm_bindgen::__rt::IntoJsResult>::into_js_result(_ret.await)
        })
        .into(),
    )
}
ve {
            <() as wasm_bindgen::__rt::IntoJsResult>::into_js_result(_ret.await)
        })
        .into(),
    )
}
