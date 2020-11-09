(function() {var implementors = {};
implementors["byteorder"] = [{"text":"impl Ord for BigEndian","synthetic":false,"types":[]},{"text":"impl Ord for LittleEndian","synthetic":false,"types":[]}];
implementors["console"] = [{"text":"impl Ord for Attribute","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?Sized + Pointable, '_&gt; Ord for Shared&lt;'_, T&gt;","synthetic":false,"types":[]}];
implementors["deflate"] = [{"text":"impl Ord for Compression","synthetic":false,"types":[]},{"text":"impl Ord for MatchingType","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L:&nbsp;Ord, R:&nbsp;Ord&gt; Ord for Either&lt;L, R&gt;","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T:&nbsp;Ord, N&gt; Ord for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["minifb"] = [{"text":"impl Ord for Key","synthetic":false,"types":[]}];
implementors["ndarray"] = [{"text":"impl Ord for Axis","synthetic":false,"types":[]}];
implementors["noisy_float"] = [{"text":"impl&lt;F:&nbsp;Float, C:&nbsp;FloatChecker&lt;F&gt;&gt; Ord for NoisyFloat&lt;F, C&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer&gt; Ord for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["pest"] = [{"text":"impl&lt;'i&gt; Ord for Position&lt;'i&gt;","synthetic":false,"types":[]}];
implementors["pest_meta"] = [{"text":"impl Ord for Rule","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl Ord for Transformations","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Ord for Ident","synthetic":false,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl Ord for Span","synthetic":false,"types":[]},{"text":"impl Ord for Position","synthetic":false,"types":[]},{"text":"impl Ord for Literal","synthetic":false,"types":[]},{"text":"impl Ord for ClassUnicodeRange","synthetic":false,"types":[]},{"text":"impl Ord for ClassBytesRange","synthetic":false,"types":[]},{"text":"impl Ord for Utf8Sequence","synthetic":false,"types":[]},{"text":"impl Ord for Utf8Range","synthetic":false,"types":[]}];
implementors["rgb"] = [{"text":"impl&lt;ComponentType:&nbsp;Ord&gt; Ord for BGR&lt;ComponentType&gt;","synthetic":false,"types":[]},{"text":"impl&lt;ComponentType:&nbsp;Ord, AlphaComponentType:&nbsp;Ord&gt; Ord for BGRA&lt;ComponentType, AlphaComponentType&gt;","synthetic":false,"types":[]},{"text":"impl&lt;ComponentType:&nbsp;Ord&gt; Ord for Gray&lt;ComponentType&gt;","synthetic":false,"types":[]},{"text":"impl&lt;ComponentType:&nbsp;Ord, AlphaComponentType:&nbsp;Ord&gt; Ord for GrayAlpha&lt;ComponentType, AlphaComponentType&gt;","synthetic":false,"types":[]},{"text":"impl&lt;ComponentType:&nbsp;Ord&gt; Ord for RGB&lt;ComponentType&gt;","synthetic":false,"types":[]},{"text":"impl&lt;ComponentType:&nbsp;Ord, AlphaComponentType:&nbsp;Ord&gt; Ord for RGBA&lt;ComponentType, AlphaComponentType&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl Ord for Lifetime","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl Ord for B0","synthetic":false,"types":[]},{"text":"impl Ord for B1","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Ord + Unsigned + NonZero&gt; Ord for PInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Ord + Unsigned + NonZero&gt; Ord for NInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl Ord for Z0","synthetic":false,"types":[]},{"text":"impl Ord for UTerm","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Ord, B:&nbsp;Ord&gt; Ord for UInt&lt;U, B&gt;","synthetic":false,"types":[]},{"text":"impl Ord for ATerm","synthetic":false,"types":[]},{"text":"impl&lt;V:&nbsp;Ord, A:&nbsp;Ord&gt; Ord for TArr&lt;V, A&gt;","synthetic":false,"types":[]},{"text":"impl Ord for Greater","synthetic":false,"types":[]},{"text":"impl Ord for Less","synthetic":false,"types":[]},{"text":"impl Ord for Equal","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()