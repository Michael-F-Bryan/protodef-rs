(function() {var implementors = {};
implementors["hashbrown"] = [{"text":"impl&lt;T&gt; Freeze for Bucket&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for RawTable&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for RawIter&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for RawIntoIter&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Freeze for RawDrain&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Freeze for RawIterHash&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for DefaultHashBuilder","synthetic":true,"types":[]},{"text":"impl&lt;K, V, S&gt; Freeze for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Iter&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for IterMut&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;K, V&gt; Freeze for IntoIter&lt;K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Keys&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Values&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Drain&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, F&gt; Freeze for DrainFilter&lt;'a, K, V, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for ValuesMut&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Freeze for RawEntryBuilderMut&lt;'a, K, V, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Freeze for RawEntryMut&lt;'a, K, V, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Freeze for RawOccupiedEntryMut&lt;'a, K, V, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Freeze for RawVacantEntryMut&lt;'a, K, V, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Freeze for RawEntryBuilder&lt;'a, K, V, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Freeze for Entry&lt;'a, K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Freeze for OccupiedEntry&lt;'a, K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Freeze for VacantEntry&lt;'a, K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T, S&gt; Freeze for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K&gt; Freeze for Iter&lt;'a, K&gt;","synthetic":true,"types":[]},{"text":"impl&lt;K&gt; Freeze for IntoIter&lt;K&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K&gt; Freeze for Drain&lt;'a, K&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, F&gt; Freeze for DrainFilter&lt;'a, K, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, S&gt; Freeze for Intersection&lt;'a, T, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, S&gt; Freeze for Difference&lt;'a, T, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, S&gt; Freeze for SymmetricDifference&lt;'a, T, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, S&gt; Freeze for Union&lt;'a, T, S&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for TryReserveError","synthetic":true,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;'a, K, V&gt; Freeze for OccupiedEntry&lt;'a, K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Entry&lt;'a, K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for VacantEntry&lt;'a, K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;K, V, S&gt; Freeze for IndexMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Keys&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Values&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for ValuesMut&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Iter&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for IterMut&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;K, V&gt; Freeze for IntoIter&lt;K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, K, V&gt; Freeze for Drain&lt;'a, K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T, S&gt; Freeze for IndexSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for IntoIter&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Freeze for Iter&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Freeze for Drain&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, S&gt; Freeze for Difference&lt;'a, T, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, S&gt; Freeze for Intersection&lt;'a, T, S&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, S1, S2&gt; Freeze for SymmetricDifference&lt;'a, T, S1, S2&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, T, S&gt; Freeze for Union&lt;'a, T, S&gt;","synthetic":true,"types":[]}];
implementors["itoa"] = [{"text":"impl Freeze for Buffer","synthetic":true,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Freeze for IntoIter","synthetic":true,"types":[]},{"text":"impl Freeze for TokenStream","synthetic":true,"types":[]},{"text":"impl Freeze for LexError","synthetic":true,"types":[]},{"text":"impl Freeze for Span","synthetic":true,"types":[]},{"text":"impl Freeze for TokenTree","synthetic":true,"types":[]},{"text":"impl Freeze for Group","synthetic":true,"types":[]},{"text":"impl Freeze for Delimiter","synthetic":true,"types":[]},{"text":"impl Freeze for Punct","synthetic":true,"types":[]},{"text":"impl Freeze for Spacing","synthetic":true,"types":[]},{"text":"impl Freeze for Ident","synthetic":true,"types":[]},{"text":"impl Freeze for Literal","synthetic":true,"types":[]}];
implementors["protodef_codegen"] = [{"text":"impl Freeze for Diagnostics","synthetic":true,"types":[]},{"text":"impl Freeze for Diagnostic","synthetic":true,"types":[]},{"text":"impl Freeze for CompilationUnit","synthetic":true,"types":[]},{"text":"impl Freeze for Type","synthetic":true,"types":[]},{"text":"impl Freeze for Struct","synthetic":true,"types":[]},{"text":"impl Freeze for Field","synthetic":true,"types":[]},{"text":"impl Freeze for Enum","synthetic":true,"types":[]},{"text":"impl Freeze for Variant","synthetic":true,"types":[]},{"text":"impl Freeze for BitFields","synthetic":true,"types":[]},{"text":"impl Freeze for LengthPrefixedString","synthetic":true,"types":[]},{"text":"impl Freeze for TypeId","synthetic":true,"types":[]},{"text":"impl Freeze for Protocol","synthetic":true,"types":[]},{"text":"impl Freeze for Type","synthetic":true,"types":[]},{"text":"impl Freeze for Container","synthetic":true,"types":[]},{"text":"impl Freeze for Field","synthetic":true,"types":[]},{"text":"impl Freeze for Switch","synthetic":true,"types":[]},{"text":"impl Freeze for BitFields","synthetic":true,"types":[]},{"text":"impl Freeze for BitField","synthetic":true,"types":[]},{"text":"impl Freeze for Mapper","synthetic":true,"types":[]},{"text":"impl Freeze for ParseError","synthetic":true,"types":[]},{"text":"impl Freeze for ErrorKind","synthetic":true,"types":[]}];
implementors["protodef_core"] = [{"text":"impl Freeze for DeserializeError","synthetic":true,"types":[]}];
implementors["ryu"] = [{"text":"impl Freeze for Buffer","synthetic":true,"types":[]}];
implementors["serde"] = [{"text":"impl Freeze for Error","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for UnitDeserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for BoolDeserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for I8Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for I16Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for I32Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for I64Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for IsizeDeserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for U8Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for U16Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for U64Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for UsizeDeserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for F32Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for F64Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for CharDeserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for I128Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for U128Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for U32Deserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, E&gt; Freeze for StrDeserializer&lt;'a, E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'de, E&gt; Freeze for BorrowedStrDeserializer&lt;'de, E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; Freeze for StringDeserializer&lt;E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, E&gt; Freeze for CowStrDeserializer&lt;'a, E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'de, E&gt; Freeze for BorrowedBytesDeserializer&lt;'de, E&gt;","synthetic":true,"types":[]},{"text":"impl&lt;I, E&gt; Freeze for SeqDeserializer&lt;I, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Freeze for SeqAccessDeserializer&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'de, I, E&gt; Freeze for MapDeserializer&lt;'de, I, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;I as Iterator&gt;::Item as Pair&gt;::Second: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; Freeze for MapAccessDeserializer&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Freeze for IgnoredAny","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for Unexpected&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;Ok, Error&gt; Freeze for Impossible&lt;Ok, Error&gt;","synthetic":true,"types":[]}];
implementors["serde_json"] = [{"text":"impl&lt;'a&gt; Freeze for SliceRead&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for StrRead&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Freeze for IoRead&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Freeze for Deserializer&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'de, R, T&gt; Freeze for StreamDeserializer&lt;'de, R, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Freeze for Error","synthetic":true,"types":[]},{"text":"impl Freeze for Category","synthetic":true,"types":[]},{"text":"impl&lt;K, V&gt; Freeze for Map&lt;K, V&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for Entry&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for VacantEntry&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for OccupiedEntry&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for Iter&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for IterMut&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for IntoIter","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for Keys&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for Values&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for ValuesMut&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;W, F&gt; Freeze for Serializer&lt;W, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Freeze for CharEscape","synthetic":true,"types":[]},{"text":"impl Freeze for CompactFormatter","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for PrettyFormatter&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for Serializer","synthetic":true,"types":[]},{"text":"impl Freeze for Number","synthetic":true,"types":[]},{"text":"impl Freeze for Value","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()