use crate::value::object::JSObjAddr;

/// 6.1.7.4 Well-Known Intrinsic Objects
/// https://262.ecma-international.org/15.0/#sec-well-known-intrinsic-objects
#[derive(Debug, Default)]
pub struct Intrinsics {
    // %AggregateError%
    pub aggregate_error: Option<JSObjAddr>,
    // %Array%
    pub array: Option<JSObjAddr>,
    // %ArrayBuffer%
    pub array_buffer: Option<JSObjAddr>,
    // %ArrayIteratorPrototype%
    pub array_iterator_prototype: Option<JSObjAddr>,
    // %AsyncFromSyncIteratorPrototype%
    pub async_from_sync_iterator_prototype: Option<JSObjAddr>,
    // %AsyncFunction%
    pub async_function: Option<JSObjAddr>,
    // %AsyncGeneratorFunction%
    pub async_generator_function: Option<JSObjAddr>,
    // %AsyncGeneratorPrototype%
    pub async_generator_prototype: Option<JSObjAddr>,
    // %AsyncIteratorPrototype%
    pub async_iterator_prototype: Option<JSObjAddr>,
    // %Atomics%
    pub atomics: Option<JSObjAddr>,
    // %BigInt%
    pub big_int: Option<JSObjAddr>,
    // %BigInt64Array%
    pub big_int64_array: Option<JSObjAddr>,
    // %BigUint64Array%
    pub big_uint64_array: Option<JSObjAddr>,
    // %Boolean%
    pub boolean: Option<JSObjAddr>,
    // %DataView%
    pub data_view: Option<JSObjAddr>,
    // %Date%
    pub date: Option<JSObjAddr>,
    // %decodeURI%
    pub decode_uri: Option<JSObjAddr>,
    // %decodeURIComponent%
    pub decode_uri_component: Option<JSObjAddr>,
    // %encodeURI%
    pub encode_uri: Option<JSObjAddr>,
    // %encodeURIComponent%
    pub encode_uri_component: Option<JSObjAddr>,
    // %Error%
    pub error: Option<JSObjAddr>,
    // %eval%
    pub eval: Option<JSObjAddr>,
    // %EvalError%
    pub eval_error: Option<JSObjAddr>,
    // %FinalizationRegistry%
    pub finalization_registry: Option<JSObjAddr>,
    // %Float16Array%
    pub float16_array: Option<JSObjAddr>,
    // %Float32Array%
    pub float32_array: Option<JSObjAddr>,
    // %Float64Array%
    pub float64_array: Option<JSObjAddr>,
    // %ForInIteratorPrototype%
    pub for_in_iterator_prototype: Option<JSObjAddr>,
    // %Function%
    pub function: Option<JSObjAddr>,
    // %GeneratorFunction%
    pub generator_function: Option<JSObjAddr>,
    // %GeneratorPrototype%
    pub generator_prototype: Option<JSObjAddr>,
    // %Int8Array%
    pub int8_array: Option<JSObjAddr>,
    // %Int16Array%
    pub int16_array: Option<JSObjAddr>,
    // %Int32Array%
    pub int32_array: Option<JSObjAddr>,
    // %isFinite%
    pub is_finite: Option<JSObjAddr>,
    // %isNaN%
    pub is_nan: Option<JSObjAddr>,
    // %Iterator%
    pub iterator: Option<JSObjAddr>,
    // %IteratorHelperPrototype%
    pub iterator_helper_prototype: Option<JSObjAddr>,
    // %JSON%
    pub json: Option<JSObjAddr>,
    // %Map%
    pub map: Option<JSObjAddr>,
    // %MapIteratorPrototype%
    pub map_iterator_prototype: Option<JSObjAddr>,
    // %Math%
    pub math: Option<JSObjAddr>,
    // %Number%
    pub number: Option<JSObjAddr>,
    // %Object%
    pub object: Option<JSObjAddr>,
    // %parseFloat%
    pub parse_float: Option<JSObjAddr>,
    // %parseInt%
    pub parse_int: Option<JSObjAddr>,
    // %Promise%
    pub promise: Option<JSObjAddr>,
    // %Proxy%
    pub proxy: Option<JSObjAddr>,
    // %RangeError%
    pub range_error: Option<JSObjAddr>,
    // %ReferenceError%
    pub reference_error: Option<JSObjAddr>,
    // %Reflect%
    pub reflect: Option<JSObjAddr>,
    // %RegExp%
    pub reg_exp: Option<JSObjAddr>,
    // %RegExpStringIteratorPrototype%
    pub reg_exp_string_iterator_prototype: Option<JSObjAddr>,
    // %Set%
    pub set: Option<JSObjAddr>,
    // %SetIteratorPrototype%
    pub set_iterator_prototype: Option<JSObjAddr>,
    // %SharedArrayBuffer%
    pub shared_array_buffer: Option<JSObjAddr>,
    // %String%
    pub string: Option<JSObjAddr>,
    // %StringIteratorPrototype%
    pub string_iterator_prototype: Option<JSObjAddr>,
    // %Symbol%
    pub symbol: Option<JSObjAddr>,
    // %SyntaxError%
    pub syntax_error: Option<JSObjAddr>,
    // %ThrowTypeError%
    pub throw_type_error: Option<JSObjAddr>,
    // %TypedArray%
    pub typed_array: Option<JSObjAddr>,
    // %TypeError%
    pub type_error: Option<JSObjAddr>,
    // %Uint8Array%
    pub uint8_array: Option<JSObjAddr>,
    // %Uint8ClampedArray%
    pub uint8_clamped_array: Option<JSObjAddr>,
    // %Uint16Array%
    pub uint16_array: Option<JSObjAddr>,
    // %Uint32Array%
    pub uint32_array: Option<JSObjAddr>,
    // %URIError%
    pub uri_error: Option<JSObjAddr>,
    // %WeakMap%
    pub weak_map: Option<JSObjAddr>,
    // %WeakRef%
    pub weak_ref: Option<JSObjAddr>,
    // %WeakSet%
    pub weak_set: Option<JSObjAddr>,
    // %WrapForValidIteratorPrototype%
    pub wrap_for_valid_iterator_prototype: Option<JSObjAddr>,
    // %AggregateError.prototype%
    pub aggregate_error_prototype: Option<JSObjAddr>,
    // %Array.prototype.values%
    pub array_prototype_values: Option<JSObjAddr>,
    // %Array.prototype%
    pub array_prototype: Option<JSObjAddr>,
    // %ArrayBuffer.prototype%
    pub array_buffer_prototype: Option<JSObjAddr>,
    // %AsyncFunction.prototype%
    pub async_function_prototype: Option<JSObjAddr>,
    // %AsyncGeneratorFunction.prototype.prototype%
    pub async_generator_function_prototype_prototype: Option<JSObjAddr>,
    // %AsyncGeneratorFunction.prototype%
    pub async_generator_function_prototype: Option<JSObjAddr>,
    // %BigInt.prototype%
    pub big_int_prototype: Option<JSObjAddr>,
    // %BigInt64Array.prototype%
    pub big_int64_array_prototype: Option<JSObjAddr>,
    // %BigUint64Array.prototype%
    pub big_uint64_array_prototype: Option<JSObjAddr>,
    // %Boolean.prototype%
    pub boolean_prototype: Option<JSObjAddr>,
    // %DataView.prototype%
    pub data_view_prototype: Option<JSObjAddr>,
    // %Date.prototype%
    pub date_prototype: Option<JSObjAddr>,
    // %Error.prototype%
    pub error_prototype: Option<JSObjAddr>,
    // %Error.prototype.toString%
    pub error_prototype_to_string: Option<JSObjAddr>,
    // %EvalError.prototype%
    pub eval_error_prototype: Option<JSObjAddr>,
    // %FinalizationRegistry.prototype%
    pub finalization_registry_prototype: Option<JSObjAddr>,
    // %Float32Array.prototype%
    pub float32_array_prototype: Option<JSObjAddr>,
    // %Float64Array.prototype%
    pub float64_array_prototype: Option<JSObjAddr>,
    // %Function.prototype%
    pub function_prototype: Option<JSObjAddr>,
    // %GeneratorFunction.prototype.prototype.next%
    pub generator_function_prototype_prototype_next: Option<JSObjAddr>,
    // %GeneratorFunction.prototype.prototype%
    pub generator_function_prototype_prototype: Option<JSObjAddr>,
    // %GeneratorFunction.prototype%
    pub generator_function_prototype: Option<JSObjAddr>,
    // %Int16Array.prototype%
    pub int16_array_prototype: Option<JSObjAddr>,
    // %Int32Array.prototype%
    pub int32_array_prototype: Option<JSObjAddr>,
    // %Int8Array.prototype%
    pub int8_array_prototype: Option<JSObjAddr>,
    // %Iterator.prototype%
    pub iterator_prototype: Option<JSObjAddr>,
    // %JSON.parse%
    pub json_parse: Option<JSObjAddr>,
    // %JSON.stringify%
    pub json_stringify: Option<JSObjAddr>,
    // %Map.prototype%
    pub map_prototype: Option<JSObjAddr>,
    // %Number.prototype%
    pub number_prototype: Option<JSObjAddr>,
    // %Object.prototype.toString%
    pub object_prototype_to_string: Option<JSObjAddr>,
    // %Object.prototype.valueOf%
    pub object_prototype_value_of: Option<JSObjAddr>,
    // %Object.prototype%
    pub object_prototype: Option<JSObjAddr>,
    // %Promise.prototype.then%
    pub promise_prototype_then: Option<JSObjAddr>,
    // %Promise.prototype%
    pub promise_prototype: Option<JSObjAddr>,
    // %Promise.resolve%
    pub promise_resolve: Option<JSObjAddr>,
    // %RangeError.prototype%
    pub range_error_prototype: Option<JSObjAddr>,
    // %ReferenceError.prototype%
    pub reference_error_prototype: Option<JSObjAddr>,
    // %RegExp.prototype%
    pub reg_exp_prototype: Option<JSObjAddr>,
    // %Set.prototype%
    pub set_prototype: Option<JSObjAddr>,
    // %String.prototype%
    pub string_prototype: Option<JSObjAddr>,
    // %Symbol.prototype%
    pub symbol_prototype: Option<JSObjAddr>,
    // %SyntaxError.prototype%
    pub syntax_error_prototype: Option<JSObjAddr>,
    // %TypedArray.prototype%
    pub typed_array_prototype: Option<JSObjAddr>,
    // %TypeError.prototype%
    pub type_error_prototype: Option<JSObjAddr>,
    // %Uint16Array.prototype%
    pub uint16_array_prototype: Option<JSObjAddr>,
    // %Uint32Array.prototype%
    pub uint32_array_prototype: Option<JSObjAddr>,
    // %Uint8Array.prototype%
    pub uint8_array_prototype: Option<JSObjAddr>,
    // %Uint8ClampedArray.prototype%
    pub uint8_clamped_array_prototype: Option<JSObjAddr>,
    // %URIError.prototype%
    pub uri_error_prototype: Option<JSObjAddr>,
    // %WeakMap.prototype%
    pub weak_map_prototype: Option<JSObjAddr>,
    // %WeakRef.prototype%
    pub weak_ref_prototype: Option<JSObjAddr>,
    // %WeakSet.prototype%
    pub weak_set_prototype: Option<JSObjAddr>,
}
