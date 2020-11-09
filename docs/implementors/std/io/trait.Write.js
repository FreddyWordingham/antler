(function() {var implementors = {};
implementors["console"] = [{"text":"impl Write for Term","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for &amp;'a Term","synthetic":false,"types":[]}];
implementors["deflate"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Write for DeflateEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for ZlibEncoder&lt;W&gt;","synthetic":false,"types":[]}];
implementors["indicatif"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Write for ProgressBarWrap&lt;W&gt;","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl&lt;'a, W:&nbsp;Write&gt; Write for StreamWriter&lt;'a, W&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()