# Händigt o̲ behändigt 😉

## Html_escape
_escape html_  
html_escape::encode_safe_to_string( a \<String>, b \<&mut String>)  
[manual](https://docs.rs/html-escape/latest/html_escape/fn.decode_html_entities_to_string.html)

_"raw" från escaped_  
html_escape::decode_html_entities_to_string( a \<String>, b \<&mut String>)  
[manual](https://docs.rs/html-escape/latest/html_escape/fn.decode_html_entities_to_string.html)
___
## Serde_json
_ändra värde i jsonstruktur_  
\<*serde_json::Value>.pointer_mut( json_pointer \<String>) = new_value\<&mut Value>  
[manual](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html#method.pointer_mut)
