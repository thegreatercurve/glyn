use crate::value::object::JSObjAddr;

/// 6.1.7.4 Well-Known Intrinsic Objects
/// https://262.ecma-international.org/16.0/#sec-well-known-intrinsic-objects
#[derive(Debug, Default)]
pub(crate) struct Intrinsics {
    // %AggregateError%
    pub(crate) aggregate_error: Option<JSObjAddr>,
    // %Array%
    pub(crate) array: Option<JSObjAddr>,
    // %ArrayBuffer%
    pub(crate) array_buffer: Option<JSObjAddr>,
    // %ArrayIteratorPrototype%
    pub(crate) array_iterator_prototype: Option<JSObjAddr>,
    // %AsyncFromSyncIteratorPrototype%
    pub(crate) async_from_sync_iterator_prototype: Option<JSObjAddr>,
    // %AsyncFunction%
    pub(crate) async_function: Option<JSObjAddr>,
    // %AsyncGeneratorFunction%
    pub(crate) async_generator_function: Option<JSObjAddr>,
    // %AsyncGeneratorPrototype%
    pub(crate) async_generator_prototype: Option<JSObjAddr>,
    // %AsyncIteratorPrototype%
    pub(crate) async_iterator_prototype: Option<JSObjAddr>,
    // %Atomics%
    pub(crate) atomics: Option<JSObjAddr>,
    // %BigInt%
    pub(crate) big_int: Option<JSObjAddr>,
    // %BigInt64Array%
    pub(crate) big_int64_array: Option<JSObjAddr>,
    // %BigUint64Array%
    pub(crate) big_uint64_array: Option<JSObjAddr>,
    // %Boolean%
    pub(crate) boolean: Option<JSObjAddr>,
    // %DataView%
    pub(crate) data_view: Option<JSObjAddr>,
    // %Date%
    pub(crate) date: Option<JSObjAddr>,
    // %decodeURI%
    pub(crate) decode_uri: Option<JSObjAddr>,
    // %decodeURIComponent%
    pub(crate) decode_uri_component: Option<JSObjAddr>,
    // %encodeURI%
    pub(crate) encode_uri: Option<JSObjAddr>,
    // %encodeURIComponent%
    pub(crate) encode_uri_component: Option<JSObjAddr>,
    // %Error%
    pub(crate) error: Option<JSObjAddr>,
    // %eval%
    pub(crate) eval: Option<JSObjAddr>,
    // %EvalError%
    pub(crate) eval_error: Option<JSObjAddr>,
    // %FinalizationRegistry%
    pub(crate) finalization_registry: Option<JSObjAddr>,
    // %Float16Array%
    pub(crate) float16_array: Option<JSObjAddr>,
    // %Float32Array%
    pub(crate) float32_array: Option<JSObjAddr>,
    // %Float64Array%
    pub(crate) float64_array: Option<JSObjAddr>,
    // %ForInIteratorPrototype%
    pub(crate) for_in_iterator_prototype: Option<JSObjAddr>,
    // %Function%
    pub(crate) function: Option<JSObjAddr>,
    // %GeneratorFunction%
    pub(crate) generator_function: Option<JSObjAddr>,
    // %GeneratorPrototype%
    pub(crate) generator_prototype: Option<JSObjAddr>,
    // %Int8Array%
    pub(crate) int8_array: Option<JSObjAddr>,
    // %Int16Array%
    pub(crate) int16_array: Option<JSObjAddr>,
    // %Int32Array%
    pub(crate) int32_array: Option<JSObjAddr>,
    // %isFinite%
    pub(crate) is_finite: Option<JSObjAddr>,
    // %isNaN%
    pub(crate) is_nan: Option<JSObjAddr>,
    // %Iterator%
    pub(crate) iterator: Option<JSObjAddr>,
    // %IteratorHelperPrototype%
    pub(crate) iterator_helper_prototype: Option<JSObjAddr>,
    // %JSON%
    pub(crate) json: Option<JSObjAddr>,
    // %Map%
    pub(crate) map: Option<JSObjAddr>,
    // %MapIteratorPrototype%
    pub(crate) map_iterator_prototype: Option<JSObjAddr>,
    // %Math%
    pub(crate) math: Option<JSObjAddr>,
    // %Number%
    pub(crate) number: Option<JSObjAddr>,
    // %Object%
    pub(crate) object: Option<JSObjAddr>,
    // %parseFloat%
    pub(crate) parse_float: Option<JSObjAddr>,
    // %parseInt%
    pub(crate) parse_int: Option<JSObjAddr>,
    // %Promise%
    pub(crate) promise: Option<JSObjAddr>,
    // %Proxy%
    pub(crate) proxy: Option<JSObjAddr>,
    // %RangeError%
    pub(crate) range_error: Option<JSObjAddr>,
    // %ReferenceError%
    pub(crate) reference_error: Option<JSObjAddr>,
    // %Reflect%
    pub(crate) reflect: Option<JSObjAddr>,
    // %RegExp%
    pub(crate) reg_exp: Option<JSObjAddr>,
    // %RegExpStringIteratorPrototype%
    pub(crate) reg_exp_string_iterator_prototype: Option<JSObjAddr>,
    // %Set%
    pub(crate) set: Option<JSObjAddr>,
    // %SetIteratorPrototype%
    pub(crate) set_iterator_prototype: Option<JSObjAddr>,
    // %SharedArrayBuffer%
    pub(crate) shared_array_buffer: Option<JSObjAddr>,
    // %String%
    pub(crate) string: Option<JSObjAddr>,
    // %StringIteratorPrototype%
    pub(crate) string_iterator_prototype: Option<JSObjAddr>,
    // %Symbol%
    pub(crate) symbol: Option<JSObjAddr>,
    // %SyntaxError%
    pub(crate) syntax_error: Option<JSObjAddr>,
    // %ThrowTypeError%
    pub(crate) throw_type_error: Option<JSObjAddr>,
    // %TypedArray%
    pub(crate) typed_array: Option<JSObjAddr>,
    // %TypeError%
    pub(crate) type_error: Option<JSObjAddr>,
    // %Uint8Array%
    pub(crate) uint8_array: Option<JSObjAddr>,
    // %Uint8ClampedArray%
    pub(crate) uint8_clamped_array: Option<JSObjAddr>,
    // %Uint16Array%
    pub(crate) uint16_array: Option<JSObjAddr>,
    // %Uint32Array%
    pub(crate) uint32_array: Option<JSObjAddr>,
    // %URIError%
    pub(crate) uri_error: Option<JSObjAddr>,
    // %WeakMap%
    pub(crate) weak_map: Option<JSObjAddr>,
    // %WeakRef%
    pub(crate) weak_ref: Option<JSObjAddr>,
    // %WeakSet%
    pub(crate) weak_set: Option<JSObjAddr>,
    // %WrapForValidIteratorPrototype%
    pub(crate) wrap_for_valid_iterator_prototype: Option<JSObjAddr>,
    // %AggregateError.prototype%
    pub(crate) aggregate_error_prototype: Option<JSObjAddr>,
    // %Array.prototype.values%
    pub(crate) array_prototype_values: Option<JSObjAddr>,
    // %Array.prototype%
    pub(crate) array_prototype: Option<JSObjAddr>,
    // %ArrayBuffer.prototype%
    pub(crate) array_buffer_prototype: Option<JSObjAddr>,
    // %AsyncFunction.prototype%
    pub(crate) async_function_prototype: Option<JSObjAddr>,
    // %AsyncGeneratorFunction.prototype.prototype%
    pub(crate) async_generator_function_prototype_prototype: Option<JSObjAddr>,
    // %AsyncGeneratorFunction.prototype%
    pub(crate) async_generator_function_prototype: Option<JSObjAddr>,
    // %BigInt.prototype%
    pub(crate) big_int_prototype: Option<JSObjAddr>,
    // %BigInt64Array.prototype%
    pub(crate) big_int64_array_prototype: Option<JSObjAddr>,
    // %BigUint64Array.prototype%
    pub(crate) big_uint64_array_prototype: Option<JSObjAddr>,
    // %Boolean.prototype%
    pub(crate) boolean_prototype: Option<JSObjAddr>,
    // %DataView.prototype%
    pub(crate) data_view_prototype: Option<JSObjAddr>,
    // %Date.prototype%
    pub(crate) date_prototype: Option<JSObjAddr>,
    // %Error.prototype%
    pub(crate) error_prototype: Option<JSObjAddr>,
    // %Error.prototype.toString%
    pub(crate) error_prototype_to_string: Option<JSObjAddr>,
    // %EvalError.prototype%
    pub(crate) eval_error_prototype: Option<JSObjAddr>,
    // %FinalizationRegistry.prototype%
    pub(crate) finalization_registry_prototype: Option<JSObjAddr>,
    // %Float32Array.prototype%
    pub(crate) float32_array_prototype: Option<JSObjAddr>,
    // %Float64Array.prototype%
    pub(crate) float64_array_prototype: Option<JSObjAddr>,
    // %Function.prototype%
    pub(crate) function_prototype: Option<JSObjAddr>,
    // %GeneratorFunction.prototype.prototype.next%
    pub(crate) generator_function_prototype_prototype_next: Option<JSObjAddr>,
    // %GeneratorFunction.prototype.prototype%
    pub(crate) generator_function_prototype_prototype: Option<JSObjAddr>,
    // %GeneratorFunction.prototype%
    pub(crate) generator_function_prototype: Option<JSObjAddr>,
    // %Int16Array.prototype%
    pub(crate) int16_array_prototype: Option<JSObjAddr>,
    // %Int32Array.prototype%
    pub(crate) int32_array_prototype: Option<JSObjAddr>,
    // %Int8Array.prototype%
    pub(crate) int8_array_prototype: Option<JSObjAddr>,
    // %Iterator.prototype%
    pub(crate) iterator_prototype: Option<JSObjAddr>,
    // %JSON.parse%
    pub(crate) json_parse: Option<JSObjAddr>,
    // %JSON.stringify%
    pub(crate) json_stringify: Option<JSObjAddr>,
    // %Map.prototype%
    pub(crate) map_prototype: Option<JSObjAddr>,
    // %Number.prototype%
    pub(crate) number_prototype: Option<JSObjAddr>,
    // %Object.prototype.toString%
    pub(crate) object_prototype_to_string: Option<JSObjAddr>,
    // %Object.prototype.valueOf%
    pub(crate) object_prototype_value_of: Option<JSObjAddr>,
    // %Object.prototype%
    pub(crate) object_prototype: Option<JSObjAddr>,
    // %Promise.prototype.then%
    pub(crate) promise_prototype_then: Option<JSObjAddr>,
    // %Promise.prototype%
    pub(crate) promise_prototype: Option<JSObjAddr>,
    // %Promise.resolve%
    pub(crate) promise_resolve: Option<JSObjAddr>,
    // %RangeError.prototype%
    pub(crate) range_error_prototype: Option<JSObjAddr>,
    // %ReferenceError.prototype%
    pub(crate) reference_error_prototype: Option<JSObjAddr>,
    // %RegExp.prototype%
    pub(crate) reg_exp_prototype: Option<JSObjAddr>,
    // %Set.prototype%
    pub(crate) set_prototype: Option<JSObjAddr>,
    // %String.prototype%
    pub(crate) string_prototype: Option<JSObjAddr>,
    // %Symbol.prototype%
    pub(crate) symbol_prototype: Option<JSObjAddr>,
    // %SyntaxError.prototype%
    pub(crate) syntax_error_prototype: Option<JSObjAddr>,
    // %TypedArray.prototype%
    pub(crate) typed_array_prototype: Option<JSObjAddr>,
    // %TypeError.prototype%
    pub(crate) type_error_prototype: Option<JSObjAddr>,
    // %Uint16Array.prototype%
    pub(crate) uint16_array_prototype: Option<JSObjAddr>,
    // %Uint32Array.prototype%
    pub(crate) uint32_array_prototype: Option<JSObjAddr>,
    // %Uint8Array.prototype%
    pub(crate) uint8_array_prototype: Option<JSObjAddr>,
    // %Uint8ClampedArray.prototype%
    pub(crate) uint8_clamped_array_prototype: Option<JSObjAddr>,
    // %URIError.prototype%
    pub(crate) uri_error_prototype: Option<JSObjAddr>,
    // %WeakMap.prototype%
    pub(crate) weak_map_prototype: Option<JSObjAddr>,
    // %WeakRef.prototype%
    pub(crate) weak_ref_prototype: Option<JSObjAddr>,
    // %WeakSet.prototype%
    pub(crate) weak_set_prototype: Option<JSObjAddr>,
}
