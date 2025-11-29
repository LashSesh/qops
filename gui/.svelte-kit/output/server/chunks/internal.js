import{c as u,a as g,v as f,m as h}from"./ssr.js";import{a as y}from"./ssr2.js";import"./environment.js";let x={};function S(n){}function C(n){x=n}let k=null;function F(n){k=n}function P(n){}const w=u((n,t,e,v)=>{let{stores:o}=t,{page:r}=t,{constructors:s}=t,{components:a=[]}=t,{form:d}=t,{data_0:c=null}=t,{data_1:m=null}=t;g("__svelte__",o),y(o.page.notify),t.stores===void 0&&e.stores&&o!==void 0&&e.stores(o),t.page===void 0&&e.page&&r!==void 0&&e.page(r),t.constructors===void 0&&e.constructors&&s!==void 0&&e.constructors(s),t.components===void 0&&e.components&&a!==void 0&&e.components(a),t.form===void 0&&e.form&&d!==void 0&&e.form(d),t.data_0===void 0&&e.data_0&&c!==void 0&&e.data_0(c),t.data_1===void 0&&e.data_1&&m!==void 0&&e.data_1(m);let l,p,_=n.head;do l=!0,n.head=_,o.page.set(r),p=`  ${s[1]?`${f(s[0]||h,"svelte:component").$$render(n,{data:c,params:r.params,this:a[0]},{this:i=>{a[0]=i,l=!1}},{default:()=>`${f(s[1]||h,"svelte:component").$$render(n,{data:m,form:d,params:r.params,this:a[1]},{this:i=>{a[1]=i,l=!1}},{})}`})}`:`${f(s[0]||h,"svelte:component").$$render(n,{data:c,form:d,params:r.params,this:a[0]},{this:i=>{a[0]=i,l=!1}},{})}`} `;while(!l);return p}),U={app_template_contains_nonce:!1,async:!1,csp:{mode:"auto",directives:{"upgrade-insecure-requests":!1,"block-all-mixed-content":!1},reportOnly:{"upgrade-insecure-requests":!1,"block-all-mixed-content":!1}},csrf_check_origin:!0,csrf_trusted_origins:[],embedded:!1,env_public_prefix:"PUBLIC_",env_private_prefix:"",hash_routing:!1,hooks:null,preload_strategy:"modulepreload",root:w,service_worker:!1,service_worker_options:void 0,templates:{app:({head:n,body:t,assets:e,nonce:v,env:o})=>`<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="utf-8" />
		<link rel="icon" href="`+e+`/favicon.png" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />
		<meta name="theme-color" content="#0F172A" />
		<title>QOPS - Quantum Operator Processing System</title>
		`+n+`
	</head>
	<body data-sveltekit-preload-data="hover" class="bg-surface-900 text-slate-50">
		<div style="display: contents">`+t+`</div>
	</body>
</html>
`,error:({status:n,message:t})=>`<!doctype html>
<html lang="en">
	<head>
		<meta charset="utf-8" />
		<title>`+t+`</title>

		<style>
			body {
				--bg: white;
				--fg: #222;
				--divider: #ccc;
				background: var(--bg);
				color: var(--fg);
				font-family:
					system-ui,
					-apple-system,
					BlinkMacSystemFont,
					'Segoe UI',
					Roboto,
					Oxygen,
					Ubuntu,
					Cantarell,
					'Open Sans',
					'Helvetica Neue',
					sans-serif;
				display: flex;
				align-items: center;
				justify-content: center;
				height: 100vh;
				margin: 0;
			}

			.error {
				display: flex;
				align-items: center;
				max-width: 32rem;
				margin: 0 1rem;
			}

			.status {
				font-weight: 200;
				font-size: 3rem;
				line-height: 1;
				position: relative;
				top: -0.05rem;
			}

			.message {
				border-left: 1px solid var(--divider);
				padding: 0 0 0 1rem;
				margin: 0 0 0 1rem;
				min-height: 2.5rem;
				display: flex;
				align-items: center;
			}

			.message h1 {
				font-weight: 400;
				font-size: 1em;
				margin: 0;
			}

			@media (prefers-color-scheme: dark) {
				body {
					--bg: #222;
					--fg: #ddd;
					--divider: #666;
				}
			}
		</style>
	</head>
	<body>
		<div class="error">
			<span class="status">`+n+`</span>
			<div class="message">
				<h1>`+t+`</h1>
			</div>
		</div>
	</body>
</html>
`},version_hash:"19cl76d"};async function q(){return{handle:void 0,handleFetch:void 0,handleError:void 0,handleValidationError:void 0,init:void 0,reroute:void 0,transport:void 0}}export{C as a,F as b,P as c,q as g,U as o,x as p,k as r,S as s};
