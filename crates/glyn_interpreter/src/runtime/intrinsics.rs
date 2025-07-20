use crate::value::object::ObjectAddr;

/// 6.1.7.4 Well-Known Intrinsic Objects
/// https://262.ecma-international.org/16.0/#sec-well-known-intrinsic-objects
#[derive(Debug, Default)]
pub(crate) struct Intrinsics {
    // %AggregateError%
    pub(crate) aggregate_error: Option<ObjectAddr>,
    // %Array%
    pub(crate) array: Option<ObjectAddr>,
    // %ArrayBuffer%
    pub(crate) array_buffer: Option<ObjectAddr>,
    // %ArrayIteratorPrototype%
    pub(crate) array_iterator_prototype: Option<ObjectAddr>,
    // %AsyncFromSyncIteratorPrototype%
    pub(crate) async_from_sync_iterator_prototype: Option<ObjectAddr>,
    // %AsyncFunction%
    pub(crate) async_function: Option<ObjectAddr>,
    // %AsyncGeneratorFunction%
    pub(crate) async_generator_function: Option<ObjectAddr>,
    // %AsyncGeneratorPrototype%
    pub(crate) async_generator_prototype: Option<ObjectAddr>,
    // %AsyncIteratorPrototype%
    pub(crate) async_iterator_prototype: Option<ObjectAddr>,
    // %Atomics%
    pub(crate) atomics: Option<ObjectAddr>,
    // %BigInt%
    pub(crate) big_int: Option<ObjectAddr>,
    // %BigInt64Array%
    pub(crate) big_int64_array: Option<ObjectAddr>,
    // %BigUint64Array%
    pub(crate) big_uint64_array: Option<ObjectAddr>,
    // %Boolean%
    pub(crate) boolean: Option<ObjectAddr>,
    // %DataView%
    pub(crate) data_view: Option<ObjectAddr>,
    // %Date%
    pub(crate) date: Option<ObjectAddr>,
    // %decodeURI%
    pub(crate) decode_uri: Option<ObjectAddr>,
    // %decodeURIComponent%
    pub(crate) decode_uri_component: Option<ObjectAddr>,
    // %encodeURI%
    pub(crate) encode_uri: Option<ObjectAddr>,
    // %encodeURIComponent%
    pub(crate) encode_uri_component: Option<ObjectAddr>,
    // %Error%
    pub(crate) error: Option<ObjectAddr>,
    // %eval%
    pub(crate) eval: Option<ObjectAddr>,
    // %EvalError%
    pub(crate) eval_error: Option<ObjectAddr>,
    // %FinalizationRegistry%
    pub(crate) finalization_registry: Option<ObjectAddr>,
    // %Float16Array%
    pub(crate) float16_array: Option<ObjectAddr>,
    // %Float32Array%
    pub(crate) float32_array: Option<ObjectAddr>,
    // %Float64Array%
    pub(crate) float64_array: Option<ObjectAddr>,
    // %ForInIteratorPrototype%
    pub(crate) for_in_iterator_prototype: Option<ObjectAddr>,
    // %Function%
    pub(crate) function: Option<ObjectAddr>,
    // %GeneratorFunction%
    pub(crate) generator_function: Option<ObjectAddr>,
    // %GeneratorPrototype%
    pub(crate) generator_prototype: Option<ObjectAddr>,
    // %Int8Array%
    pub(crate) int8_array: Option<ObjectAddr>,
    // %Int16Array%
    pub(crate) int16_array: Option<ObjectAddr>,
    // %Int32Array%
    pub(crate) int32_array: Option<ObjectAddr>,
    // %isFinite%
    pub(crate) is_finite: Option<ObjectAddr>,
    // %isNaN%
    pub(crate) is_nan: Option<ObjectAddr>,
    // %Iterator%
    pub(crate) iterator: Option<ObjectAddr>,
    // %IteratorHelperPrototype%
    pub(crate) iterator_helper_prototype: Option<ObjectAddr>,
    // %JSON%
    pub(crate) json: Option<ObjectAddr>,
    // %Map%
    pub(crate) map: Option<ObjectAddr>,
    // %MapIteratorPrototype%
    pub(crate) map_iterator_prototype: Option<ObjectAddr>,
    // %Math%
    pub(crate) math: Option<ObjectAddr>,
    // %Number%
    pub(crate) number: Option<ObjectAddr>,
    // %Object%
    pub(crate) object: Option<ObjectAddr>,
    // %parseFloat%
    pub(crate) parse_float: Option<ObjectAddr>,
    // %parseInt%
    pub(crate) parse_int: Option<ObjectAddr>,
    // %Promise%
    pub(crate) promise: Option<ObjectAddr>,
    // %Proxy%
    pub(crate) proxy: Option<ObjectAddr>,
    // %RangeError%
    pub(crate) range_error: Option<ObjectAddr>,
    // %ReferenceError%
    pub(crate) reference_error: Option<ObjectAddr>,
    // %Reflect%
    pub(crate) reflect: Option<ObjectAddr>,
    // %RegExp%
    pub(crate) reg_exp: Option<ObjectAddr>,
    // %RegExpStringIteratorPrototype%
    pub(crate) reg_exp_string_iterator_prototype: Option<ObjectAddr>,
    // %Set%
    pub(crate) set: Option<ObjectAddr>,
    // %SetIteratorPrototype%
    pub(crate) set_iterator_prototype: Option<ObjectAddr>,
    // %SharedArrayBuffer%
    pub(crate) shared_array_buffer: Option<ObjectAddr>,
    // %String%
    pub(crate) string: Option<ObjectAddr>,
    // %StringIteratorPrototype%
    pub(crate) string_iterator_prototype: Option<ObjectAddr>,
    // %Symbol%
    pub(crate) symbol: Option<ObjectAddr>,
    // %SyntaxError%
    pub(crate) syntax_error: Option<ObjectAddr>,
    // %ThrowTypeError%
    pub(crate) throw_type_error: Option<ObjectAddr>,
    // %TypedArray%
    pub(crate) typed_array: Option<ObjectAddr>,
    // %TypeError%
    pub(crate) type_error: Option<ObjectAddr>,
    // %Uint8Array%
    pub(crate) uint8_array: Option<ObjectAddr>,
    // %Uint8ClampedArray%
    pub(crate) uint8_clamped_array: Option<ObjectAddr>,
    // %Uint16Array%
    pub(crate) uint16_array: Option<ObjectAddr>,
    // %Uint32Array%
    pub(crate) uint32_array: Option<ObjectAddr>,
    // %URIError%
    pub(crate) uri_error: Option<ObjectAddr>,
    // %WeakMap%
    pub(crate) weak_map: Option<ObjectAddr>,
    // %WeakRef%
    pub(crate) weak_ref: Option<ObjectAddr>,
    // %WeakSet%
    pub(crate) weak_set: Option<ObjectAddr>,
    // %WrapForValidIteratorPrototype%
    pub(crate) wrap_for_valid_iterator_prototype: Option<ObjectAddr>,
    // %AggregateError.prototype%
    pub(crate) aggregate_error_prototype: Option<ObjectAddr>,
    // %Array.prototype.values%
    pub(crate) array_prototype_values: Option<ObjectAddr>,
    // %Array.prototype%
    pub(crate) array_prototype: Option<ObjectAddr>,
    // %ArrayBuffer.prototype%
    pub(crate) array_buffer_prototype: Option<ObjectAddr>,
    // %AsyncFunction.prototype%
    pub(crate) async_function_prototype: Option<ObjectAddr>,
    // %AsyncGeneratorFunction.prototype.prototype%
    pub(crate) async_generator_function_prototype_prototype: Option<ObjectAddr>,
    // %AsyncGeneratorFunction.prototype%
    pub(crate) async_generator_function_prototype: Option<ObjectAddr>,
    // %BigInt.prototype%
    pub(crate) big_int_prototype: Option<ObjectAddr>,
    // %BigInt64Array.prototype%
    pub(crate) big_int64_array_prototype: Option<ObjectAddr>,
    // %BigUint64Array.prototype%
    pub(crate) big_uint64_array_prototype: Option<ObjectAddr>,
    // %Boolean.prototype%
    pub(crate) boolean_prototype: Option<ObjectAddr>,
    // %DataView.prototype%
    pub(crate) data_view_prototype: Option<ObjectAddr>,
    // %Date.prototype%
    pub(crate) date_prototype: Option<ObjectAddr>,
    // %Error.prototype%
    pub(crate) error_prototype: Option<ObjectAddr>,
    // %Error.prototype.toString%
    pub(crate) error_prototype_to_string: Option<ObjectAddr>,
    // %EvalError.prototype%
    pub(crate) eval_error_prototype: Option<ObjectAddr>,
    // %FinalizationRegistry.prototype%
    pub(crate) finalization_registry_prototype: Option<ObjectAddr>,
    // %Float32Array.prototype%
    pub(crate) float32_array_prototype: Option<ObjectAddr>,
    // %Float64Array.prototype%
    pub(crate) float64_array_prototype: Option<ObjectAddr>,
    // %Function.prototype%
    pub(crate) function_prototype: Option<ObjectAddr>,
    // %GeneratorFunction.prototype.prototype.next%
    pub(crate) generator_function_prototype_prototype_next: Option<ObjectAddr>,
    // %GeneratorFunction.prototype.prototype%
    pub(crate) generator_function_prototype_prototype: Option<ObjectAddr>,
    // %GeneratorFunction.prototype%
    pub(crate) generator_function_prototype: Option<ObjectAddr>,
    // %Int16Array.prototype%
    pub(crate) int16_array_prototype: Option<ObjectAddr>,
    // %Int32Array.prototype%
    pub(crate) int32_array_prototype: Option<ObjectAddr>,
    // %Int8Array.prototype%
    pub(crate) int8_array_prototype: Option<ObjectAddr>,
    // %Iterator.prototype%
    pub(crate) iterator_prototype: Option<ObjectAddr>,
    // %JSON.parse%
    pub(crate) json_parse: Option<ObjectAddr>,
    // %JSON.stringify%
    pub(crate) json_stringify: Option<ObjectAddr>,
    // %Map.prototype%
    pub(crate) map_prototype: Option<ObjectAddr>,
    // %Number.prototype%
    pub(crate) number_prototype: Option<ObjectAddr>,
    // %Object.prototype.toString%
    pub(crate) object_prototype_to_string: Option<ObjectAddr>,
    // %Object.prototype.valueOf%
    pub(crate) object_prototype_value_of: Option<ObjectAddr>,
    // %Object.prototype%
    pub(crate) object_prototype: Option<ObjectAddr>,
    // %Promise.prototype.then%
    pub(crate) promise_prototype_then: Option<ObjectAddr>,
    // %Promise.prototype%
    pub(crate) promise_prototype: Option<ObjectAddr>,
    // %Promise.resolve%
    pub(crate) promise_resolve: Option<ObjectAddr>,
    // %RangeError.prototype%
    pub(crate) range_error_prototype: Option<ObjectAddr>,
    // %ReferenceError.prototype%
    pub(crate) reference_error_prototype: Option<ObjectAddr>,
    // %RegExp.prototype%
    pub(crate) reg_exp_prototype: Option<ObjectAddr>,
    // %Set.prototype%
    pub(crate) set_prototype: Option<ObjectAddr>,
    // %String.prototype%
    pub(crate) string_prototype: Option<ObjectAddr>,
    // %Symbol.prototype%
    pub(crate) symbol_prototype: Option<ObjectAddr>,
    // %SyntaxError.prototype%
    pub(crate) syntax_error_prototype: Option<ObjectAddr>,
    // %TypedArray.prototype%
    pub(crate) typed_array_prototype: Option<ObjectAddr>,
    // %TypeError.prototype%
    pub(crate) type_error_prototype: Option<ObjectAddr>,
    // %Uint16Array.prototype%
    pub(crate) uint16_array_prototype: Option<ObjectAddr>,
    // %Uint32Array.prototype%
    pub(crate) uint32_array_prototype: Option<ObjectAddr>,
    // %Uint8Array.prototype%
    pub(crate) uint8_array_prototype: Option<ObjectAddr>,
    // %Uint8ClampedArray.prototype%
    pub(crate) uint8_clamped_array_prototype: Option<ObjectAddr>,
    // %URIError.prototype%
    pub(crate) uri_error_prototype: Option<ObjectAddr>,
    // %WeakMap.prototype%
    pub(crate) weak_map_prototype: Option<ObjectAddr>,
    // %WeakRef.prototype%
    pub(crate) weak_ref_prototype: Option<ObjectAddr>,
    // %WeakSet.prototype%
    pub(crate) weak_set_prototype: Option<ObjectAddr>,
}
