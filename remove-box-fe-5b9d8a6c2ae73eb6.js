let S=0,W=`string`,U=1,X=`Object`,Q=`utf-8`,O=null,P=`undefined`,Z=4,V=`function`,M=128,L=Array,R=Error,Y=FinalizationRegistry,$=Object,_=Reflect,T=Uint8Array,N=undefined;var F=(async(a,b)=>{if(typeof Response===V&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===V){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var s=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==O){return `${a}`};if(b==W){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==O){return `Symbol`}else{return `Symbol(${b})`}};if(b==V){const b=a.name;if(typeof b==W&&b.length>S){return `Function(${b})`}else{return `Function`}};if(L.isArray(a)){const b=a.length;let c=`[`;if(b>S){c+=s(a[S])};for(let d=U;d<b;d++){c+=`, `+ s(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>U){d=c[U]}else{return toString.call(a)};if(d==X){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return X}};if(a instanceof R){return `${a.name}: ${a.message}\n${a.stack}`};return d});var v=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__h2a6249bbee5620cf(b,c)});var H=((a,b)=>{});var A=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h476dee5682830e02(b,c,k(d))});var E=((a,b)=>{a=a>>>S;const c=D();const d=c.subarray(a/Z,a/Z+ b);const e=[];for(let a=S;a<d.length;a++){e.push(f(d[a]))};return e});var k=(a=>{if(d===b.length)b.push(b.length+ U);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});function B(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var K=(async(b)=>{if(a!==N)return a;if(typeof b===P){b=new URL(`remove-box-fe-5b9d8a6c2ae73eb6_bg.wasm`,import.meta.url)};const c=G();if(typeof b===W||typeof Request===V&&b instanceof Request||typeof URL===V&&b instanceof URL){b=fetch(b)};H(c);const {instance:d,module:e}=await F(await b,c);return I(d,e)});var J=(b=>{if(a!==N)return a;const c=G();H(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return I(d,b)});var r=(()=>{if(q===O||q.byteLength===S){q=new Int32Array(a.memory.buffer)};return q});var p=(a=>a===N||a===O);var c=(a=>b[a]);var I=((b,c)=>{a=b.exports;K.__wbindgen_wasm_module=c;q=O;C=O;h=O;a.__wbindgen_start();return a});var G=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==U){b.a=S;return !0};const c=!1;return c});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===N;return b});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===W?e:N;var g=p(f)?S:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/Z+ U]=h;r()[b/Z+ S]=g});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===W;return b});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>S});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/Z+ U]=p(d)?S:d;r()[a/Z+ S]=!p(d)});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/Z+ U]=p(d)?S:d;r()[a/Z+ S]=!p(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>S});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/Z+ U]=p(d)?S:d;r()[a/Z+ S]=!p(d)});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>S});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new R();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,U)}});b.wbg.__wbg_clearTimeout_541ac0980ffcef74=(a=>{const b=clearTimeout(f(a));return k(b)});b.wbg.__wbg_setTimeout_7d81d052875b0f4f=function(){return B(((a,b)=>{const d=setTimeout(c(a),b);return k(d)}),arguments)};b.wbg.__wbg_queueMicrotask_f61ee94ee663068b=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_f82fc5d1e8f816ae=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===V;return b});b.wbg.__wbg_crypto_d05b68a3572bb8ca=(a=>{const b=c(a).crypto;return k(b)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==O;return d});b.wbg.__wbg_process_b02b3570280d0366=(a=>{const b=c(a).process;return k(b)});b.wbg.__wbg_versions_c1cb42213cedf0f5=(a=>{const b=c(a).versions;return k(b)});b.wbg.__wbg_node_43b1089f407e4ec2=(a=>{const b=c(a).node;return k(b)});b.wbg.__wbg_msCrypto_10fc94afee92bd76=(a=>{const b=c(a).msCrypto;return k(b)});b.wbg.__wbg_require_9a7e0f667ead4995=function(){return B((()=>{const a=module.require;return k(a)}),arguments)};b.wbg.__wbg_randomFillSync_b70ccbdf4926a99d=function(){return B(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_7e42b4fb8779dc6d=function(){return B(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=E(b,c).slice();a.__wbindgen_free(b,c*Z,Z);console.error(...d)});b.wbg.__wbg_log_7c3433e130418e14=((b,c)=>{var d=E(b,c).slice();a.__wbindgen_free(b,c*Z,Z);console.log(...d)});b.wbg.__wbg_body_874ccb42daaab363=(a=>{const b=c(a).body;return p(b)?S:k(b)});b.wbg.__wbg_createElement_03cf347ddad1c8c0=function(){return B(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_createElementNS_93f8de4acdef6da8=function(){return B(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===S?N:j(b,d),j(e,f));return k(g)}),arguments)};b.wbg.__wbg_createTextNode_ea32ad2506f7ae78=((a,b,d)=>{const e=c(a).createTextNode(j(b,d));return k(e)});b.wbg.__wbg_instanceof_Element_813f33306edae612=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_230708ae7f4baac5=((b,d)=>{const e=c(d).namespaceURI;var f=p(e)?S:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_setinnerHTML_95222f1a2e797983=((a,b,d)=>{c(a).innerHTML=j(b,d)});b.wbg.__wbg_outerHTML_eb21059e86b1e9f4=((b,d)=>{const e=c(d).outerHTML;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_removeAttribute_0c021c98a4dc7402=function(){return B(((a,b,d)=>{c(a).removeAttribute(j(b,d))}),arguments)};b.wbg.__wbg_setAttribute_f7ffa687ef977957=function(){return B(((a,b,d,e,f)=>{c(a).setAttribute(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_instanceof_Window_cee7a886d55e7df5=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_eb7fd66bde3ee213=(a=>{const b=c(a).document;return p(b)?S:k(b)});b.wbg.__wbg_localStorage_3d538af21ea07fcc=function(){return B((a=>{const b=c(a).localStorage;return p(b)?S:k(b)}),arguments)};b.wbg.__wbg_fetch_33c84c2bf739f490=((a,b)=>{const d=c(a).fetch(c(b));return k(d)});b.wbg.__wbg_instanceof_WorkerGlobalScope_42acbb685bed964e=(a=>{let b;try{b=c(a) instanceof WorkerGlobalScope}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_fetch_10edd7d7da150227=((a,b)=>{const d=c(a).fetch(c(b));return k(d)});b.wbg.__wbg_setchecked_50e21357d62a8ccd=((a,b)=>{c(a).checked=b!==S});b.wbg.__wbg_value_99f5294791d62576=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_setvalue_bba31de32cdbb32c=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_search_6b70a3bf2ceb3f63=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_setsearch_e3e6802fd5fe58c4=((a,b,d)=>{c(a).search=j(b,d)});b.wbg.__wbg_new_79acf9a4da56c772=function(){return B(((a,b)=>{const c=new URL(j(a,b));return k(c)}),arguments)};b.wbg.__wbg_value_ffef403d62e3df58=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_setvalue_cbab536654d8dd52=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_new_a72fe5a17d68e2f8=function(){return B((()=>{const a=new URLSearchParams();return k(a)}),arguments)};b.wbg.__wbg_parentNode_e3a5ee563364a613=(a=>{const b=c(a).parentNode;return p(b)?S:k(b)});b.wbg.__wbg_parentElement_45a9756dc74ff48b=(a=>{const b=c(a).parentElement;return p(b)?S:k(b)});b.wbg.__wbg_childNodes_535387effca84f4e=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_lastChild_d22dbf81f92f163b=(a=>{const b=c(a).lastChild;return p(b)?S:k(b)});b.wbg.__wbg_nextSibling_87d2b32dfbf09fe3=(a=>{const b=c(a).nextSibling;return p(b)?S:k(b)});b.wbg.__wbg_setnodeValue_d1cec51282858afe=((a,b,d)=>{c(a).nodeValue=b===S?N:j(b,d)});b.wbg.__wbg_textContent_528ff517a0418a3e=((b,d)=>{const e=c(d).textContent;var f=p(e)?S:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_cloneNode_ea49a704f0699b2e=function(){return B((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_insertBefore_2be91083083caa9e=function(){return B(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_removeChild_660924798c7e128c=function(){return B(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_4db22fd5d40c5665=function(){return B((()=>{const a=new Headers();return k(a)}),arguments)};b.wbg.__wbg_set_4ad92a627c50c8ef=function(){return B(((a,b,d,e,f)=>{c(a).set(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_getItem_5c179cd36e9529e8=function(){return B(((b,d,e,f)=>{const g=c(d).getItem(j(e,f));var h=p(g)?S:o(g,a.__wbindgen_malloc,a.__wbindgen_realloc);var i=l;r()[b/Z+ U]=i;r()[b/Z+ S]=h}),arguments)};b.wbg.__wbg_setItem_7b55989efb4d45f7=function(){return B(((a,b,d,e,f)=>{c(a).setItem(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_addEventListener_bc4a7ad4cc72c6bf=function(){return B(((a,b,d,e,f)=>{c(a).addEventListener(j(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_url_abf923e9619bea18=((b,d)=>{const e=c(d).url;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_newwithstr_7a7cde4cfdb27ce4=function(){return B(((a,b)=>{const c=new Request(j(a,b));return k(c)}),arguments)};b.wbg.__wbg_newwithstrandinit_11fbc38beb4c26b0=function(){return B(((a,b,d)=>{const e=new Request(j(a,b),c(d));return k(e)}),arguments)};b.wbg.__wbg_instanceof_Response_b5451a06784a2404=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_ok_d44e03dcd4b678b0=(a=>{const b=c(a).ok;return b});b.wbg.__wbg_statusText_9f8aede060c44220=((b,d)=>{const e=c(d).statusText;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbg_text_24a1c9b21feed3de=function(){return B((a=>{const b=c(a).text();return k(b)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_ef56f954a86c7472=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_dfffc3b2ba786fb8=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_target_6795373f170fd786=(a=>{const b=c(a).target;return p(b)?S:k(b)});b.wbg.__wbg_bubbles_31126fc08276cf99=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_ae95595adf5ae83d=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_bd8a0336a042e053=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_get_0ee8ea3c7c984c45=((a,b)=>{const d=c(a)[b>>>S];return k(d)});b.wbg.__wbg_length_161c0d89c6535c1d=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_cfecb3965268594c=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_get_3fddfed2c83f434c=function(){return B(((a,b)=>{const d=_.get(c(a),c(b));return k(d)}),arguments)};b.wbg.__wbg_call_3f093dd26d5569f8=function(){return B(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_632630b5cec17f21=(()=>{const a=new $();return k(a)});b.wbg.__wbg_self_05040bd9523805b9=function(){return B((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_adc720039f2cb14f=function(){return B((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_622105db80c1457d=function(){return B((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_f56b013ed9bcf359=function(){return B((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbg_from_58c79ccfb68060f5=(a=>{const b=L.from(c(a));return k(b)});b.wbg.__wbg_instanceof_Error_5869c4f17aac9eb2=(a=>{let b;try{b=c(a) instanceof R}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_message_2a19bb5b62cf8e22=(a=>{const b=c(a).message;return k(b)});b.wbg.__wbg_name_405bb0aa047a1bf5=(a=>{const b=c(a).name;return k(b)});b.wbg.__wbg_toString_07f01913ec9af122=(a=>{const b=c(a).toString();return k(b)});b.wbg.__wbg_call_67f2111acd2dfdb6=function(){return B(((a,b,d)=>{const e=c(a).call(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_now_ba25f0a487340763=(()=>{const a=Date.now();return a});b.wbg.__wbg_is_bd5dc4ae269cba1c=((a,b)=>{const d=$.is(c(a),c(b));return d});b.wbg.__wbg_toString_6eb7c1f755c00453=(a=>{const b=c(a).toString();return k(b)});b.wbg.__wbg_resolve_5da6faf2c96fd1d5=(a=>{const b=Promise.resolve(c(a));return k(b)});b.wbg.__wbg_then_f9e58f5a50f43eae=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_then_20a5920e447d1cb1=((a,b,d)=>{const e=c(a).then(c(b),c(d));return k(e)});b.wbg.__wbg_buffer_b914fb8b50ebbc3e=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_newwithbyteoffsetandlength_0de9ee56e9f6ee6e=((a,b,d)=>{const e=new T(c(a),b>>>S,d>>>S);return k(e)});b.wbg.__wbg_new_b1f2d6842d615181=(a=>{const b=new T(c(a));return k(b)});b.wbg.__wbg_set_7d988c98e6ced92d=((a,b,d)=>{c(a).set(c(b),d>>>S)});b.wbg.__wbg_newwithlength_0d03cef43b68a530=(a=>{const b=new T(a>>>S);return k(b)});b.wbg.__wbg_subarray_adc418253d76e2f1=((a,b,d)=>{const e=c(a).subarray(b>>>S,d>>>S);return k(e)});b.wbg.__wbg_set_961700853a212a39=function(){return B(((a,b,d)=>{const e=_.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=s(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ U]=g;r()[b/Z+ S]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new R(j(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return k(b)});b.wbg.__wbindgen_closure_wrapper620=((a,b,c)=>{const d=u(a,b,339,v);return k(d)});b.wbg.__wbindgen_closure_wrapper1063=((a,b,c)=>{const d=w(a,b,516,z);return k(d)});b.wbg.__wbindgen_closure_wrapper1170=((a,b,c)=>{const d=u(a,b,556,A);return k(d)});return b});var D=(()=>{if(C===O||C.byteLength===S){C=new Uint32Array(a.memory.buffer)};return C});var z=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_ref__h714f5b4bd2b49886(c,d,y(e))}finally{b[x++]=N}});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:U,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===S){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=S;t.unregister(f)}}};g.original=f;t.register(g,f,f);return g});var e=(a=>{if(a<132)return;b[a]=d;d=a});var u=((b,c,d,e)=>{const f={a:b,b:c,cnt:U,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=S;try{return e(c,f.b,...b)}finally{if(--f.cnt===S){a.__wbindgen_export_2.get(f.dtor)(c,f.b);t.unregister(f)}else{f.a=c}}};g.original=f;t.register(g,f,f);return g});var o=((a,b,c)=>{if(c===N){const c=m.encode(a);const d=b(c.length,U)>>>S;i().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,U)>>>S;const f=i();let g=S;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==S){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,U)>>>S;const b=i().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written;e=c(e,d,g,U)>>>S};l=g;return e});var i=(()=>{if(h===O||h.byteLength===S){h=new T(a.memory.buffer)};return h});var j=((a,b)=>{a=a>>>S;return g.decode(i().subarray(a,a+ b))});var y=(a=>{if(x==U)throw new R(`out of js stack`);b[--x]=a;return x});let a;const b=new L(M).fill(N);b.push(N,O,!0,!1);let d=b.length;const g=typeof TextDecoder!==P?new TextDecoder(Q,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw R(`TextDecoder not available`)}};if(typeof TextDecoder!==P){g.decode()};let h=O;let l=S;const m=typeof TextEncoder!==P?new TextEncoder(Q):{encode:()=>{throw R(`TextEncoder not available`)}};const n=typeof m.encodeInto===V?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=O;const t=typeof Y===P?{register:()=>{},unregister:()=>{}}:new Y(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});let x=M;let C=O;export default K;export{J as initSync}